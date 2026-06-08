use mplot::prelude::*;
use std::path::Path;

fn main() -> Result<()> {
    let x = [1.0, 2.0, 3.0, 4.0];
    let heights = [2.0, 5.0, 3.0, 7.0];
    let data = [1.0, 1.2, 1.8, 2.0, 2.1, 2.5, 3.0, 3.2, 3.8, 4.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.bar(&x, &heights, BarStyle::new().color(Color::TABLEAU[0]))
                .axes(AxesStyle::new().title("Bar chart").x_label("x").y_label("height"));
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.histogram(
                &data,
                HistStyle::new().bins(6).color(Color::TABLEAU[1]),
            )
            .axes(AxesStyle::new().title("Histogram").x_label("value").y_label("count"));
        })
        .build()?;

    let path = Path::new("/tmp/mplot/bar_histogram.png");
    figure.save(path, SaveOptions::default())?;
    println!("saved {}", path.display());
    Ok(())
}
