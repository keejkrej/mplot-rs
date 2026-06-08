use mplot::prelude::*;
use std::path::Path;

fn main() -> Result<()> {
    let x = [1.0, 10.0, 100.0, 1000.0];
    let y = [1.0, 2.0, 3.0, 4.0];
    let x_log = [1.0, 10.0, 100.0];
    let y_log = [10.0, 100.0, 1000.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.line(&x, &y, LineStyle::new().color(Color::TABLEAU[0]))
                .axes(
                    AxesStyle::new()
                        .title("Log x")
                        .x_label("x")
                        .y_label("y")
                        .x_scale(Scale::Log),
                );
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.line(&x_log, &y_log, LineStyle::new().color(Color::TABLEAU[1]))
                .axes(
                    AxesStyle::new()
                        .title("Log-log")
                        .x_label("x")
                        .y_label("y")
                        .x_scale(Scale::Log)
                        .y_scale(Scale::Log),
                );
        })
        .build()?;

    let path = Path::new("/tmp/mplot/line_log_x.png");
    figure.save(path, SaveOptions::default())?;
    println!("saved {}", path.display());
    Ok(())
}
