# mplot

Native Rust plotting with a [plotpy](https://github.com/cpmech/plotpy)-compatible API. Where plotpy generates Python/Matplotlib scripts, **mplot renders PNGs directly** using [plotters](https://docs.rs/plotters).

## Example

```rust
use mplot::{Curve, Plot, StrError};

fn main() -> Result<(), StrError> {
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, 1.0, 4.0, 9.0];

    let mut curve = Curve::new();
    curve.set_line_color("green").set_line_alpha(0.8);
    curve.draw(&x, &y);

    let mut plot = Plot::new();
    plot.set_figure_size_inches(12.0, 8.0)
        .add(&curve)
        .set_title("example")
        .grid_and_labels("minutes", "intensity");

    plot.save("/tmp/mplot/example.png")?;
    Ok(())
}
```

## MVP scope

- `Plot`, `Curve`, `Boxplot`, `GraphMaker`
- Subplots via `set_subplot`
- Linear and log *y* axes
- PNG export via `Plot::save`

See `examples/` for transfection-style timeseries grids and box plots.
