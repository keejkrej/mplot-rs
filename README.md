# mplot

[![Crates.io](https://img.shields.io/crates/v/mplot.svg)](https://crates.io/crates/mplot)
[![Documentation](https://docs.rs/mplot/badge.svg)](https://docs.rs/mplot)
[![CI](https://github.com/keejkrej/mplot-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/keejkrej/mplot-rs/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

Native Rust 2D plotting with matplotlib-inspired rendering fidelity.

**mplot** renders figures directly in Rust (PNG, SVG, and PDF) using an idiomatic builder API: configure panels with typed styles, add series, then export.

---

## Installation

Add `mplot` to your `Cargo.toml`:

```toml
[dependencies]
mplot = "1.0.0"
```

---

## Quick start

```rust
use mplot::prelude::*;

fn main() -> mplot::Result<()> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [0.0, 1.0, 4.0, 9.0, 16.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(
                &x,
                &y,
                LineStyle::new()
                    .color(Color::hex("#1f77b4"))
                    .label("y = x²"),
            )
            .axes(
                AxesStyle::new()
                    .title("Simple line plot")
                    .x_label("x")
                    .y_label("y"),
            );
        })
        .build()?;

    figure.save("line.png", SaveOptions::default())?;
    Ok(())
}
```

Run the bundled examples:

```bash
cargo run --example line_panel
cargo run --example simple_line
cargo run --example subplot_lines
cargo run --example boxplot_linear
cargo run --example boxplot_log
cargo run --example scatter_errorbar
cargo run --example bar_histogram
cargo run --example violin_plot
cargo run --example gridspec_layout
cargo run --example advanced_plots
cargo run --example gallery
```

---

## API Overview

| Type | Role |
|------|------|
| `Figure` / `FigureBuilder` | Root document; size, panels, export |
| `PanelBuilder` | One subplot: add series (`line`, `scatter`, `errorbar`, `step`, `stem`, `stackplot`, `stairs`, `quiver`, `pie`, `hist2d`, `hlines`, `vlines`, `eventplot`, `boxplot`, `violinplot`, `bar`, `barh`, `histogram`, `fill_between`, `fill_betweenx`, `image`, `contour`, `annotate`, `axline`, `axhline`, `axvline`, `axhspan`, `axvspan`) + attach `AxesStyle` |
| `Series` | Enum of plot kinds (`Line`, `Scatter`, `ErrorBar`, `Step`, `Stem`, `Stackplot`, `Stairs`, `Quiver`, `Pie`, `Hist2D`, `HLines`, `VLines`, `EventPlot`, `Boxplot`, `Violin`, `Bar`, `BarH`, `Annotation`, …) |
| `LineStyle`, `ScatterStyle`, `StemStyle`, `StackplotStyle`, `StairsStyle`, `QuiverStyle`, `PieStyle`, `Hist2DStyle`, `EventplotStyle`, `BboxStyle`, … | Per-series styling |
| `AxesStyle` | Labels, scales, limits, inverted axes, ticks, percentage & engineering formatting, spines control / `despine()`, legend positioning & columns, grid (`GridStyle`) |
| `GridPos` | Subplot address (rows, cols, index) |
| `GridSpec` / `SubplotSlot` | Irregular grids with rowspan/colspan |
| `ExportFormat` | PNG, SVG, or PDF export (from path extension or `SaveOptions`) |
| `Color`, `Scale`, `LineDash`, `Marker`, `Colormap` | Typed styling enums |
| `SaveOptions` | dpi (default 200), tight bbox, pad |

Typical flow:

1. `Figure::builder()` — set figure size and gaps if needed.
2. `.panel(GridPos::new(r, c, i), |p| { … })` or `.panel(GridSpec::new(r, c).at(row, col), |p| { … })` — add series and axes style per subplot.
3. `.build()?` then `figure.save(path, SaveOptions::…)?` — format inferred from `.png`, `.svg`, or `.pdf` extension.

Import everything common via `use mplot::prelude::*;`.

---

## Features

- **Subplot layouts**: Grids via `GridPos` or `GridSpec` (arbitrary rowspan/colspan)
- **Comprehensive series**: Line, scatter (variable sizes & colormapped values), error bars (with caps), stem plots (vertical and horizontal), stacked area plots (`stackplot`), step curves, stairs plots, 2D vector fields (`quiver`), pie charts (with explosion and autopct), 2D histograms (`hist2d`), horizontal and vertical line segments (`hlines`, `vlines`), event/raster plots (`eventplot`), vertical/horizontal stacked bars (`bottom` and `left`), bar error bars (`yerr` and `xerr`), histograms, shaded regions (`fill_between`, `fill_betweenx`, `axhspan`, `axvspan`), annotations with callout arrows and bounding boxes (`annotate`, `BboxStyle`), reference lines (`axline`, `axhline`, `axvline`), boxplots, violin plots (KDE density, medians, extrema), heatmaps (`image`), and contour lines
- **Axes spine control**: Visibility & styling (`AxesStyle::despine()` to hide top/right spines, or `spines_visible(top, right, bottom, left)`)
- **Legends**: Placement (`LegendLoc::UpperRight`, `LowerLeft`, `Center`, etc.), `ncols`, `title`, and `frameon`
- **Tick formatting**: `TickFormat::Percent`, `TickFormat::Engineering`, scientific, custom closures
- **Inverted axes**: Reverse ranges (`invert_x(true)`, `invert_y(true)`, or `y_range(max, min)`)
- **Grid styling**: `GridStyle::new().color(...).width(...).alpha(...)`
- **Export formats**: High-resolution PNG, SVG vector graphics, and vector PDF (`figure.save("out.pdf", …)`)
- **Layout management**: Constrained layout for multi-panel figures and colorbar insets (`.constrained_layout(true)`)
- **Scales**: Linear and logarithmic x/y scales (`Scale::Log` on `AxesStyle::x_scale` / `y_scale`)
- **Tick rotation & labels**: Custom tick labels and rotated tick labels (`AxesStyle::x_tick_labels`, `AxesStyle::x_tick_rotation`)
- **Standard markers**: `Circle`, `Square`, `Cross`, `Plus`, `TriangleUp`, `TriangleDown`, `TriangleLeft`, `TriangleRight`, `Diamond`, `Star`, `Pentagon`, `Hexagon`, `Point`, `VLine`, `HLine`
- **Perceptual colormaps**: `Viridis`, `Plasma`, `Inferno`, `Magma`, `Cividis`, `Coolwarm`, `Turbo`, `Greys`, `Blues`, `Reds`, `Spectral`, `RdBu`, `YlGnBu`, and reversed variants (`.reversed()`)
- **Matplotlib defaults**: DejaVu Sans font, standard line widths, boxplot geometry, axes titles centered on axes box

---

## Testing & Parity

Run unit tests, integration tests, and doctests:

```bash
cargo test
```

### Fidelity Test Suite

Run fidelity tests comparing rendering against golden references:

```bash
cargo test --test fidelity
```

---

## License

Dual-licensed under either:
- MIT License ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))

at your option.
