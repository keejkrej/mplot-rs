//! Write a small set of generic figures under `/tmp/mplot/gallery/`.

use mplot::prelude::*;

fn main() -> Result<()> {
    std::fs::create_dir_all("/tmp/mplot/gallery").ok();

    write_line_plot("/tmp/mplot/gallery/line.png")?;
    write_subplot_grid("/tmp/mplot/gallery/subplots.png")?;
    write_boxplot("/tmp/mplot/gallery/boxplot.png")?;

    println!("wrote figures under /tmp/mplot/gallery/");
    Ok(())
}

fn write_line_plot(path: &str) -> Result<()> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [1.0, 1.4, 1.8, 2.2, 2.6, 3.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(&x, &y, LineStyle::new().color(Color::TABLEAU[2]))
                .axes(
                    AxesStyle::new()
                        .title("Line plot")
                        .x_label("x")
                        .y_label("f(x)"),
                );
        })
        .build()?;

    figure.save(path, SaveOptions::default())
}

fn write_subplot_grid(path: &str) -> Result<()> {
    let xs = [0.0, 2.0, 4.0, 6.0, 8.0];
    let panels = [
        ("Sine-ish", Color::TABLEAU[0], [0.0, 0.9, 1.4, 1.2, 0.8]),
        ("Ramp", Color::TABLEAU[1], [0.0, 0.5, 1.0, 1.5, 2.0]),
    ];

    let mut builder = Figure::builder().size(Size::inches(10.0, 4.0));
    for (index, (title, color, ys)) in panels.iter().enumerate() {
        builder = builder.panel(GridPos::new(1, 2, index + 1), |p| {
            p.line(&xs, ys, LineStyle::new().color(*color))
                .axes(
                    AxesStyle::new()
                        .title(*title)
                        .x_label("x")
                        .y_label("y"),
                );
        });
    }

    builder.build()?.save(path, SaveOptions::default())
}

fn write_boxplot(path: &str) -> Result<()> {
    let groups = vec![vec![2.0, 3.0, 4.0, 5.0], vec![4.0, 5.0, 6.0, 7.0, 8.0]];
    let ticks = [1, 2];
    let labels = ["Low", "High"];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.boxplot(&groups, BoxplotStyle::new())
                .axes(
                    AxesStyle::new()
                        .title("Two-group boxplot")
                        .x_label("group")
                        .y_label("measurement")
                        .x_tick_labels(&ticks, &labels),
                );
        })
        .build()?
        .save(path, SaveOptions::default())
}
