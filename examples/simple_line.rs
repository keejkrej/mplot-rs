use mplot::{Curve, Plot, StrError};

fn main() -> Result<(), StrError> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [0.0, 1.0, 4.0, 9.0, 16.0];

    let mut curve = Curve::new();
    curve.set_line_color("#1f77b4").set_label("y = x²");
    curve.draw(&x, &y);

    let mut plot = Plot::new();
    plot.add(&curve)
        .set_title("Simple line plot")
        .set_labels("x", "y");

    plot.save("/tmp/mplot/simple_line.png")?;
    println!("wrote /tmp/mplot/simple_line.png");
    Ok(())
}
