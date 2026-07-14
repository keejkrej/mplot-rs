# Public vocabulary and compatibility-matrix schema

## Scope and evidence

This note answers only the naming and classification question. It does not select plotting defaults, edge-case behavior, validation rules, or fidelity thresholds; those belong to later map tickets.

The two normative source trees are:

- mplot's current public Rust source. The actual exposure topology is the private modules plus the `prelude` and crate-root wildcard re-exports in `src/lib.rs:38-73`.
- adjacent Matplotlib at commit `bde111fb4ee0023852b48d48f14436647400663d` (verified from `../matplotlib/.git`). The relevant OO entry points are `Figure`, `Axes`, `GridSpec`, `SubplotSpec`, `Normalize`, and the typed concepts adapted from their parameters.

Matplotlib calls a subplot's plotting object `Axes` and returns it from `Figure.add_subplot` (`../matplotlib/lib/matplotlib/figure.py:652-725`). `GridSpec` indexing returns `SubplotSpec` (`../matplotlib/lib/matplotlib/gridspec.py:304-317,531-550`). Those names are therefore normative; `Panel`, `GridPos`, and `SubplotSlot` are not compatibility vocabulary.

## Target public shape

The release should expose canonical names at the crate root and mirror the useful subset in `mplot::prelude`; implementation modules may remain private. Rust construction remains retained and builder-based:

```rust
Figure::builder()
    .figsize(FigureSize::inches(6.4, 4.8))
    .add_subplot((1, 1, 1), |ax: &mut AxesBuilder<'_>| {
        ax.plot(&x, &y, PlotOptions::new().linewidth(1.5));
        ax.set_xlabel("x", SetLabelOptions::new());
    })
    .build()?
    .savefig(path, SavefigOptions::new())?;
```

The closure and option builders are deliberate `Adapted` Rust representations. Method and option spellings inside them are Matplotlib spellings. In particular, do not retain deprecated aliases for the old names.

### Exhaustive public type inventory

| Current public type/concept | Current evidence | Target public name/disposition | Vocabulary classification |
|---|---|---|---|
| `Figure` | `src/figure.rs:129-189` | `Figure` | Supported exact name |
| `FigureBuilder` | `src/figure.rs:191-279` | `FigureBuilder`, obtained through `Figure::builder()` | Adapted construction |
| `Size` | `src/figure.rs:12-40` | `FigureSize` (`inches`, width/height accessors are Rust adaptations) | Adapted typed `figsize` |
| `SaveOptions` | `src/figure.rs:64-127` | `SavefigOptions` | Adapted method-derived options |
| `ExportFormat::{Png,Svg,Pdf}` | `src/figure.rs:42-62` | unchanged typed values; `from_path` becomes private | Adapted typed `format` |
| `PanelBuilder` | `src/panel.rs:244-347` | `AxesBuilder` | Supported domain name, adapted retained builder |
| `AxesStyle` | `src/panel.rs:81-230` | no aggregate type; canonical `AxesBuilder` methods below | Unsupported old wrapper |
| `GridPos` | `src/panel.rs:5-41` | no named public wrapper; `(nrows, ncols, index)` accepted by `add_subplot` through a Rust conversion | Unsupported old name / adapted call form |
| `GridSpec` | `src/gridspec.rs:1-49`; prelude exposure `src/lib.rs:59` | `GridSpec` | Supported exact name, adapted constructor |
| `SubplotSlot` | `src/gridspec.rs:8-17,51-107`; prelude exposure `src/lib.rs:59` | opaque `SubplotSpec` | Supported canonical name, adapted Rust representation |
| `TickLabels` | `src/panel.rs:43-65` | no standalone type; `SetTicksOptions::labels` | Unsupported old wrapper |
| `LegendStyle` | `src/panel.rs:67-77` | `LegendOptions` (empty/default for today's `show`) | Unsupported old wrapper / adapted options |
| `Scale::{Linear,Log}` | `src/series.rs:7-13` | `Scale::{Linear,Log}` used by `set_xscale`/`set_yscale` | Adapted typed value (Matplotlib accepts string or `ScaleBase`, `../matplotlib/lib/matplotlib/axes/_base.pyi:416-417,443-444`) |
| style struct `LineStyle` | `src/series.rs:35-117` | `PlotOptions` | Unsupported misleading old type name |
| `LineDash::{Solid,Dashed,DashDot,Dotted}` | `src/series.rs:15-23` | `LineStyle::{Solid,Dashed,DashDot,Dotted}` | Adapted typed `linestyle` |
| `Marker::{None,Circle,Square,Cross}` | `src/series.rs:25-33` | `Marker`; rename ambiguous `Cross` to the actual rendered canonical marker (`Plus` or `X`) during semantic baseline work | Adapted typed `marker`; no ambiguous alias (`../matplotlib/lib/matplotlib/markers.py:167-208`) |
| `BoxplotStyle` | `src/series.rs:119-201` | `BoxplotOptions` | Adapted method-derived options |
| `BarStyle` | `src/series.rs:203-263` | `BarOptions` | Adapted method-derived options |
| `HistStyle` | `src/series.rs:265-314` | `HistOptions` | Adapted method-derived options |
| `FillBetweenStyle` | `src/series.rs:316-365` | `FillBetweenOptions` | Adapted method-derived options |
| `ImageStyle` | `src/series.rs:367-427` | `ImshowOptions` | Adapted method-derived options |
| `ContourStyle` | `src/series.rs:429-511` | `ContourOptions` | Adapted method-derived options |
| `TextStyle` | `src/series.rs:513-551` | `TextOptions` | Adapted method-derived options |
| no current orientation type | current booleans at `src/series.rs:127,173-175` | `Orientation::{Vertical,Horizontal}` for canonical `orientation` options | Adapted typed string enum |
| `Series` and all public variant fields/constructors | `src/series.rs:553-673` | private retained command/artist representation; no public replacement | Unsupported accidental internal |
| `Color` with public `r/g/b`, `TABLEAU`, `rgb`, `hex`, `with_alpha` | `src/color.rs:3-55` | typed `Color`; private fields; `rgb` and fallible `from_hex` are Rust adaptations; canonical palette spelling is `TABLEAU_COLORS`; remove white-preblending `with_alpha` from the compatibility surface | Adapted typed Matplotlib color; old representation details unsupported (`../matplotlib/lib/matplotlib/_color_data.py:1-25`) |
| `Colormap::{Viridis,Grayscale}` and public `map` | `src/colormap.rs:3-9,54-64`; prelude exposure `src/lib.rs:56` | `Colormap::{Viridis,Gray}`; make renderer lookup private | Adapted typed `cmap` (Matplotlib's spelling is `gray`, `../matplotlib/lib/matplotlib/axes/_axes.py:6212-6223`) |
| `Normalize { vmin, vmax }`, `new`, `of_slice`, `apply` | `src/colormap.rs:11-52`; prelude exposure `src/lib.rs:56` | `Normalize` with typed constructor/options; fields and evaluation/autoscale helpers private | Adapted OO type (`../matplotlib/lib/matplotlib/colors.py:2405-2458,2536-2555`) |
| `Error`, `Result<T>` | `src/error.rs:3-38` | retain as Rust error infrastructure; variant set is not part of Matplotlib vocabulary | Adapted Rust infrastructure |
| `AsVector`, `vector_to_f64` | `src/as_vector.rs:3-55`; leaked by `src/lib.rs:67` | private input conversion, using standard public bounds at API edges | Unsupported accidental internal |
| `linspace` | `src/auxiliary.rs:1-10`; leaked by `src/lib.rs:68` | remove from compatibility surface (NumPy utility, not Matplotlib OO API) | Unsupported accidental utility |

The public `axes`, `scale`, `ticker`, `constants`, render, and compile items are currently module-private (`src/lib.rs:38-52`), so their `pub` declarations are not externally reachable and are not current API. In particular `DataBounds`, `ViewLimits`, `PanelScales`, `TickPlan`, ticker helpers, and default constants must remain implementation details; they must not be promoted into the compatibility matrix merely because they use `pub` internally.

### Exhaustive method and option rename inventory

| Current name | Target name | Canonical evidence | Classification / note |
|---|---|---|---|
| `Figure::builder` | `Figure::builder` | `src/figure.rs:142-145` | Adapted Rust construction; keep |
| `Figure::save(path, SaveOptions)` | `Figure::savefig(path, SavefigOptions)` | Matplotlib `savefig` and option spellings `dpi`, `format`, `bbox_inches`, `pad_inches`: `../matplotlib/lib/matplotlib/figure.py:3360-3416` | exact method name, adapted `Result<()>` |
| `FigureBuilder::new` | no primary entry point | `src/figure.rs:204-216` | make private; `Figure::builder` is the Rust entry point |
| `FigureBuilder::size(Size)` | `figsize(FigureSize)` | Figure constructor parameter `figsize`: `../matplotlib/lib/matplotlib/figure.py:2470-2499` | canonical option name, typed adaptation |
| `gaps(horizontal, vertical)` | `subplots_adjust(SubplotsAdjustOptions::new().wspace(...).hspace(...))` | `../matplotlib/lib/matplotlib/figure.py:1322-1351` | canonical method/options, queued adaptation |
| `label_fontsize` | per-Axes `set_xlabel`/`set_ylabel` `fontsize` option | `../matplotlib/lib/matplotlib/axes/_base.py:3745-3766,4013-4034` | old global convenience unsupported; capability remains per Axes |
| `tick_fontsize` | `tick_params(TickParamsOptions::new().labelsize(...))` | canonical method and `labelsize`: `../matplotlib/lib/matplotlib/axes/_base.py:3627-3656` | old global convenience unsupported; canonical per-Axes replacement |
| `title_fontsize` | `set_title(..., SetTitleOptions::new().fontsize(...))` | `../matplotlib/lib/matplotlib/axes/_axes.py:139-190` | old global convenience unsupported; canonical per-Axes replacement |
| `constrained_layout(bool)` | `layout(Layout::{None,Tight,Constrained})` | Figure `layout` parameter and engine names: `../matplotlib/lib/matplotlib/figure.py:2479-2482,2688-2716` | typed adaptation |
| `panel(slot, closure)` | `add_subplot(spec, closure)` | `../matplotlib/lib/matplotlib/figure.py:652-680` | exact canonical method name; closure-return adaptation |
| `Size::inches`, `width_in`, `height_in` | `FigureSize::inches`, `width_inches`, `height_inches` | units and `set_size_inches`: `../matplotlib/lib/matplotlib/figure.py:2487-2496,3129-3157` | adapted typed value; spell out `inches` |
| `SaveOptions::dpi` | `SavefigOptions::dpi` | `../matplotlib/lib/matplotlib/figure.py:3366-3369,3410-3412` | exact option |
| `SaveOptions::tight(bool)` | `bbox_inches(BboxInches::Tight)` | `savefig` call signature at `../matplotlib/lib/matplotlib/figure.py:3366-3369` | typed canonical option; old name unsupported |
| `SaveOptions::pad_inches` | `SavefigOptions::pad_inches` | same source | exact option |
| `SaveOptions::format` | `SavefigOptions::format` | `../matplotlib/lib/matplotlib/figure.py:3381-3389,3414-3416` | exact option, typed value |
| `GridSpec::new(rows, cols)` | `GridSpec::new(nrows, ncols)` | `../matplotlib/lib/matplotlib/gridspec.py:33-56,314-317` | Rust constructor adaptation; canonical parameter names |
| `rows`, `cols` | `nrows`, `ncols` | `../matplotlib/lib/matplotlib/gridspec.py:70-73` | exact property vocabulary |
| `at`, `span` | `new_subplotspec((row,col), rowspan, colspan)` | `../matplotlib/lib/matplotlib/gridspec.py:85-99` | exact canonical method/options, Rust tuple/arguments adapted |
| `SubplotSlot` getters (`grid_rows`, `grid_cols`, `row`, `col`, `rowspan`, `colspan`) | opaque `SubplotSpec`; only canonical `rowspan`/`colspan` may later be exposed with Matplotlib range semantics | `src/gridspec.rs:51-74`; `../matplotlib/lib/matplotlib/gridspec.py:531-559` | old introspection unsupported |
| `PanelBuilder::line` | `AxesBuilder::plot` | `../matplotlib/lib/matplotlib/axes/_axes.py:1549-1575` | exact method name |
| `boxplot` | `boxplot` | `../matplotlib/lib/matplotlib/axes/_axes.py:4370-4379` | exact |
| `bar` | `bar` | `../matplotlib/lib/matplotlib/axes/_axes.py:2308-2315` | exact |
| `histogram` | `hist` | `../matplotlib/lib/matplotlib/axes/_axes.py:7260-7263` | exact canonical name; old name unsupported |
| `fill_between` | `fill_between` | `../matplotlib/lib/matplotlib/axes/_axes.py:6180-6192` | exact |
| `image` | `imshow` | `../matplotlib/lib/matplotlib/axes/_axes.py:6212-6218` | exact canonical name; old name unsupported |
| `contour` | `contour` | `../matplotlib/lib/matplotlib/axes/_axes.py:7202-7216` | exact |
| `text` | `text` | `../matplotlib/lib/matplotlib/axes/_axes.py:654-692` | exact |
| `PanelBuilder::series` | no public replacement | `src/panel.rs:338-341` | accidental retained-command escape hatch; private |
| `PanelBuilder::axes(AxesStyle)` | no replacement wrapper | `src/panel.rs:343-346` | call canonical Axes methods directly |
| `AxesStyle::title` | `set_title` | `../matplotlib/lib/matplotlib/axes/_axes.py:139-190` | exact canonical method |
| `x_label`, `y_label` | `set_xlabel`, `set_ylabel` | `../matplotlib/lib/matplotlib/axes/_base.py:3745-3766,4013-4034` | exact |
| `x_scale`, `y_scale` | `set_xscale`, `set_yscale` | generated Axes methods typed at `../matplotlib/lib/matplotlib/axes/_base.pyi:416-417,443-444` | exact method, typed value |
| `x_range`, `y_range` | `set_xlim`, `set_ylim` | `../matplotlib/lib/matplotlib/axes/_base.py:3916-3951,4165-4200` | exact |
| `grid(bool)` | `grid(visible, GridOptions)` | `../matplotlib/lib/matplotlib/axes/_base.py:3466-3488` | exact method/`visible`, typed `which`/`axis` options if claimed |
| `hide(bool)` | `set_axis_off()` / `set_axis_on()` | `../matplotlib/lib/matplotlib/axes/_base.py:3711-3734` | split into canonical operations; old boolean unsupported |
| `x_tick_labels`, `y_tick_labels` | `set_xticks(ticks, SetTicksOptions::labels(...))`, `set_yticks(...)` | Axes wrappers `../matplotlib/lib/matplotlib/axes/_base.pyi:418-426,445-453`; semantics/options `../matplotlib/lib/matplotlib/axis.py:2414-2463` | exact methods/options; typed builder adaptation |
| `legend(LegendStyle)` | `legend(LegendOptions)` | `../matplotlib/lib/matplotlib/axes/_axes.py:235-331` | exact method; default options means show |
| `LineStyle::color` | `PlotOptions::color` | plot forwards `Line2D` properties, with example names: `../matplotlib/lib/matplotlib/axes/_axes.py:1569-1575,1682-1688` | exact option |
| `LineStyle::width` | `PlotOptions::linewidth` | same source | old name unsupported |
| `LineStyle::dash` | `PlotOptions::linestyle` | same source | old name unsupported; typed enum |
| `LineStyle::{marker,label,alpha}` | `PlotOptions::{marker,label,alpha}` | same method/source; alpha is a standard Artist property | exact option names, typed values where relevant |
| `BoxplotStyle::width` | `BoxplotOptions::widths` | signature `../matplotlib/lib/matplotlib/axes/_axes.py:4370-4379` | canonical plural |
| `whisker` | `whis` | same source | exact canonical name |
| `positions` | `positions` | same source | exact |
| `no_fliers(flag)` | `showfliers(!flag)` | same source | old inverted name unsupported |
| `patch_artist` | `patch_artist` | same source | exact |
| `horizontal(flag)` | `orientation(Orientation)` | same source | canonical option, typed adaptation |
| `BarStyle::{color,width,label}` | `BarOptions::{color,width,label}` | `../matplotlib/lib/matplotlib/axes/_axes.py:2308-2383` | exact |
| `BarStyle::baseline` | `BarOptions::bottom` | `../matplotlib/lib/matplotlib/axes/_axes.py:2313-2315,2349-2350` | old name unsupported |
| `HistStyle::{bins,color,label}` | `HistOptions::{bins,color,label}` | `../matplotlib/lib/matplotlib/axes/_axes.py:7260-7263,7299-7316` | exact; `bins` may remain a finite typed subset |
| implicit vertical histogram | `HistOptions::orientation(Orientation)` | signature `../matplotlib/lib/matplotlib/axes/_axes.py:7260-7263` | introduce canonical name because orientation is an advertised release term |
| `FillBetweenStyle::{color,alpha,label}` | `FillBetweenOptions::{color,alpha,label}` | `fill_between` and forwarded polygon properties: `../matplotlib/lib/matplotlib/axes/_axes.py:6180-6192,6137-6151` | exact option names |
| `ImageStyle::extent` | `ImshowOptions::extent` | `../matplotlib/lib/matplotlib/axes/_axes.py:6212-6216,6353-6355` | exact |
| `ImageStyle::colormap` | `ImshowOptions::cmap` | `../matplotlib/lib/matplotlib/axes/_axes.py:6212-6223` | old name unsupported; typed value |
| `ImageStyle::normalize` | `ImshowOptions::norm` | same source | old name unsupported; typed value |
| `ImageStyle::colorbar(bool)` | `AxesBuilder::colorbar(ColorbarOptions)` queued against the preceding mappable | canonical owner/signature is `Figure.colorbar`: `../matplotlib/lib/matplotlib/figure.py:1202-1243` | Adapted ownership because artist handles are out of scope; old boolean option unsupported |
| `ContourStyle::{extent,levels}` | `ContourOptions::{extent,levels}` | contour signature/delegation `../matplotlib/lib/matplotlib/axes/_axes.py:7202-7216` | exact option names |
| `line_color` | `ContourOptions::colors` | contour keyword vocabulary is consumed by `QuadContourSet`; exact supported subset must be sourced in the later semantic matrix | canonical plural; old name unsupported |
| `colormap`, `normalize` | `ContourOptions::cmap`, `ContourOptions::norm` | same contour/colormapping vocabulary | canonical names, typed adaptation |
| `ContourStyle::colorbar(bool)` | `AxesBuilder::colorbar(ColorbarOptions)` | `../matplotlib/lib/matplotlib/figure.py:1202-1243` | same queued ownership adaptation as imshow |
| `TextStyle::{color,fontsize}` | `TextOptions::{color,fontsize}` | text accepts Text properties: `../matplotlib/lib/matplotlib/axes/_axes.py:654-701` | exact option names |
| every option type's `new()` | same target option type `new()`/`Default` | current constructors throughout `src/series.rs:59-60,143-145,223-225,283-285,334-336,387-389,453-455,529-531` | Adapted Rust builder convention, not a Matplotlib row |

### Return-value adaptation inventory

Mutable Artist/container handles are explicitly out of scope. Therefore every queued `AxesBuilder` plotting/configuration method returns `&mut AxesBuilder` and every `FigureBuilder` method returns `FigureBuilder`; each such return is an `Adapted` row in the checked-in matrix, never `Supported` by implication.

| Canonical method | Matplotlib return | Rust target return |
|---|---|---|
| `Figure.add_subplot` | `Axes` (`../matplotlib/lib/matplotlib/figure.py:708-714`) | queued closure; `FigureBuilder` |
| `Figure.savefig` | effectful save | `Result<()>` |
| `Axes.plot` | `list[Line2D]` (`../matplotlib/lib/matplotlib/axes/_axes.py:1670-1673`) | `&mut AxesBuilder` |
| `Axes.boxplot` | component dictionary (`../matplotlib/lib/matplotlib/axes/_axes.py:4531-4551`) | `&mut AxesBuilder` |
| `Axes.bar` | `BarContainer` (`../matplotlib/lib/matplotlib/axes/_axes.py:2364-2367`) | `&mut AxesBuilder` |
| `Axes.hist` | counts, bins, patches tuple (`../matplotlib/lib/matplotlib/axes/_axes.py:7412-7431`) | `&mut AxesBuilder` |
| `Axes.fill_between` | `FillBetweenPolyCollection` (`../matplotlib/lib/matplotlib/axes/_axes.py:6137-6140`) | `&mut AxesBuilder` |
| `Axes.imshow` | `AxesImage` (`../matplotlib/lib/matplotlib/axes/_axes.py:6394-6405`) | `&mut AxesBuilder` |
| `Axes.contour` | `QuadContourSet` from implementation (`../matplotlib/lib/matplotlib/axes/_axes.py:7202-7216`) | `&mut AxesBuilder` |
| `Axes.text` | `Text` (`../matplotlib/lib/matplotlib/axes/_axes.py:684-687`) | `&mut AxesBuilder` |
| `set_title`, `set_xlabel`, `set_ylabel`, `set_xlim`, `set_ylim`, `set_xticks`, `set_yticks`, `legend`, `colorbar` | Matplotlib Text/limits/Tick/Legend/Colorbar values as documented at the method sources above | `&mut AxesBuilder` (or queued figure command for colorbar) |
| `FigureBuilder::build` | no Matplotlib analogue | `Result<Figure>`; Rust-only `Adapted` construction boundary and the validation point |

## Required checked-in matrix schema

Use one row per atomic claim. A method, each option, its default, its return adaptation, and each unsupported boundary are separate rows. This prevents a method marked `Supported` from hiding an adapted default or discarded return value.

| Column | Required content |
|---|---|
| `id` | Stable kebab-case ID, e.g. `axes-plot-option-linewidth` |
| `family` | `figure`, `layout`, `axes`, `plot`, `boxplot`, `bar`, `hist`, `fill_between`, `imshow`, `contour`, `text`, `legend`, `colorbar`, `ticks`, `scale`, `color`, or `export` |
| `element_kind` | Exactly one of `type`, `method`, `option`, `default`, `return`, `validation`, `continuation`, `boundary` |
| `rust_path` | Exact public Rust path/signature; `—` for an intentionally absent boundary |
| `matplotlib_path` | Exact OO owner and member/parameter, e.g. `matplotlib.axes.Axes.plot(linewidth=...)` |
| `parent_id` | ID of the method/type row this atomic row refines; blank only for roots |
| `status` | Exactly `Supported`, `Adapted`, or `Unsupported` using definitions below |
| `adaptation` | Mandatory concise description for `Adapted`; mandatory `—` for `Supported`; reason/boundary for `Unsupported` |
| `matplotlib_default` | Literal/default source expression or `N/A`; never “same” |
| `rust_default` | Literal/typed default or `N/A`; never “same” |
| `default_status` | `Supported`, `Adapted`, `Unsupported`, or `N/A`; used only on `default` rows |
| `matplotlib_return` | Documented Matplotlib return or `N/A` |
| `rust_return` | Exact Rust return or `N/A` |
| `return_status` | `Supported`, `Adapted`, `Unsupported`, or `N/A`; used only on `return` rows |
| `validation_phase` | `call`, `build`, `savefig`, or `N/A` |
| `invalid_input` | Fatal error, deterministic continuation, or `N/A`; must not say “clamped/ignored” without an explicit compatibility decision |
| `mpl_source` | Pinned commit-relative path plus line span for every inspected in-scope row |
| `rust_source` | Repository-relative path plus line span for implemented rows; planned file/path for unimplemented rows |
| `fixture_or_test` | Stable fixture/test IDs proving the claim, or `TODO(<ticket>)` before implementation |
| `notes` | Narrow caveat only; must not carry unclassified behavior |

Classification rules:

- `Supported`: the public canonical name and the finite claimed behavior/default/return represented by that row match the pinned source. A matching name alone is insufficient.
- `Adapted`: the same finite user-visible capability is present but Rust typing, ownership, builder timing, input shape, validation timing, or return shape necessarily differs. The `adaptation` cell must state exactly how.
- `Unsupported`: the capability or representation is absent from the finite release. The row must name the unsupported boundary and, for removed mplot API, the migration replacement or “none”.

Matrix invariants:

1. No status inheritance: parent method status does not classify its options, defaults, returns, validation, or continuation.
2. No source-free claims: every inspected in-scope row cites the pinned Matplotlib path/lines; every implemented Rust row cites Rust path/lines.
3. No ambiguous partial support: enumerate typed enum variants and accepted input shapes as child rows; unlisted Matplotlib overloads/kwargs get explicit `Unsupported` boundary rows.
4. No silent behavior: clamping, dropping mismatched lengths, default substitution after parse failure, and ignored layout calls each require a `continuation` or `validation` row.
5. No Artist fiction: discarded Matplotlib Artist/container returns are `Adapted` return rows with the exact replacement, normally `&mut AxesBuilder`.
6. The matrix header records Matplotlib commit `bde111fb4ee0023852b48d48f14436647400663d`; source links are commit-relative and ordinary CI does not resolve moving web documentation.

## Decision

Adopt Matplotlib OO nouns and spellings (`Axes`, `add_subplot`, `plot`, `hist`, `imshow`, `set_*`, `savefig`, `linewidth`, `linestyle`, `bottom`, `whis`, `orientation`, `cmap`, `norm`) while representing calls as retained Rust builders and typed method-option builders. Remove old aliases and accidental public internals. The compatibility matrix is atomic by type/method/option/default/return/validation/continuation/boundary, so `Supported`, `Adapted`, and `Unsupported` remain auditable rather than aspirational.
