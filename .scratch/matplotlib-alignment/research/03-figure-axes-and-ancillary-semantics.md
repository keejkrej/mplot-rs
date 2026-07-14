# Figure, Axes, layout, and ancillary semantic baseline

## Scope and evidence

This is the release-1 decision matrix for the non-plot-family surface. It uses
only the current mplot source/tests and the adjacent Matplotlib checkout at
commit `bde111fb4ee0023852b48d48f14436647400663d`. Source paths into Matplotlib
are relative to that checkout. `Supported` means the finite behavior and
default match; `Adapted` means the user-visible capability is retained through
Rust typing, queued builders, ownership, validation timing, or a changed return;
`Unsupported` is an explicit release boundary. A canonical name alone never
promotes a row to `Supported`.

The current implementation is evidence, not the target baseline. In particular,
it defaults saves to 200 DPI and a hand-chosen tight rectangle, silently clamps
invalid grids/spans, stores Axes state in an order-insensitive aggregate, gives
every line the first Tableau color, approximates viridis with five stops, and
implements constrained layout only as fixed margins plus a colorbar inset
(`src/constants.rs:1-14`, `src/figure.rs:64-127,191-279`,
`src/gridspec.rs:19-47,76-105`, `src/panel.rs:81-230`,
`src/colormap.rs:54-94`, `src/render/layout.rs:15-73`,
`src/render/model.rs:6-20`). Those behaviors must not be grandfathered in as
Matplotlib semantics.

## Release-1 public shape

The retained construction shape is:

```rust
Figure::builder()
    .figsize(FigureSize::inches(6.4, 4.8))
    .layout(Layout::Tight)
    .add_subplot((1, 1, 1), |ax| {
        ax.plot(&x, &y, PlotOptions::new());
        ax.set_title("title", SetTitleOptions::new());
        ax.set_xlabel("x", SetLabelOptions::new());
        ax.set_xlim(0.0, 4.0);
    })
    .build()?
    .savefig(path, SavefigOptions::new())?;
```

Figure methods consume/return `FigureBuilder`; Axes methods queue commands and
return `&mut AxesBuilder`; `build()` validates the complete retained program.
Setter order is observable and is preserved. This is essential for such cases
as `set_xticks` expanding limits followed by `set_xlim` replacing them
(`../matplotlib/lib/matplotlib/axis.py:2389-2412,2414-2463`). Mutable
`Axes`, `Artist`, `Legend`, and `Colorbar` handles are outside release 1.

## Figure, subplot, and Axes matrix

| ID | Release-1 element | Status | Finite semantics and default | Source |
|---|---|---|---|---|
| `figure-builder` | `Figure::builder() -> FigureBuilder` | Adapted | Retained Rust construction; default state below is a pristine pinned `Figure`. | Matplotlib constructs `Figure` directly: `../matplotlib/lib/matplotlib/figure.py:2470-2483`; Rust precedent: `src/figure.rs:142-145,191-216`. |
| `figure-figsize` | `FigureBuilder::figsize(FigureSize)` | Adapted | Inches-only typed release input; default `(6.4, 4.8)` inches; both dimensions must be finite and strictly positive at `build()`. Centimeter/pixel tuples and partial `None` dimensions are unsupported. | `../matplotlib/lib/matplotlib/figure.py:2487-2499,2626-2642`; pristine rc values `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:606-607`; current Rust size `src/figure.rs:12-40`. |
| `figure-dpi` | no Figure DPI setter; `SavefigOptions::dpi` | Adapted | A built figure has the pristine 100-DPI baseline; output DPI is selected at `savefig`. Interactive canvas DPI mutation is unsupported. | `../matplotlib/lib/matplotlib/figure.py:2498-2499,2626-2642`; `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:607,737`. |
| `figure-default-patch` | pristine white Figure patch | Supported | White, opaque face; frame enabled; edge white; frame linewidth 0. These are fixed release defaults, not public setters. | Figure resolves patch rc values at `../matplotlib/lib/matplotlib/figure.py:2626-2653`; pristine entries `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:606-610`. |
| `figure-add-subplot-tuple` | `add_subplot((nrows, ncols, index), closure)` | Adapted | Rectilinear-only, 1-based row-major `index`; positive `nrows`, `ncols`, and in-range index are validated at `build`; closure configures the queued Axes and the builder is returned. | Canonical tuple semantics and return: `../matplotlib/lib/matplotlib/figure.py:652-714`; current retained closure: `src/figure.rs:250-263`. |
| `figure-add-subplot-spec` | `add_subplot(SubplotSpec, closure)` | Adapted | Same retained closure using an opaque typed `SubplotSpec`; coordinates are 0-based inside `GridSpec`. | `../matplotlib/lib/matplotlib/figure.py:665-680`; `../matplotlib/lib/matplotlib/gridspec.py:531-559`. |
| `figure-add-subplot-overloads` | 3-digit integers, existing Axes, projections, `polar`, `axes_class`, `sharex/sharey`, arbitrary Axes kwargs | Unsupported | No release-1 representation. The default no-argument `(1,1,1)` form is also omitted so every retained subplot address is explicit. | Full Matplotlib boundary: `../matplotlib/lib/matplotlib/figure.py:656-725`. |
| `figure-empty` | `FigureBuilder::build()` with no subplot | Adapted | Fatal `Error::EmptyFigure` at `build`; empty canvases are outside the advertised plotting release even though Matplotlib permits them. | Current validation `src/figure.rs:265-279`; Matplotlib initializes before Axes exist at `../matplotlib/lib/matplotlib/figure.py:2574-2663`. |
| `gridspec-new` | `GridSpec::new(nrows, ncols)` | Adapted | Positive integers; equal row heights and column widths; bad dimensions are fatal at `build`, never clamped. | Matplotlib validation/default equal ratios: `../matplotlib/lib/matplotlib/gridspec.py:33-56,108-113`; current clamping to remove: `src/gridspec.rs:19-24`. |
| `gridspec-new-subplotspec` | `new_subplotspec((row, col), rowspan, colspan) -> SubplotSpec` | Adapted | 0-based origin; defaults `rowspan=1`, `colspan=1`; positive spans must remain inside the grid. | `../matplotlib/lib/matplotlib/gridspec.py:85-99`; current span storage `src/gridspec.rs:35-47`. |
| `gridspec-indexing` | Python slicing/indexing and negative indices | Unsupported | No operator-style or negative indexing; the named constructor above is the sole release form. `width_ratios`, `height_ratios`, nested GridSpecs, and per-GridSpec subplot margins are also unsupported. | Broader GridSpec contract: `../matplotlib/lib/matplotlib/gridspec.py:101-113,304-330`. |
| `subplotspec` | opaque `SubplotSpec` | Adapted | May be copied and passed to `add_subplot`; no public cell-number or parent introspection. | Matplotlib normally obtains rather than directly instantiates it: `../matplotlib/lib/matplotlib/gridspec.py:531-559`; current overexposed accessors `src/gridspec.rs:51-74`. |
| `axes-builder` | closure-scoped `AxesBuilder<'_>` | Adapted | Retained command builder, not a mutable Matplotlib `Axes`; commands preserve call order and return `&mut AxesBuilder`. | Matplotlib returns `Axes`: `../matplotlib/lib/matplotlib/figure.py:708-714`; current builder precedent `src/panel.rs:244-346`. |
| `axes-set-title` | `set_title(label, SetTitleOptions)` | Adapted | Center title only; default pinned title font properties; release option is `fontsize` only. `loc`, `y`, `pad`, font dictionaries, and other Text properties are unsupported. | `../matplotlib/lib/matplotlib/axes/_axes.py:139-218`. |
| `axes-set-labels` | `set_xlabel` / `set_ylabel(label, SetLabelOptions)` | Adapted | Canonical text setter, with `fontsize` as the sole release option; default label size/pad follow pristine rc. Position, loc, font dictionaries, and arbitrary Text properties are unsupported. | `../matplotlib/lib/matplotlib/axes/_base.py:3745-3766,4013-4034`; current aggregate `src/panel.rs:103-116`. |
| `axes-set-limits` | `set_xlim(left, right)` / `set_ylim(bottom, top)` | Adapted | Both finite endpoints required; reversed order intentionally inverts the axis; calling either disables autoscaling for that axis; return limits are discarded into builder chaining. Partial bounds, aliases, `emit`, and `auto` options are unsupported. | `../matplotlib/lib/matplotlib/axes/_base.py:3916-3951,4165-4200`; current storage `src/panel.rs:128-136`. |
| `axes-grid` | `grid(visible, GridOptions)` | Adapted | Explicit Boolean visibility only; defaults `which=Major`, `axis=Both`; typed `axis={Both,X,Y}` is supported. Toggle-on-`None`, minor/both grids, and arbitrary Line2D grid styling are unsupported. Pristine grid is off. | `../matplotlib/lib/matplotlib/axes/_base.py:3466-3506`; rc defaults `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:391-393`; current Boolean `src/panel.rs:138-141`. |
| `axes-axis-on-off` | `set_axis_off()` / `set_axis_on()` | Adapted | Last queued call wins; suppresses/restores all axis decorations without removing plotted data. The old Boolean `hide` API is removed. | `../matplotlib/lib/matplotlib/axes/_base.py:3711-3734`; current aggregate `src/panel.rs:143-146`. |
| `axes-tick-params` | `tick_params(TickParamsOptions)` | Adapted | Release subset is `axis={Both,X,Y}` plus `labelsize`; replaces the old Figure-global tick font size. Other tick sides, colors, rotation, direction, widths, lengths, and minor selection are unsupported. | `../matplotlib/lib/matplotlib/axes/_base.py:3627-3656`; current global font knobs `src/figure.rs:229-243`. |
| `axes-returns` | all setters/configuration return `&mut AxesBuilder` | Adapted | Matplotlib Text/limits/Tick/Legend values are intentionally unavailable; there is no post-build mutation or introspection. | Representative Matplotlib returns: `../matplotlib/lib/matplotlib/axes/_axes.py:181-218`; `../matplotlib/lib/matplotlib/axes/_base.py:3943-3951,4197-4200`; `../matplotlib/lib/matplotlib/axis.py:2460-2463`. |

## Scales, ticks, property cycle, and autoscale matrix

| ID | Release-1 element | Status | Finite semantics and default | Source |
|---|---|---|---|---|
| `scale-linear` | `set_xscale(Scale::Linear)` / `set_yscale(Scale::Linear)` | Adapted | Typed value; identity transform; default on both axes. | Generated Axes methods `../matplotlib/lib/matplotlib/axes/_base.pyi:416-417,443-444`; linear transform `../matplotlib/lib/matplotlib/scale.py:188-230`; Rust enum `src/series.rs:7-13`. |
| `scale-log` | `Scale::Log` | Adapted | Base 10, automatic log major locator/formatter, `nonpositive="clip"`; release fidelity cases require at least one positive finite datum. | `../matplotlib/lib/matplotlib/scale.py:381-434`; current transform/ticker `src/scale/mod.rs:22-71`, `src/ticker/mod.rs:46-100`. |
| `scale-other` | other registered/custom scales; log `base`, `subs`, `nonpositive="mask"` | Unsupported | Only pristine linear and default base-10 log are release claims. | Extensible accepted type: `../matplotlib/lib/matplotlib/axes/_base.pyi:416-417,443-444`; log options `../matplotlib/lib/matplotlib/scale.py:387-409`. |
| `ticks-auto-linear` | automatic major linear ticks/labels | Supported | Match pinned AutoLocator/ScalarFormatter for the finite reference ranges; no minor ticks. An approximate fixed-count ticker is insufficient. | Linear scale installs locator/formatter: `../matplotlib/lib/matplotlib/scale.py:188-214`; current approximation `src/ticker/mod.rs:3-44,71-84`. |
| `ticks-auto-log` | automatic major log ticks/labels | Supported | Base-10 decade behavior and pinned scientific formatter for finite reference ranges; no minor-tick fidelity claim. | `../matplotlib/lib/matplotlib/scale.py:415-422`; current approximation `src/ticker/mod.rs:46-100`. |
| `ticks-set-x-y` | `set_xticks(ticks, SetTicksOptions)` / `set_yticks(...)` | Adapted | Finite 1-D numeric ticks; options permit same-length labels and label `fontsize`; default `minor=false`. Setting ticks expands the view so all are visible; later explicit limits may replace that expansion. Empty ticks remove major ticks. | `../matplotlib/lib/matplotlib/axis.py:2389-2463`; typed Axes signatures `../matplotlib/lib/matplotlib/axes/_base.pyi:418-426,445-453`. |
| `ticks-invalid-labels` | mismatched tick/label lengths | Adapted | Fatal at `build`; never silently ignore the whole call as current mplot does. | Matplotlib requires equal lengths `../matplotlib/lib/matplotlib/axis.py:2435-2438,2460-2463`; current silent continuation `src/panel.rs:148-176`. |
| `ticks-other` | minor ticks, arbitrary Formatter/Locator/Text kwargs, standalone tick-label setter | Unsupported | No release representation. | Matplotlib boundary `../matplotlib/lib/matplotlib/axis.py:2423-2445`. |
| `cycle-default` | pristine per-Axes property cycle | Supported | Independent `tab10` color cycle for each Axes; a plotting call without explicit color consumes the next color, while an explicit color does not advance a color-only cycle. The exact ten colors are pinned below. | rc default `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:437`; conditional advancement `../matplotlib/lib/matplotlib/axes/_base.py:205-238,350-354`; current hard-coded first color `src/series.rs:46-55`. |
| `cycle-set` | `set_prop_cycle(ColorCycle)` | Adapted | Typed, non-empty color-only sequence; affects only later queued plotting commands on that Axes. Invalid/empty cycles fail at `build`. `None` reset and multi-property cyclers are unsupported. | Matplotlib ordering and future-only effect `../matplotlib/lib/matplotlib/axes/_base.py:1624-1698`. |
| `autoscale-default` | implicit autoscale | Supported | On for x and y until explicit limits disable the relevant axis; default margins are 5% in transformed data space with `axes.autolimit_mode=data`. Artist sticky edges remain effective, notably for image extents and bar/hist baselines. | rc defaults `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:445-448`; Axes initializes margins/state at `../matplotlib/lib/matplotlib/axes/_base.py:1362-1380`; autoscale behavior `../matplotlib/lib/matplotlib/axes/_base.py:3038-3130`; current broad 5% expansion `src/axes/mod.rs:60-215`. |
| `autoscale-method` | `autoscale(enable, AutoscaleOptions)` | Adapted | Supports typed `axis={Both,X,Y}` and `tight={None,True,False}`; default `enable=true`, axis both, tight none. It affects later view resolution in retained command order. | `../matplotlib/lib/matplotlib/axes/_base.py:3038-3090`. |
| `margins-method` | `margins(MarginsOptions)` | Adapted | Supports explicit x and/or y values greater than `-0.5`, and typed tight state; evaluated in queued order. Getter form and positional Python overloads are unsupported. | `../matplotlib/lib/matplotlib/axes/_base.py:2897-3012`. |

## Legend, colorbar, colors, and colormaps matrix

| ID | Release-1 element | Status | Finite semantics and default | Source |
|---|---|---|---|---|
| `legend-call` | `legend(LegendOptions::new())` | Adapted | Automatic-label form only; scans supported line/bar/hist/fill-between commands in plotting order, excludes empty labels and labels starting `_`, and returns builder chaining rather than `Legend`. Calling with no eligible entry deterministically draws no legend. | Automatic discovery and underscore rule `../matplotlib/lib/matplotlib/axes/_axes.py:220-273`; return `../matplotlib/lib/matplotlib/axes/_axes.py:331-333`; current subset `src/render/legend.rs:27-68`. |
| `legend-defaults` | default legend appearance/placement | Supported | `loc=Best`, one column, frame on, frame alpha 0.8, inherited white face, edge color 0.8, medium text. The finite fixtures must exercise a placement where pinned `Best` is deterministic; fixed upper-right placement is not silently called `Best`. | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:572-595`; `../matplotlib/lib/matplotlib/legend.py:362-418`; current fixed upper-right geometry `src/render/legend.rs:84-120`. |
| `legend-other` | explicit handles/labels, non-default locations or styling, Figure legend, outside legends | Unsupported | Artist handles are unavailable; no overload fiction. | Full call boundary `../matplotlib/lib/matplotlib/axes/_axes.py:235-337`. |
| `colorbar-call` | `AxesBuilder::colorbar(ColorbarOptions)` | Adapted | Must immediately follow an `imshow` or `contour` command and binds to that retained mappable; at most one colorbar per parent Axes. Missing/ambiguous mappable or a second colorbar is fatal at `build`. Ownership differs because Matplotlib's canonical owner is `Figure.colorbar(mappable, ...)`. | Matplotlib ownership/return `../matplotlib/lib/matplotlib/figure.py:1202-1243`; current Boolean scan/last-wins behavior to remove `src/compile/mod.rs:35-47`, `src/render/canvas.rs:199-228`. |
| `colorbar-defaults` | default `ColorbarOptions` | Adapted | Right-side vertical bar, ticks on right, `fraction=0.15`, `pad=0.05`, `shrink=1`, `aspect=20`; normalizer/colormap come from the bound mappable. Works in the finite layouts below. | Defaults and placement `../matplotlib/lib/matplotlib/colorbar.py:33-55,1360-1394,1415-1453`. |
| `colorbar-other` | other locations/orientations, multiple/shared/manual `cax`, labels/ticks/extend, non-default geometry | Unsupported | No release representation. | Broad Figure API `../matplotlib/lib/matplotlib/figure.py:1223-1260`; colorbar keyword boundary `../matplotlib/lib/matplotlib/colorbar.py:76-117`. |
| `color-rgb` | `Color::rgb(u8,u8,u8)` | Adapted | Opaque typed sRGB bytes. Float tuples, RGBA tuples, grayscale strings, `none`, and arbitrary color-like Python objects are unsupported as inputs. Alpha remains an independently validated plot option. | Matplotlib RGBA result and accepted representations `../matplotlib/lib/matplotlib/colors.py:314-440`; current typed bytes `src/color.rs:3-27`. |
| `color-hex` | `Color::from_hex("#RRGGBB") -> Result<Color>` | Adapted | Exactly six hex digits; invalid text is fatal at the call boundary, never default blue or partially zero-filled. Short/alpha hex forms are unsupported. | Matplotlib hex parsing/errors `../matplotlib/lib/matplotlib/colors.py:390-421`; current silent fallback `src/color.rs:29-46`. |
| `color-tableau` | `Color::TABLEAU_COLORS` | Adapted | Exact ordered ten-color `tab10` palette: `#1f77b4`, `#ff7f0e`, `#2ca02c`, `#d62728`, `#9467bd`, `#8c564b`, `#e377c2`, `#7f7f7f`, `#bcbd22`, `#17becf`. | `../matplotlib/lib/matplotlib/_color_data.py:13-25`; current values `src/color.rs:11-23`. |
| `color-names` | named CSS/XKCD/base colors and `C<n>` syntax | Unsupported | Constants/palette and typed RGB/hex are the finite input forms. | Matplotlib mappings/cycle syntax `../matplotlib/lib/matplotlib/colors.py:76-88,343-347,380-421`. |
| `cmap-typed` | `Colormap::{Viridis, Gray}` | Adapted | Typed two-map subset; `Viridis` is the exact pinned 256-entry table and `Gray` the exact linear map. Default `cmap` is Viridis. `Grayscale` is renamed `Gray`. | Default `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:652`; registry alias `../matplotlib/lib/matplotlib/cm.py:46-50`; viridis data `../matplotlib/lib/matplotlib/_cm_listed.py:774-1031`; gray definition `../matplotlib/lib/matplotlib/_cm.py:203-204`; current approximation `src/colormap.rs:3-9,54-94`. |
| `cmap-other` | all other registered/custom/reversed/resampled colormaps and bad/under/over customization | Unsupported | No release representation. Invalid typed values are unrepresentable. | General colormap contract `../matplotlib/lib/matplotlib/colors.py:713-790`. |
| `normalize-auto` | omitted `norm` | Supported | Derive `vmin`/`vmax` from finite unmasked input; equal bounds map to 0. Invalid/masked color handling is not claimed beyond rejecting non-finite Rust input during build. | `../matplotlib/lib/matplotlib/colors.py:2427-2458,2536-2564`; current auto range `src/colormap.rs:23-40`. |
| `normalize-explicit` | typed `Normalize` with `vmin`, `vmax`, `clip` | Adapted | Optional finite bounds; default `clip=false`; `vmin>vmax` is fatal when built/evaluated. For `clip=false`, normalized values may leave `[0,1]`, while default colormap under/over colors equal endpoint colors. | `../matplotlib/lib/matplotlib/colors.py:2405-2458,2536-2564`; current always-clipped behavior `src/colormap.rs:42-50`. |
| `normalize-other` | other Norm subclasses/string scale names | Unsupported | Linear `Normalize` only. | Matplotlib base/extension point `../matplotlib/lib/matplotlib/colors.py:2360-2403`. |

## Finite layout matrix

`Layout` is a typed Rust enum with exactly `None`, `Tight`, and `Constrained`.
Its default is `None`, matching pristine `figure.autolayout=false` and
`figure.constrained_layout.use=false` (`../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:620-633`).
The names correspond to pinned engines, but support is case-based, not an
engine-equivalence claim (`../matplotlib/lib/matplotlib/figure.py:2688-2735`).

| Case ID | Layout | Status | Exact release-1 case |
|---|---|---|---|
| `layout-none-1x1` | `None` | Supported | One rectilinear Axes at pristine subplot bounds left `.125`, right `.9`, bottom `.11`, top `.88`; no colorbar. |
| `layout-none-grid` | `None` | Supported | Equal-ratio, non-overlapping `1xN`, `Nx1`, or `2x2` GridSpec with default `wspace=hspace=.2`; each Axes occupies one cell; no colorbar. |
| `layout-none-span` | `None` | Supported | Equal-ratio GridSpec no larger than `2x2`, with one rectangular contiguous row/column span and no overlapping SubplotSpecs; no colorbar. |
| `layout-tight-1x1` | `Tight` | Supported | One Axes, optionally title/xlabel/ylabel/custom major labels/inside legend, using pinned `pad=1.08`, `h_pad=w_pad=pad`, and full-figure rect. |
| `layout-tight-grid` | `Tight` | Supported | Equal-ratio non-overlapping `1xN`, `Nx1`, or `2x2` grids, optionally the single contiguous span above, with the same finite decorations; no colorbar. |
| `layout-constrained-1x1-cbar` | `Constrained` | Supported | One Axes, with or without one default right vertical colorbar bound to its imshow/contour mappable. |
| `layout-constrained-grid` | `Constrained` | Supported | Equal-ratio non-overlapping `1xN`, `Nx1`, or `2x2`, optionally the single contiguous span above; at most one default right vertical colorbar per occupied Axes. |
| `layout-other` | any | Unsupported | Grids larger than the enumerated fixture cases until separately matrixed; overlapping specs; unequal ratios; nested/subfigure layouts; shared/twin/inset Axes; suptitles; outside legends; multiple/shared/horizontal/non-right colorbars; compressed/custom engines; arbitrary layout parameters; post-build relayout. |

The `Tight` defaults come from the pinned engine
(`../matplotlib/lib/matplotlib/layout_engine.py:137-193`), while constrained
layout's solver and padding are substantially broader
(`../matplotlib/lib/matplotlib/layout_engine.py:216-282`). Passing the finite
fixtures therefore proves only the rows above. It must never produce a README
claim that mplot implements Matplotlib's general tight- or constrained-layout
engine. The current fixed `MPL_TIGHT_*` rectangle is not an implementation of
either engine (`src/render/mpl_style.rs:43-51`, `src/render/layout.rs:15-34`).

## Pristine defaults ledger

These defaults apply before fixture overrides. They are independently testable
matrix rows, not prose inherited from a parent method.

| Default ID | Pinned value | Release status | Source |
|---|---|---|---|
| `default-figsize` | `6.4 x 4.8 in` | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:606`. |
| `default-figure-dpi` | `100` | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:607`. |
| `default-layout` | none | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:630-633`; `../matplotlib/lib/matplotlib/figure.py:2719-2726`. |
| `default-subplot-box` | `.125/.9/.11/.88`, `wspace=.2`, `hspace=.2` | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:620-627`. |
| `default-axes-margins` | x/y `.05`; autolimit mode `data` | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:445-448`. |
| `default-cycle` | color-only `tab10`, per Axes | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:437`; `../matplotlib/lib/matplotlib/axes/_base.py:1377-1378`. |
| `default-scales` | linear x and y | Supported | scale setters are conditional at `../matplotlib/lib/matplotlib/axes/_base.py:760-763`; linear scale `../matplotlib/lib/matplotlib/scale.py:188-230`. |
| `default-grid` | off, major, both axes | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:391-393`. |
| `default-font-family` | generic sans-serif headed by DejaVu Sans; base 10 pt | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:259-276`. |
| `default-axes-fonts` | title `large` = 12 pt; labels/tick labels `medium` = 10 pt | Supported | axes settings `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:394-402`; scale factors `../matplotlib/lib/matplotlib/font_manager.py:53-63`. The current reference generator's forced 10-pt title is a fixture override, not pristine state (`scripts/mpl_reference.py:17-31`). |
| `default-save-dpi` | `'figure'` => 100 pristine | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:737`; `../matplotlib/lib/matplotlib/figure.py:3410-3412`. |
| `default-save-format` | PNG if no format/extension in Matplotlib | Adapted | Rust requires a path and either a recognized extension or typed `format`; it does not append `.png`. Pinned behavior: `../matplotlib/lib/matplotlib/figure.py:3381-3390`; current resolver: `src/figure.rs:51-61,121-126`. |
| `default-save-bbox` | standard (`None`), not tight | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:741`; `../matplotlib/lib/matplotlib/figure.py:3435-3437`. |
| `default-save-pad` | `0.1 in`, effective only for tight bbox | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:744`; `../matplotlib/lib/matplotlib/figure.py:3439-3443`. |
| `default-save-colors` | face/edge `auto`; opaque | Supported | `../matplotlib/lib/matplotlib/mpl-data/matplotlibrc:738-749`; `../matplotlib/lib/matplotlib/figure.py:3394-3405,3445-3451`. |

## `savefig` PNG/SVG/PDF matrix

| ID | Release-1 element | Status | Finite semantics and default | Source |
|---|---|---|---|---|
| `savefig-call` | `Figure::savefig(path, SavefigOptions) -> Result<()>` | Adapted | Canonical method name with Rust path and error result. The typed `format`, when set, wins without rewriting the extension; otherwise `.png`, `.svg`, or `.pdf` is inferred case-insensitively. Missing/unknown extension is fatal at `savefig`. | `../matplotlib/lib/matplotlib/figure.py:3360-3390`; current dispatch `src/figure.rs:147-155`, `src/render/export.rs:7-16`. |
| `savefig-dpi` | `dpi(u32)` | Adapted | Positive integer; default 100 (`figure`). PNG dimensions are `round(figsize_inches * dpi)` before an optional tight crop. For SVG/PDF, physical page size remains `figsize * 72 pt`; DPI affects only rasterized content, not vector page geometry. | `../matplotlib/lib/matplotlib/figure.py:3410-3412`; SVG mixed-mode sizing `../matplotlib/lib/matplotlib/backends/backend_svg.py:1317-1366`; PDF point sizing `../matplotlib/lib/matplotlib/backends/backend_pdf.py:2715-2742`; current backend dimensions to correct `src/render/canvas.rs:23-30`. |
| `savefig-bbox` | `bbox_inches(BboxInches::{Standard,Tight})` | Adapted | Typed subset; default Standard. Tight computes the union of finite in-layout Figure/Axes decorations and then applies `pad_inches`; it is not a synonym for `Layout::Tight`. Arbitrary Bboxes and extra artists are unsupported. | `../matplotlib/lib/matplotlib/figure.py:3435-3443`; print boundary `../matplotlib/lib/matplotlib/backend_bases.py:2182-2194`; current fixed-margin behavior `src/render/layout.rs:15-34`. |
| `savefig-pad` | `pad_inches(f64)` | Adapted | Finite non-negative value; default 0.1; ignored under Standard bbox. `'layout'` is unsupported. | `../matplotlib/lib/matplotlib/figure.py:3439-3443`; current unconditional canvas growth to remove `src/render/canvas.rs:23-30`. |
| `savefig-png` | `ExportFormat::Png` | Supported | Raster output and normative pixel-fidelity oracle; dimensions, compound image comparison, and semantic probes gate release. Checked-in references come from pinned Agg, but ordinary Rust CI invokes no Python. | Pinned save supports backend-selected PNG `../matplotlib/lib/matplotlib/figure.py:3360-3427`; current native PNG `src/render/png.rs:9-16`; provenance contract `.scratch/matplotlib-alignment/research/reference-provenance-and-regeneration.md`. |
| `savefig-svg` | `ExportFormat::Svg` | Adapted | Same compiled scene and page geometry as PNG, emitted as SVG; structural parse, expected dimensions/elements, and rasterization smoke are required. Direct XML/path equivalence with Matplotlib is not a release gate. | Matplotlib format/metadata surface `../matplotlib/lib/matplotlib/figure.py:3414-3427`; current shared renderer `src/render/svg.rs:8-24`. |
| `savefig-pdf` | `ExportFormat::Pdf` | Adapted | Same compiled scene and physical page geometry, emitted as one-page PDF; header/page/dimension checks and rasterization smoke are required. Direct PDF object/byte equivalence is not a release gate. | Matplotlib format/metadata surface `../matplotlib/lib/matplotlib/figure.py:3414-3427`; current SVG-derived PDF `src/render/pdf.rs:8-23`. |
| `savefig-other-options` | transparent, metadata, facecolor, edgecolor, backend, orientation, file-like output | Unsupported | Fixed opaque white output and filesystem paths only in release 1. No other format is accepted. | Full pinned boundary `../matplotlib/lib/matplotlib/figure.py:3360-3455`; backend format rejection `../matplotlib/lib/matplotlib/backend_bases.py:2085-2145`. |

PNG is the only direct visual oracle. SVG and PDF still must render the same
semantic scene, but their acceptance is deliberately structural plus
rasterization-smoke coverage. This follows the effort's provenance decision
without pretending that Plotters/svg2pdf object structure is Matplotlib's.

## Validation and continuation rules

- `build()` rejects non-finite/non-positive figure sizes, invalid subplot/grid
  addresses or spans, non-finite limits/ticks/margins/norm bounds, mismatched
  tick labels, impossible log-only data cases, invalid property cycles, and
  missing/ambiguous colorbar mappables. Current clamping, saturation, and
  silent drops are not retained (`src/gridspec.rs:19-24,39-47,76-105`,
  `src/panel.rs:148-176`).
- `savefig()` rejects zero DPI, negative/non-finite padding, unrecognized
  format resolution, and backend/write failures. It does not mutate the built
  figure.
- Plot commands and setters remain infallible while queued; cross-command
  validation happens at `build()`. Parse-like constructors such as
  `Color::from_hex` fail at the call boundary because they do not require
  Figure context.
- Unsupported Python overloads/kwargs are absent typed APIs, not ignored
  inputs. There is no warning channel and no silent fallback to a default
  color, layout, range, or format.

## Decision

Release 1 uses Matplotlib's canonical Figure/Axes/GridSpec/SubplotSpec and
setter vocabulary with retained Rust builders and typed options. It resets
pristine defaults to the pinned values, including 100 DPI, standard save bbox,
0.1-inch tight padding, 5% autoscale margins, independent `tab10` cycles, and
layout none. It supports only the explicitly enumerated 1x1, small equal-grid,
single-span, and right-colorbar layout cases. PNG is normative; SVG/PDF share
the semantic scene and get structural/rasterization smoke gates. Everything
outside those rows is `Unsupported`, especially general layout-engine
equivalence, Artist handles, arbitrary Python kwargs/overloads, custom
locators/scales/colormaps, and direct vector-output equivalence.
