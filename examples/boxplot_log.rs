use mplot::{Boxplot, Plot, StrError};

fn main() -> Result<(), StrError> {
    let groups = vec![
        vec![12.0, 18.0, 25.0, 31.0],
        vec![8.0, 15.0, 22.0, 29.0, 35.0],
        vec![10.0, 20.0, 40.0, 55.0],
    ];
    let ticks = [1, 2, 3];
    let labels = ["Group A\n(n=4)", "Group B\n(n=5)", "Group C\n(n=4)"];

    let mut boxes = Boxplot::new();
    boxes.draw(&groups);

    let mut plot = Plot::new();
    plot.set_figure_size_inches(8.0, 6.0)
        .add(&boxes)
        .set_title("Boxplot (log y)")
        .set_labels("category", "value")
        .set_ticks_x_labels(&ticks, &labels)
        .set_log_y(true);

    plot.save("/tmp/mplot/boxplot_log.png")?;
    println!("wrote /tmp/mplot/boxplot_log.png");
    Ok(())
}
