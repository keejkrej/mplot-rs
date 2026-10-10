use mplot::prelude::*;

fn main() -> Result<()> {
    let groups = vec![
        vec![1.2, 1.5, 1.8, 2.0, 2.1, 2.2, 2.5, 3.0],
        vec![2.0, 2.3, 2.5, 2.8, 3.0, 3.2, 3.5, 3.8, 4.0],
        vec![0.8, 1.0, 1.1, 1.4, 1.5, 1.7, 1.9],
    ];
    let ticks = [1, 2, 3];
    let labels = ["Group A", "Group B", "Group C"];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.violinplot(
                &groups,
                ViolinStyle::new()
                    .show_means(true)
                    .show_medians(true)
                    .show_extrema(true),
            )
            .axes(
                AxesStyle::new()
                    .title("Violin Plot")
                    .x_label("category")
                    .y_label("value")
                    .x_tick_labels(&ticks, &labels),
            );
        })
        .build()?;

    figure.save("/tmp/mplot/violin_plot.png", SaveOptions::default())?;
    println!("wrote /tmp/mplot/violin_plot.png");
    Ok(())
}
