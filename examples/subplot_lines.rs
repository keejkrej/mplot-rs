use mplot::{Curve, Plot, StrError};

fn main() -> Result<(), StrError> {
    let rows = 2;
    let cols = 2;
    let mut plot = Plot::new();
    plot.set_figure_size_inches(10.0, 8.0);

    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let panels: [(&str, &str, [f64; 5]); 4] = [
        ("Panel A", "#1f77b4", [0.0, 0.5, 1.5, 2.0, 2.5]),
        ("Panel B", "#ff7f0e", [0.0, 0.8, 1.2, 1.8, 2.2]),
        ("Panel C", "#2ca02c", [0.0, 0.3, 1.0, 1.4, 1.9]),
        ("Panel D", "#d62728", [0.0, 0.6, 1.1, 1.6, 2.4]),
    ];

    for (index, (title, color, ys)) in panels.iter().enumerate() {
        let mut curve = Curve::new();
        curve.set_line_color(color);
        curve.draw(&xs, ys);

        plot.set_subplot(rows, cols, index + 1)
            .add(&curve)
            .set_title(title)
            .set_labels("x", "y")
            .set_yrange(0.0, 3.0);
    }

    plot.save("/tmp/mplot/subplot_lines.png")?;
    println!("wrote /tmp/mplot/subplot_lines.png");
    Ok(())
}
