//! Write a small set of generic figures under `/tmp/mplot/gallery/`.

use mplot::{Boxplot, Curve, Plot, StrError};

fn main() -> Result<(), StrError> {
    std::fs::create_dir_all("/tmp/mplot/gallery").ok();

    write_line_plot("/tmp/mplot/gallery/line.png")?;
    write_subplot_grid("/tmp/mplot/gallery/subplots.png")?;
    write_boxplot("/tmp/mplot/gallery/boxplot.png")?;

    println!("wrote figures under /tmp/mplot/gallery/");
    Ok(())
}

fn write_line_plot(path: &str) -> Result<(), StrError> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [1.0, 1.4, 1.8, 2.2, 2.6, 3.0];

    let mut curve = Curve::new();
    curve.set_line_color("#2ca02c");
    curve.draw(&x, &y);

    let mut plot = Plot::new();
    plot.add(&curve)
        .set_title("Line plot")
        .set_labels("x", "f(x)");

    plot.save(path)
}

fn write_subplot_grid(path: &str) -> Result<(), StrError> {
    let xs = [0.0, 2.0, 4.0, 6.0, 8.0];
    let mut plot = Plot::new();
    plot.set_figure_size_inches(10.0, 4.0);

    for (index, (title, color, ys)) in [
        ("Sine-ish", "#1f77b4", [0.0, 0.9, 1.4, 1.2, 0.8]),
        ("Ramp", "#ff7f0e", [0.0, 0.5, 1.0, 1.5, 2.0]),
    ]
    .iter()
    .enumerate()
    {
        let mut curve = Curve::new();
        curve.set_line_color(color);
        curve.draw(&xs, ys);

        plot.set_subplot(1, 2, index + 1)
            .add(&curve)
            .set_title(title)
            .set_labels("x", "y");
    }

    plot.save(path)
}

fn write_boxplot(path: &str) -> Result<(), StrError> {
    let groups = vec![vec![2.0, 3.0, 4.0, 5.0], vec![4.0, 5.0, 6.0, 7.0, 8.0]];
    let ticks = [1, 2];
    let labels = ["Low", "High"];

    let mut boxes = Boxplot::new();
    boxes.draw(&groups);

    let mut plot = Plot::new();
    plot.add(&boxes)
        .set_title("Two-group boxplot")
        .set_labels("group", "measurement")
        .set_ticks_x_labels(&ticks, &labels);

    plot.save(path)
}
