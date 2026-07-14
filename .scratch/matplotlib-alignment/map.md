# Chart a finite Matplotlib compatibility release

## Destination

A decision-complete, phased plan ready for `to-spec` and `to-tickets` that aligns every currently advertised mplot capability with the object-oriented API vocabulary, supported semantics, and visual output of Matplotlib commit `bde111fb4ee0023852b48d48f14436647400663d`, while retaining Rust builders and an explicitly finite compatibility boundary.

## Notes

- Normative source: the exact adjacent `../matplotlib` checkout at commit `bde111fb4ee0023852b48d48f14436647400663d` (`v3.11.0-348-gbde111fb4e` when charted). Source references in later artifacts must be commit-relative and inspectable.
- This map plans; it does not implement. When complete, hand its decisions to local `to-spec` and `to-tickets` artifacts under this effort directory.
- Public vocabulary follows Matplotlib's object-oriented API exactly where Rust permits: `Axes`, `add_subplot`, `plot`, `hist`, `imshow`, `set_xlabel`, `set_xlim`, `savefig`, `linewidth`, `linestyle`, `bottom`, `whis`, `orientation`, and corresponding canonical names.
- Rust construction remains `Figure::builder()` with closure-configured `AxesBuilder` values and method-derived typed option builders. Infallible chaining is preserved; contextual validation errors surface from `build()` rather than being silently clamped, ignored, or dropped.
- Breaking the existing 0.1 public API is intentional. No deprecated aliases are planned. Every in-tree example, test, rustdoc example, and README snippet must migrate.
- The finite release includes the eight existing plotting families (`plot`, `boxplot`, `bar`, `hist`, `fill_between`, `imshow`, `contour`, `text`) and all advertised ancillary features: GridSpec/spans, finite tight/constrained layouts, colorbars, legends, linear/log scales, ticks, colors/colormaps, and PNG/SVG/PDF export.
- The release must publish a checked-in compatibility matrix using `Supported`, `Adapted`, and `Unsupported`, with a pinned Matplotlib source reference for every inspected in-scope method and option. Typed Rust values are preferred over Python strings and overloads.
- Fidelity references are checked in and generated explicitly from the pinned checkout. Ordinary Rust CI must not require Python. PNG is the normative visual oracle; SVG/PDF share the compiled scene and receive structural/rasterization smoke coverage.
- Release acceptance uses a calibrated compound fidelity gate: dimensions, tighter whole-image comparison, region-sensitive comparison, and semantic probes. Thresholds must be measured, not guessed.
- Each wayfinder session should use `research` for pinned-source facts, `prototype` for runnable design or fidelity questions, and `domain-modeling`/`grilling` for compatibility and architecture decisions. HITL sessions interview the orchestrator rather than answering decision questions themselves.

## Decisions so far

<!-- Empty when charted. Resolved tickets append one named, linked gist here; detail remains in the ticket. -->

- [Public vocabulary and compatibility matrix](issues/01-public-vocabulary-and-compatibility-matrix.md): use Matplotlib OO names with retained Rust builders and typed method-option builders; remove old aliases and accidental public internals, and classify every method, option, default, return, validation, continuation, and boundary as an independently sourced atomic matrix row.

- [Reference provenance and regeneration](issues/05-reference-provenance-and-regeneration.md): replace the unprovenanced Matplotlib 3.10.9 goldens through a separate locked canonical builder pinned to the target checkout, Agg, local FreeType 2.14.3, packaged DejaVu Sans, deterministic rc/layout/environment, and a checked-in manifest; keep ordinary Rust CI Python-free.

- [Plot-method semantic baselines](issues/02-plot-method-semantic-baselines.md): release 1 keeps one typed Rust call shape for each of the eight plot families, reproduces pinned defaults/state/data-limit/rendering semantics, adapts Artist returns to queued builders, and explicitly rejects every unselected overload and kwarg.

- [Figure, Axes, layout, and ancillary semantics](issues/03-figure-axes-and-ancillary-semantics.md): retain typed Rust builders around canonical OO names, restore pristine pinned defaults, and support only enumerated 1x1/small-grid/single-span/right-colorbar layouts; PNG is normative and SVG/PDF get structural/rasterization smoke coverage, with no general layout-engine claim.

## Not yet specified

- The implementation slices and module seams remain fog until the public/semantic matrices and retained-state design expose the necessary internal boundaries.
- The final fixture corpus, region definitions, and numerical fidelity thresholds remain fog until reference provenance and calibration are established.
- Whether Plotters can be retained unchanged, needs targeted augmentation, or blocks a claimed behavior remains fog until representative compatibility cases are exercised.
- Exact later-phase ordering among plot families and ancillary capabilities remains fog until architectural coupling and renderer feasibility are known.

## Out of scope

- Plot families and interactivity not currently advertised by mplot; these may form a later roadmap but do not block this finite release.
- Mutable Matplotlib Artist/container handles and post-creation introspection in release 1.
- A public warning-emission channel; deterministic continuation behavior and fatal errors remain in scope.
- Arbitrary keyword maps, format strings, stringly typed generic styling, and wholesale reproduction of Python overloads.
- General equivalence with the complete `tight_layout` or constrained-layout engines beyond the finite layouts/options explicitly claimed in the matrix and fixtures.
- Direct vector-output equivalence with Matplotlib as a release gate.
