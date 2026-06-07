use mplot::{Boxplot, Plot, StrError};

fn main() -> Result<(), StrError> {
    let groups = vec![
        vec![1.2, 1.5, 1.8, 2.0, 2.1],
        vec![2.0, 2.3, 2.5, 2.8, 3.0, 3.2],
        vec![0.8, 1.0, 1.1, 1.4],
    ];
    let ticks = [1, 2, 3];
    let labels = ["Group A\n(n=5)", "Group B\n(n=6)", "Group C\n(n=4)"];

    let mut boxes = Boxplot::new();
    boxes.draw(&groups);

    let mut plot = Plot::new();
    plot.add(&boxes)
        .set_title("Boxplot (linear y)")
        .set_labels("category", "value")
        .set_ticks_x_labels(&ticks, &labels);

    plot.save("/tmp/mplot/boxplot_linear.png")?;
    println!("wrote /tmp/mplot/boxplot_linear.png");
    Ok(())
}
