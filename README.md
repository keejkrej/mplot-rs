# mplot

Native Rust 2D plotting with matplotlib-inspired rendering fidelity.

**mplot** renders figures directly in Rust (PNG, SVG, and PDF) using an idiomatic builder API: configure panels with typed styles, add series, then export.

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
cargo run --example gallery
```

## API overview

| Type | Role |
|------|------|
| `Figure` / `FigureBuilder` | Root document; size, panels, export |
| `PanelBuilder` | One subplot: add series + attach `AxesStyle` |
| `Series` | Enum of plot kinds (`Line`, `Boxplot`, …) |
| `LineStyle`, `BoxplotStyle`, … | Per-series styling |
| `AxesStyle` | Labels, scales, limits, ticks, grid |
| `GridPos` | Subplot address (rows, cols, index) |
| `GridSpec` / `SubplotSlot` | Irregular grids with rowspan/colspan |
| `ExportFormat` | PNG, SVG, or PDF export (from path extension or `SaveOptions`) |
| `Color`, `Scale`, `LineDash` | Typed styling enums |
| `SaveOptions` | dpi, tight bbox, pad |

Typical flow:

1. `Figure::builder()` — set figure size and gaps if needed.
2. `.panel(GridPos::new(r, c, i), |p| { … })` or `.panel(GridSpec::new(r, c).at(row, col), |p| { … })` — add series and axes style per subplot.
3. `.build()?` then `figure.save(path, SaveOptions::…)?` — format inferred from `.png`, `.svg`, or `.pdf` extension.

Import everything common via `use mplot::prelude::*;`.

## Features

- Subplot grids via `GridPos` or `GridSpec` (rowspan/colspan)
- PNG, SVG, and PDF export (`figure.save("out.pdf", …)`)
- Constrained layout for multi-panel figures and colorbar insets (`.constrained_layout(true)`)
- Linear and log *y* scales (`Scale::Log`)
- Custom tick labels (`AxesStyle::x_tick_labels`)
- Figure size in inches (`Size::inches`)
- Matplotlib-inspired default styling (DejaVu Sans, default line width, boxplot colors)

## Fidelity tests

`scripts/mpl_reference.py` generates Matplotlib reference PNGs under `tests/fidelity/golden/`. Run `cargo test --test fidelity` to compare mplot output against those references.

## Status

Early development. Line, boxplot, bar, histogram, fill-between, image, and contour series are supported with legend, markers, and colormaps. PNG, SVG, and PDF export plus GridSpec layouts are available.
