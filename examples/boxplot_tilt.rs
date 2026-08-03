use mplot::prelude::*;

fn main() -> Result<()> {
    let groups = vec![
        vec![1.0, 2.0, 1.5, 2.2, 1.8],
        vec![3.0, 3.5, 2.8, 3.2, 4.0],
        vec![2.0, 2.5, 2.1, 1.9, 2.4],
        vec![4.0, 4.5, 3.8, 5.0, 4.2],
    ];
    let ticks = [1, 2, 3, 4];
    let labels = [
        "sample A (n=5)",
        "sample B long (n=5)",
        "C (n=5)",
        "control vehicle (n=5)",
    ];

    let figure = Figure::builder()
        .size(Size::inches(6.5, 4.5))
        .panel(GridPos::new(1, 1, 1), |p| {
            p.boxplot(&groups, BoxplotStyle::new()).axes(
                AxesStyle::new()
                    .x_label("sample")
                    .y_label("AUC")
                    .x_tick_labels(&ticks, &labels)
                    .x_tick_label_rotation(TickLabelRotation::Degrees(-30))
                    .y_tick_format(TickFormat::Scientific),
            );
        })
        .build()?;

    std::fs::create_dir_all("/tmp/mplot").ok();
    figure.save(
        "/tmp/mplot/boxplot_tilt30.png",
        SaveOptions::default().dpi(100),
    )?;
    println!("wrote /tmp/mplot/boxplot_tilt30.png");
    Ok(())
}
