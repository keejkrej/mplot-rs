use mplot::prelude::*;

fn main() -> Result<()> {
    std::fs::create_dir_all("/tmp/mplot").ok();

    // 1. Stem Plot
    let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [1.2, -0.8, 2.4, -1.5, 0.9, 1.7];

    let stem_fig = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.stem(
                &x,
                &y,
                StemStyle::new()
                    .color(Color::hex("#1f77b4"))
                    .marker(Marker::Circle)
                    .marker_size(5.0)
                    .label("sampled impulse"),
            )
            .axes(
                AxesStyle::new()
                    .title("Stem Plot")
                    .x_label("time (s)")
                    .y_label("amplitude")
                    .legend(LegendStyle::show().loc(LegendLoc::UpperRight)),
            );
        })
        .build()?;
    stem_fig.save("/tmp/mplot/stem_plot.png", SaveOptions::default())?;

    // 2. Stackplot
    let days = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let series_a = vec![2.0, 3.0, 4.0, 3.0, 2.0];
    let series_b = vec![1.0, 2.0, 1.5, 2.5, 3.0];
    let series_c = vec![0.5, 1.0, 1.2, 0.8, 1.5];

    let stack_fig = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.stackplot(
                &days,
                &[&series_a, &series_b, &series_c],
                StackplotStyle::new()
                    .labels(["Search", "Social", "Direct"])
                    .alpha(0.85),
            )
            .axes(
                AxesStyle::new()
                    .title("Stacked Area Plot")
                    .x_label("Day")
                    .y_label("Active Users")
                    .legend(LegendStyle::show().loc(LegendLoc::UpperLeft)),
            );
        })
        .build()?;
    stack_fig.save("/tmp/mplot/stackplot.png", SaveOptions::default())?;

    // 3. Pie Chart
    let values = [40.0, 30.0, 20.0, 10.0];
    let labels = ["Rust", "Python", "C++", "Other"];
    let explode = [0.08, 0.0, 0.0, 0.0];

    let pie_fig = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.pie(
                &values,
                PieStyle::new()
                    .labels(&labels)
                    .explode(&explode)
                    .autopct("%1.1f%%")
                    .start_angle(140.0),
            )
            .axes(
                AxesStyle::new()
                    .title("Language Distribution")
                    .hide_axes(true),
            );
        })
        .build()?;
    pie_fig.save("/tmp/mplot/pie_chart.png", SaveOptions::default())?;

    // 4. Quiver Plot
    let qx = [0.0, 1.0, 2.0, 0.0, 1.0, 2.0];
    let qy = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let qu = [0.4, 0.2, -0.3, 0.5, 0.0, -0.4];
    let qv = [0.4, 0.6, 0.2, -0.2, 0.5, -0.3];

    let quiver_fig = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.quiver(
                &qx,
                &qy,
                &qu,
                &qv,
                QuiverStyle::new()
                    .color(Color::hex("#2ca02c"))
                    .scale(0.8),
            )
            .axes(
                AxesStyle::new()
                    .title("Vector Field")
                    .x_label("x")
                    .y_label("y"),
            );
        })
        .build()?;
    quiver_fig.save("/tmp/mplot/quiver_plot.png", SaveOptions::default())?;

    println!("Wrote advanced plot examples to /tmp/mplot/");
    Ok(())
}
