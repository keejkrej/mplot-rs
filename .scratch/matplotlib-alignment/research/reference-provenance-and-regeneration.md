# Matplotlib reference provenance and regeneration

Research target: adjacent `../matplotlib` at commit
`bde111fb4ee0023852b48d48f14436647400663d` (`v3.11.0-348-gbde111fb4e`;
the built Python package reports `3.12.0.dev348+gbde111fb4`). Sources are the
checked-out source, the local generator, checked-in PNGs/tests, and a clean
local build of that checkout. No external source was used.

## Finding: the current goldens are not provenance-complete

All 14 checked-in PNGs contain exactly two ancillary chunks: `tEXt Software`
with `Matplotlib version3.10.9, https://matplotlib.org/`, and `pHYs`
`3937 x 3937 pixels/metre` (100 DPI). They contain no creation timestamp or
environment record. Their dimensions agree with the explicit fixture sizes:
11 are 640x480, `boxplot_log.png` is 800x600, `gallery_subplots.png` is
1000x400, and `subplot_lines.png` is 1000x800. Thus they are demonstrably not
outputs of the pinned 3.11-development checkout, and Python, OS/architecture,
NumPy, Pillow, FreeType build, font file, rc file/cache, and command used to
create them cannot be recovered from the repository. The README's
`.venv/bin/python` command names no lock file, and no `.venv` is present.

The version string is expected: the pinned PNG writer inserts the running
Matplotlib version as `Software` unless explicitly overridden
([`image.py` lines 1692-1708](../../../../matplotlib/lib/matplotlib/image.py)).
The generator explicitly selects Agg, writes into the golden directory, and
uses 100 DPI ([`scripts/mpl_reference.py` lines 8-14 and 34-37](../../../scripts/mpl_reference.py)).

## Evaluated clean environment (2026-07-13)

This is a verified environment capable of building and importing the exact
checkout. It is an evidence point, not a sufficient future lock: the source
declares lower bounds, so a lock/manifest must preserve these exact versions
if this builder is adopted.

| Item | Verified value |
|---|---|
| Host | macOS 27.0, arm64 |
| Python | CPython 3.14.6, Clang 21.0.0 |
| Matplotlib | `3.12.0.dev348+gbde111fb4`, local source commit above |
| Backend | `Agg` / `FigureCanvasAgg` |
| FreeType | 2.14.3, build type `local` |
| Font | packaged `mpl-data/fonts/ttf/DejaVuSans.ttf`, SHA-256 `3fdf69cabf06049ea70a00b5919340e2ce1e6d02b0cc3c4b44fb6801bd1e0d22` |
| Runtime dependencies | contourpy 1.3.3; cycler 0.12.1; fonttools 4.63.0; kiwisolver 1.5.0; NumPy 2.5.1; packaging 26.2; Pillow 12.3.0; pyparsing 3.3.2; python-dateutil 2.9.0.post0; six 1.17.0 |
| Config/cache during probe | fresh `/tmp/mplot-mpl-ref-config`; rc file was the installed package's `mpl-data/matplotlibrc` |

The checkout defaults to building its bundled FreeType rather than a system
copy ([`meson.options` lines 4-13](../../../../matplotlib/meson.options)), and
its own image-test guard requires local FreeType 2.14.3 because other versions
cause image-comparison failures
([`__init__.py` lines 1358-1375](../../../../matplotlib/lib/matplotlib/__init__.py)).
That local FreeType build and the packaged DejaVu font are therefore normative,
not merely descriptive of one workstation.

## Exact regeneration contract

Reference generation is a separate maintainer operation. Ordinary Rust CI
only consumes checked-in PNGs and runs `cargo test --test fidelity`; the test
decodes both images, requires equal dimensions, then compares RGB channels
([`tests/fidelity.rs` lines 359-400](../../../tests/fidelity.rs)). It must not
install or invoke Python.

A rebaseline must:

1. Assert the adjacent checkout is clean and exactly at the target SHA.
2. Create a fresh dedicated Python environment from a checked-in, hash-locked
   builder requirements file. Install `../matplotlib` from that checkout with
   Meson's default `system-freetype=false`; reject any build whose
   `ft2font.__freetype_version__` is not `2.14.3` or whose
   `__freetype_build_type__` is not `local`.
3. Run in a fresh empty `MPLCONFIGDIR`, unset `MATPLOTLIBRC`, and set
   `MPLBACKEND=Agg`, `MPL_IGNORE_SYSTEM_FONTS=1`, `LC_ALL=en_US.UTF-8`,
   `TZ=UTC`, `PYTHONHASHSEED=0`, and `SOURCE_DATE_EPOCH=0`. The generator must call `matplotlib.rcdefaults()`
   before applying fixture overrides. This follows Matplotlib's own baseline
   setup: fixed locale, Agg, reset defaults, DejaVu Sans, and default text
   hinting ([`testing/__init__.py` lines 20-51](../../../../matplotlib/lib/matplotlib/testing/__init__.py)).
4. Assert `font_manager.findfont(FontProperties(family="DejaVu Sans"))`
   resolves inside the installed checkout/package and has the font SHA above;
   do not accept a system-font path. Matplotlib otherwise enumerates both its
   packaged and system fonts and caches the result
   ([`font_manager.py` lines 1186-1211](../../../../matplotlib/lib/matplotlib/font_manager.py)).
5. Run the generator once, capture the manifest below, run it a second time in
   another fresh config directory, and require identical dimensions and
   decoded RGBA SHA-256 for every PNG. Byte-for-byte PNG equality is desirable
   only on the canonical locked builder; pixel equality is the portable
   reproducibility requirement because Pillow/zlib may encode the same pixels
   differently.
6. Run `cargo test --test fidelity` without the Python environment on `PATH`
   before accepting the rebaseline.

Canonical command shape (the lock filename is a required follow-on production
artifact; it does not exist today):

```sh
test "$(git -C ../matplotlib rev-parse HEAD)" = bde111fb4ee0023852b48d48f14436647400663d
test -z "$(git -C ../matplotlib status --porcelain)"
python3.14 -m venv /tmp/mplot-mpl-reference
/tmp/mplot-mpl-reference/bin/python -m pip install --require-hashes -r scripts/mpl-reference-requirements.txt
/tmp/mplot-mpl-reference/bin/python -m pip install --no-deps ../matplotlib
env -u MATPLOTLIBRC MPLCONFIGDIR=/tmp/mplot-mpl-config MPLBACKEND=Agg \
  MPL_IGNORE_SYSTEM_FONTS=1 \
  LC_ALL=en_US.UTF-8 TZ=UTC PYTHONHASHSEED=0 SOURCE_DATE_EPOCH=0 \
  /tmp/mplot-mpl-reference/bin/python scripts/mpl_reference.py
cargo test --test fidelity
```

## Defaults versus fixture configuration

The generator currently applies nine global overrides: `figure.dpi=100`, all
base/title/axis/tick font sizes `=10`, `axes.linewidth=0.8`,
`lines.linewidth=1.5`, and `font.family="DejaVu Sans"`
([`scripts/mpl_reference.py` lines 17-31](../../../scripts/mpl_reference.py)).
On the clean pinned build, figure DPI, base font size, axes/line widths already
equal those numeric values, but title/label/tick sizes are symbolic defaults
(`large`/`medium`) and the default family is generic `sans-serif`; the fixture
therefore intentionally makes all rendered text 10 pt and pins family
resolution. Unoverridden relevant library defaults include 6.4x4.8 inches,
5% x/y margins, white figure/axes, solid 1.5 pt lines, image aspect `equal`,
image interpolation `auto`, histogram bins 10, standard (not tight) save bbox,
opaque output, and path simplification enabled.

Every fixture explicitly sets `figsize`; most use 6.4x4.8, with three larger
multi/special cases listed above. Every save explicitly executes
`fig.tight_layout()` and then `savefig(..., dpi=100)`. The layout call uses the
pinned library defaults `pad=1.08`, `h_pad=w_pad=pad`, and full-figure rect
([`figure.py` lines 3633-3665](../../../../matplotlib/lib/matplotlib/figure.py));
it is not `bbox_inches="tight"`, constrained layout, or the default subplot
margins.

Per-case overrides are data plus titles/labels, explicit Tableau colors,
subplot y-limit 0..3, boxplot tick labels, log x/y scale where named, bar width
0.8, histogram bins 10 (explicit although equal to the library default),
fill alpha 0.3, image extent/cmap/origin, and contour levels/color
([`scripts/mpl_reference.py` lines 40-221](../../../scripts/mpl_reference.py)).
All other plotting semantics remain pinned-library defaults: for example
boxplot vertical orientation/1.5-IQR whiskers/unfilled boxes/fliers enabled,
histogram count/bar/mid/vertical behavior, and image aspect/interpolation.
These defaults must be copied into the manifest, not silently assumed.

## Required checked-in manifest

Store one machine-readable sidecar for the corpus (JSON is sufficient), plus
human review in this note. It must contain:

- schema version; UTC generation time as informational data; generator path
  and SHA-256; mplot commit; Matplotlib source SHA, `git describe`, clean flag,
  built `matplotlib.__version__`, and source path;
- canonical builder ID/image digest, OS/architecture, Python full version and
  executable, compiler, exact `pip freeze`/lock digest, NumPy/Pillow versions,
  Agg canvas class, FreeType version/build type, Matplotlib data path, resolved
  font path relative to that data path and font SHA-256;
- sanitized environment values above, locale actually selected, config/cache
  paths expressed as fresh/ephemeral rather than host absolute paths, rc file
  path relative to the installed package, and a sorted snapshot of every
  relevant effective rcParam;
- layout call and arguments; savefig format/DPI/bbox/padding/face/edge/
  transparency and fixed PNG metadata policy; per fixture, function name,
  explicit overrides, inherited-default snapshot, pixel dimensions, color
  mode, file SHA-256, and decoded RGBA SHA-256.

PNG metadata must be deterministic. Override `Software` with a fixed string
containing the full source SHA (or record that SHA only in the sidecar) and do
not add wall-clock creation time. The current PNG writer adds only values
provided to it plus its default `Software`, so this is controllable at
`savefig(metadata=...)`.

## Known variance and calibration implications

- FreeType version/build flags, font choice, hinting, DPI, and platform raster
  behavior change glyph pixels and therefore tight-layout extents. The
  canonical references must use local FreeType 2.14.3 and the packaged font;
  noncanonical builders verify decoded-pixel repeatability but do not
  rebaseline.
- System font discovery/cache contents can change family resolution; a fresh
  `MPLCONFIGDIR`, resolved-font assertion, and font hash close this leak.
- NumPy can affect histogram/percentile/contour numeric results; Pillow and
  zlib can affect PNG bytes and metadata/compression. Pin all three layers and
  compare decoded pixels separately from file bytes.
- `tight_layout` is renderer/font-metric sensitive. Region thresholds must
  allow measured antialiasing at text/line edges, but no tolerance should
  excuse changed dimensions, fixture semantics, axes geometry, or resolved
  font. Existing global mean thresholds of 35-40 are historical and are not
  provenance-based calibration.
- Locale/time zone/hash seed should be fixed even though the present fixtures
  contain no dates and no randomness; doing so prevents future corpus growth
  from silently introducing host dependence. `SOURCE_DATE_EPOCH` primarily
  protects future SVG/PDF metadata and is harmless for PNG.

## Decision

The 3.10.9 corpus must be treated as legacy evidence, not the release oracle.
Rebaseline only through a separately invoked, locked canonical builder for the
exact source SHA, with local FreeType 2.14.3, packaged DejaVu Sans, Agg, clean
rc/cache state, explicit layout/save settings, and a checked-in manifest.
Rust CI remains Python-free and validates only checked-in references.
