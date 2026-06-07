# mplot

Native Rust plotting with a [plotpy](https://github.com/cpmech/plotpy)-compatible API.

[plotpy](https://github.com/cpmech/plotpy) builds Python/Matplotlib scripts and shells out to `python3`. **mplot renders figures directly in Rust** (PNG today) while keeping the same builder-style workflow: configure `Curve` / `Boxplot` objects, attach them to a `Plot`, then call `save`.

## Quick start

```rust
use mplot::{Curve, Plot, StrError};

fn main() -> Result<(), StrError> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [0.0, 1.0, 4.0, 9.0, 16.0];

    let mut curve = Curve::new();
    curve.set_line_color("#1f77b4").set_label("y = x²");
    curve.draw(&x, &y);

    let mut plot = Plot::new();
    plot.add(&curve)
        .set_title("Simple line plot")
        .set_labels("x", "y");

    plot.save("line.png")?;
    Ok(())
}
```

Run the bundled examples:

```bash
cargo run --example simple_line
cargo run --example subplot_lines
cargo run --example boxplot_linear
cargo run --example boxplot_log
cargo run --example gallery
```

## API overview

| Type | Role |
|------|------|
| `Plot` | Figure driver: subplots, axes, labels, scales, `save` |
| `Curve` | Line/scatter data via `draw` |
| `Boxplot` | Box-and-whisker groups via `draw` |
| `GraphMaker` | Trait implemented by drawable entities |

Typical flow matches plotpy:

1. Build and configure a graph entity (`Curve`, `Boxplot`, …).
2. Call `draw` with your data.
3. `plot.set_subplot(...).add(&entity).set_labels(...)` (chain axis options as needed).
4. `plot.save("figure.png")`.

## Features

- Subplot grids (`set_subplot`)
- Linear and log *y* scales (`set_log_y`)
- Custom tick labels (`set_ticks_x_labels`)
- Figure size in inches (`set_figure_size_inches`)
- Matplotlib-inspired default styling (DejaVu Sans, default line width, boxplot colors)

## Comparison with plotpy

| | plotpy | mplot |
|---|--------|-------|
| Backend | Python + Matplotlib | Rust + plotters |
| Output | PNG/PDF/SVG via `python3` | PNG via `Plot::save` |
| Runtime deps | Python, Matplotlib | None beyond the Rust binary |
| API | `Plot`, `Curve`, `Boxplot`, … | Same names and patterns (MVP subset) |

Optional: `scripts/mpl_reference.py` generates Matplotlib reference PNGs under `tests/fidelity/golden/`. Run `cargo test --test fidelity` to compare mplot output against those references.

## Status

Early MVP. See `examples/` for supported plot types. HTML display, 3D, bar/histogram traces, and additional export formats are not implemented yet.
