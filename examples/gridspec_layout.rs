use mplot::prelude::*;
use std::path::Path;

fn main() -> Result<()> {
    let gs = GridSpec::new(2, 2);
    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, 1.0, 4.0, 9.0];

    let figure = Figure::builder()
        .constrained_layout(true)
        .panel(gs.at(0, 0), |p| {
            p.line(&x, &y, LineStyle::new().color(Color::TABLEAU[0]))
                .axes(AxesStyle::new().title("top-left"));
        })
        .panel(gs.at(0, 1), |p| {
            p.line(&x, &y, LineStyle::new().color(Color::TABLEAU[1]))
                .axes(AxesStyle::new().title("top-right"));
        })
        .panel(gs.span(1, 0, 1, 2), |p| {
            p.bar(
                &[1.0, 2.0, 3.0],
                &[2.0, 5.0, 3.0],
                BarStyle::new().color(Color::TABLEAU[2]),
            )
            .axes(AxesStyle::new().title("bottom span").x_label("x").y_label("y"));
        })
        .build()?;

    let png = Path::new("/tmp/mplot/gridspec.png");
    let svg = Path::new("/tmp/mplot/gridspec.svg");
    let pdf = Path::new("/tmp/mplot/gridspec.pdf");
    figure.save(png, SaveOptions::default())?;
    figure.save(svg, SaveOptions::default())?;
    figure.save(pdf, SaveOptions::default())?;
    println!(
        "saved {}, {}, and {}",
        png.display(),
        svg.display(),
        pdf.display()
    );
    Ok(())
}
