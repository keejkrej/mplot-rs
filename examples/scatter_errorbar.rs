//! Example demonstrating scatter, errorbar, step, horizontal bar, and reference lines.

use mplot::prelude::*;

fn main() -> Result<()> {
    std::fs::create_dir_all("/tmp/mplot/examples").ok();

    let figure = Figure::builder()
        .size(Size::inches(12.0, 8.0))
        // Panel 1: Scatter plot with diamond markers and threshold
        .panel(GridPos::new(2, 2, 1), |p| {
            let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
            let y = [0.8, 1.9, 1.5, 3.2, 2.7, 4.3];
            p.axhspan(1.0, 3.0, FillBetweenStyle::new().color(Color::hex("#e0e0e0")).alpha(0.5))
                .axhline(2.5, LineStyle::new().color(Color::hex("#d62728")).dash(LineDash::Dashed).label("target"))
                .scatter(
                    &x,
                    &y,
                    ScatterStyle::new()
                        .color(Color::TABLEAU[0])
                        .size(60.0)
                        .marker(Marker::Diamond)
                        .edge_color(Color::hex("#000000"))
                        .edge_width(1.0)
                        .label("observed"),
                )
                .axes(
                    AxesStyle::new()
                        .title("Scatter & Reference Lines")
                        .x_label("dose (uM)")
                        .y_label("response")
                        .legend(LegendStyle::show()),
                );
        })
        // Panel 2: Error bars with caps
        .panel(GridPos::new(2, 2, 2), |p| {
            let x = [1.0, 2.0, 3.0, 4.0];
            let y = [2.2, 3.8, 3.1, 4.6];
            let yerr = [0.4, 0.5, 0.3, 0.6];
            p.errorbar(
                &x,
                &y,
                ErrorBarStyle::new()
                    .yerr(&yerr)
                    .color(Color::TABLEAU[1])
                    .capsize(4.0)
                    .marker(Marker::Circle)
                    .label("treatment (mean +/- SD)"),
            )
            .axes(
                AxesStyle::new()
                    .title("Error Bars")
                    .x_label("group")
                    .y_label("signal")
                    .legend(LegendStyle::show()),
            );
        })
        // Panel 3: Step plot (e.g. survival curve)
        .panel(GridPos::new(2, 2, 3), |p| {
            let t = [0.0, 100.0, 250.0, 500.0, 750.0, 1000.0];
            let s_ctrl = [1.0, 0.95, 0.85, 0.70, 0.60, 0.55];
            let s_drug = [1.0, 0.80, 0.50, 0.30, 0.15, 0.05];
            p.step(
                &t,
                &s_ctrl,
                StepStyle::new()
                    .where_step(StepWhere::Post)
                    .color(Color::TABLEAU[2])
                    .width(1.8)
                    .label("control"),
            )
            .step(
                &t,
                &s_drug,
                StepStyle::new()
                    .where_step(StepWhere::Post)
                    .color(Color::TABLEAU[3])
                    .width(1.8)
                    .dash(LineDash::Dashed)
                    .label("treated"),
            )
            .axes(
                AxesStyle::new()
                    .title("Kaplan-Meier Step Curves")
                    .x_label("time (min)")
                    .y_label("survival fraction")
                    .legend(LegendStyle::show()),
            );
        })
        // Panel 4: Horizontal bar chart
        .panel(GridPos::new(2, 2, 4), |p| {
            let y = [1.0, 2.0, 3.0, 4.0];
            let widths = [12.5, 28.0, 19.5, 34.0];
            let labels = ["Condition A", "Condition B", "Condition C", "Condition D"];
            p.barh(
                &y,
                &widths,
                BarHStyle::new().height(0.6).color(Color::TABLEAU[4]),
            )
            .axes(
                AxesStyle::new()
                    .title("Horizontal Bars")
                    .x_label("viability (%)")
                    .y_label("assay")
                    .y_tick_labels(&y, &labels),
            );
        })
        .build()?;

    let path = "/tmp/mplot/examples/scatter_errorbar.png";
    figure.save(path, SaveOptions::default())?;
    println!("wrote figure to {path}");
    Ok(())
}
