use mplot::{Boxplot, Plot, StrError};

fn main() -> Result<(), StrError> {
    let grouped_values = vec![
        vec![12.0, 18.0, 25.0, 31.0],
        vec![8.0, 15.0, 22.0, 29.0, 35.0],
        vec![10.0, 20.0, 40.0],
    ];
    let slide_channels = [1, 2, 3];
    let trace_counts = [4, 5, 3];
    let labels: Vec<String> = slide_channels
        .iter()
        .zip(trace_counts.iter())
        .map(|(channel, count)| format!("channel {channel}\n(n={count})"))
        .collect();

    let mut boxes = Boxplot::new();
    boxes.draw(&grouped_values);

    let mut plot = Plot::new();
    plot.set_figure_size_inches(12.0, 8.0)
        .add(&boxes)
        .set_title("AUC")
        .set_labels("condition", "AUC")
        .set_ticks_x_labels(&slide_channels, &labels)
        .set_log_y(true);

    plot.save("/tmp/mplot/auc_boxplot.png")?;
    println!("wrote /tmp/mplot/auc_boxplot.png");
    Ok(())
}
