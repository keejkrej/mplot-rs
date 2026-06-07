use mplot::{Boxplot, Plot, StrError};

fn quartile_axis_upper(grouped_values: &[Vec<f64>]) -> f64 {
    let max_q3 = grouped_values
        .iter()
        .filter_map(|values| {
            let mut sorted = values.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            if sorted.is_empty() {
                None
            } else {
                let idx = ((sorted.len() - 1) as f64 * 0.75).round() as usize;
                Some(sorted[idx.min(sorted.len() - 1)])
            }
        })
        .fold(0.0_f64, f64::max);
    let upper = max_q3 * 1.25;
    if upper > 0.0 { upper } else { 1.0 }
}

fn main() -> Result<(), StrError> {
    let grouped_values = vec![
        vec![0.8, 1.1, 1.4, 1.2],
        vec![1.5, 1.7, 1.9, 2.1, 2.0],
        vec![0.9, 1.0, 1.2],
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
        .set_title("intensity offset")
        .set_labels("condition", "intensity offset")
        .set_ticks_x_labels(&slide_channels, &labels)
        .set_yrange(0.0, quartile_axis_upper(&grouped_values));

    plot.save("/tmp/mplot/fit_boxplot.png")?;
    println!("wrote /tmp/mplot/fit_boxplot.png");
    Ok(())
}
