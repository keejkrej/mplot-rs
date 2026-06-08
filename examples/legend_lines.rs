use mplot::prelude::*;

fn main() -> Result<()> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y1 = [0.0, 1.0, 2.0, 2.5, 3.0];
    let y2 = [3.0, 2.5, 2.0, 1.0, 0.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(
                &x,
                &y1,
                LineStyle::new()
                    .color(Color::TABLEAU[0])
                    .label("series A")
                    .marker(Marker::Circle),
            )
            .line(
                &x,
                &y2,
                LineStyle::new()
                    .color(Color::TABLEAU[1])
                    .dash(LineDash::Dashed)
                    .label("series B"),
            )
            .axes(
                AxesStyle::new()
                    .title("Legend example")
                    .x_label("x")
                    .y_label("y")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    figure.save("/tmp/mplot/legend_lines.png", SaveOptions::default())?;
    println!("wrote /tmp/mplot/legend_lines.png");
    Ok(())
}
