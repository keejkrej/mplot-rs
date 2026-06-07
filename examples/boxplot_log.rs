use mplot::prelude::*;

fn main() -> Result<()> {
    let groups = vec![
        vec![12.0, 18.0, 25.0, 31.0],
        vec![8.0, 15.0, 22.0, 29.0, 35.0],
        vec![10.0, 20.0, 40.0, 55.0],
    ];
    let ticks = [1, 2, 3];
    let labels = ["Group A\n(n=4)", "Group B\n(n=5)", "Group C\n(n=4)"];

    let figure = Figure::builder()
        .size(Size::inches(8.0, 6.0))
        .panel(GridPos::new(1, 1, 1), |p| {
            p.boxplot(&groups, BoxplotStyle::new())
                .axes(
                    AxesStyle::new()
                        .title("Boxplot (log y)")
                        .x_label("category")
                        .y_label("value")
                        .x_tick_labels(&ticks, &labels)
                        .y_scale(Scale::Log),
                );
        })
        .build()?;

    figure.save("/tmp/mplot/boxplot_log.png", SaveOptions::default())?;
    println!("wrote /tmp/mplot/boxplot_log.png");
    Ok(())
}
