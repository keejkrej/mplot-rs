use mplot::prelude::*;

fn main() -> Result<()> {
    let rows = 2;
    let cols = 2;
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let panels: [(&str, Color, [f64; 5]); 4] = [
        ("Panel A", Color::TABLEAU[0], [0.0, 0.5, 1.5, 2.0, 2.5]),
        ("Panel B", Color::TABLEAU[1], [0.0, 0.8, 1.2, 1.8, 2.2]),
        ("Panel C", Color::TABLEAU[2], [0.0, 0.3, 1.0, 1.4, 1.9]),
        ("Panel D", Color::TABLEAU[3], [0.0, 0.6, 1.1, 1.6, 2.4]),
    ];

    let mut builder = Figure::builder().size(Size::inches(10.0, 8.0));
    for (index, (title, color, ys)) in panels.iter().enumerate() {
        builder = builder.panel(GridPos::new(rows, cols, index + 1), |p| {
            p.line(&xs, ys, LineStyle::new().color(*color))
                .axes(
                    AxesStyle::new()
                        .title(*title)
                        .x_label("x")
                        .y_label("y")
                        .y_range(0.0, 3.0),
                );
        });
    }

    let figure = builder.build()?;
    figure.save("/tmp/mplot/subplot_lines.png", SaveOptions::default())?;
    println!("wrote /tmp/mplot/subplot_lines.png");
    Ok(())
}
