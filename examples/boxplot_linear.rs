use mplot::prelude::*;

fn main() -> Result<()> {
    let groups = vec![
        vec![1.2, 1.5, 1.8, 2.0, 2.1],
        vec![2.0, 2.3, 2.5, 2.8, 3.0, 3.2],
        vec![0.8, 1.0, 1.1, 1.4],
    ];
    let ticks = [1, 2, 3];
    let labels = ["Group A\n(n=5)", "Group B\n(n=6)", "Group C\n(n=4)"];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.boxplot(&groups, BoxplotStyle::new())
                .axes(
                    AxesStyle::new()
                        .title("Boxplot (linear y)")
                        .x_label("category")
                        .y_label("value")
                        .x_tick_labels(&ticks, &labels),
                );
        })
        .build()?;

    figure.save("/tmp/mplot/boxplot_linear.png", SaveOptions::default())?;
    println!("wrote /tmp/mplot/boxplot_linear.png");
    Ok(())
}
