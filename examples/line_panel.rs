use mplot::prelude::*;

fn main() -> Result<()> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [0.0, 1.0, 4.0, 9.0, 16.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(
                &x,
                &y,
                LineStyle::new()
                    .color(Color::TABLEAU[0])
                    .label("y = x²"),
            )
            .axes(
                AxesStyle::new()
                    .title("Line panel")
                    .x_label("x")
                    .y_label("y"),
            );
        })
        .build()?;

    figure.save("/tmp/mplot/line_panel.png", SaveOptions::default())?;
    println!("wrote /tmp/mplot/line_panel.png");
    Ok(())
}
