use mplot::{Curve, Plot, StrError};

fn main() -> Result<(), StrError> {
    let rows = 2;
    let cols = 3;
    let mut plot = Plot::new();
    plot.set_figure_size_inches(12.0, 8.0);

    let xs = &[0.0, 5.0, 10.0, 15.0, 20.0];
    let ys_a = &[1.0, 2.0, 1.5, 2.5, 2.0];
    let ys_b = &[0.5, 1.2, 1.0, 1.8, 1.4];

    for index in 1..=4 {
        let mut curve_a = Curve::new();
        curve_a.set_line_color("green").set_line_alpha(0.1);
        curve_a.draw(xs, ys_a);

        let mut curve_b = Curve::new();
        curve_b.set_line_color("green").set_line_alpha(0.1);
        curve_b.draw(xs, ys_b);

        plot.set_subplot(rows, cols, index)
            .add(&curve_a)
            .add(&curve_b)
            .set_title(&format!("slide channel {index} (2 traces)"))
            .grid_and_labels("minutes", "corrected intensity")
            .set_yrange(0.0, 3.0);
    }

    for index in 5..=(rows * cols) {
        plot.set_subplot(rows, cols, index).set_hide_axes(true);
    }

    plot.save("/tmp/mplot/timeseries_grid.png")?;
    println!("wrote /tmp/mplot/timeseries_grid.png");
    Ok(())
}
