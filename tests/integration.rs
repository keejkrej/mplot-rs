use mplot::prelude::*;
use std::path::Path;

const OUT_DIR: &str = "/tmp/mplot/tests";

#[test]
fn test_subplot() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = &[1.0, 2.0, 3.0, 4.0];
    let y = &[1.0, 1.424, 1.732, 2.0];
    let z = &[1.0, 4.0, 9.0, 16.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.line(x, y, LineStyle::new())
                .axes(AxesStyle::new().x_label("x").y_label("y").grid(true));
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.line(x, z, LineStyle::new())
                .axes(AxesStyle::new().x_label("x").y_label("y").grid(true));
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_subplot.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_boxplot() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let data = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 5.0, 6.0],
    ];
    let ticks = [1, 2, 3];
    let labels = ["A", "B", "C"];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.boxplot(&data, BoxplotStyle::new()).axes(
                AxesStyle::new()
                    .title("boxplot test")
                    .x_tick_labels(&ticks, &labels),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_boxplot.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_legend() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 3.0];
    let y1 = [0.0, 1.0, 2.0, 3.0];
    let y2 = [3.0, 2.0, 1.0, 0.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(
                &x,
                &y1,
                LineStyle::new().color(Color::TABLEAU[0]).label("up"),
            )
            .line(
                &x,
                &y2,
                LineStyle::new().color(Color::TABLEAU[1]).label("down"),
            )
            .axes(
                AxesStyle::new()
                    .title("legend test")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_legend.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_legend_on_a_long_time_axis() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();
    let x = [0.0, 250.0, 500.0, 750.0, 1000.0];
    let mean = [1.0, 0.8, 0.5, 0.4, 0.3];
    let median = [1.0, 1.0, 0.0, 0.0, 0.0];
    let figure = Figure::builder()
        .size(Size::inches(8.0, 6.0))
        .panel(GridPos::new(2, 2, 1), |p| {
            p.fill_between(
                &x,
                &[0.0, 0.0, 0.0, 0.0, 0.0],
                &[1.0, 1.0, 1.0, 1.0, 0.25],
                FillBetweenStyle::new()
                    .color(Color::TABLEAU[0])
                    .alpha(0.25)
                    .label("IQR"),
            )
            .line(
                &x,
                &median,
                LineStyle::new().color(Color::TABLEAU[0]).label("median"),
            )
            .line(
                &x,
                &mean,
                LineStyle::new()
                    .color(Color::TABLEAU[0])
                    .dash(LineDash::Dashed)
                    .label("mean"),
            )
            .axes(
                AxesStyle::new()
                    .title("CMRA_1_2 (240 traces)")
                    .x_label("time (min)")
                    .y_label("engagers")
                    .x_range(0.0, 1000.0)
                    .y_range(-1.0, 1.0)
                    .legend(LegendStyle::show()),
            );
        })
        .panel(GridPos::new(2, 2, 2), |p| {
            p.line(&x, &mean, LineStyle::new()).axes(
                AxesStyle::new()
                    .title("other")
                    .x_range(0.0, 1000.0)
                    .y_range(-1.0, 1.0),
            );
        })
        .panel(GridPos::new(2, 2, 3), |p| {
            p.line(&x, &mean, LineStyle::new())
                .axes(AxesStyle::new().x_range(0.0, 1000.0).y_range(-1.0, 1.0));
        })
        .panel(GridPos::new(2, 2, 4), |p| {
            p.line(&x, &[0.0, 0.0, 0.0, 0.0, 0.0], LineStyle::new())
                .axes(AxesStyle::new().x_range(0.0, 1000.0).y_range(-1.0, 1.0));
        })
        .build()?;
    let path = Path::new(OUT_DIR).join("test_legend_time_axis.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(&path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_horizontal_boxplot() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let groups = vec![vec![1.0, 2.0, 3.0, 4.0], vec![2.0, 3.0, 4.0, 5.0, 6.0]];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.boxplot(&groups, BoxplotStyle::new().horizontal(true))
                .axes(AxesStyle::new().title("horizontal boxplot"));
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_horizontal_boxplot.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_bar() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [1.0, 2.0, 3.0, 4.0];
    let heights = [3.0, 7.0, 5.0, 9.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.bar(
                &x,
                &heights,
                BarStyle::new().color(Color::TABLEAU[0]).label("counts"),
            )
            .axes(
                AxesStyle::new()
                    .title("bar chart")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_bar.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_histogram() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let data = [1.0, 1.5, 2.0, 2.2, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.histogram(
                &data,
                HistStyle::new()
                    .bins(5)
                    .color(Color::TABLEAU[1])
                    .label("samples"),
            )
            .axes(AxesStyle::new().title("histogram"));
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_histogram.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_fill_between() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y1 = [0.0, 1.0, 2.0, 1.0, 0.0];
    let y2 = [0.5, 1.5, 2.5, 1.5, 0.5];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.fill_between(
                &x,
                &y1,
                &y2,
                FillBetweenStyle::new()
                    .color(Color::TABLEAU[2])
                    .alpha(0.4)
                    .label("band"),
            )
            .axes(
                AxesStyle::new()
                    .title("fill between")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_fill_between.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_image() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let data = vec![0.0, 0.5, 1.0, 0.25, 0.75, 1.0, 0.0, 0.5, 1.0];
    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.image(
                data,
                3,
                3,
                ImageStyle::new()
                    .extent(0.0, 3.0, 0.0, 3.0)
                    .colormap(Colormap::Viridis)
                    .colorbar(true),
            )
            .axes(AxesStyle::new().title("image"));
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_image.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_svg_export() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0];
    let y = [0.0, 1.0, 4.0];
    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(&x, &y, LineStyle::new())
                .axes(AxesStyle::new().title("svg export"));
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_export.svg");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 500);
    Ok(())
}

#[test]
fn test_pdf_export() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 3.0];
    let y = [0.0, 1.0, 4.0, 9.0];
    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(&x, &y, LineStyle::new().label("y = x²"))
                .axes(AxesStyle::new().title("pdf export"));
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_export.pdf");
    figure.save(&path, SaveOptions::default())?;

    let bytes = std::fs::read(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(bytes.starts_with(b"%PDF"));
    assert!(bytes.len() > 500);
    Ok(())
}

#[test]
fn test_gridspec_span() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let gs = GridSpec::new(2, 2);
    let x = [0.0, 1.0, 2.0];
    let y = [1.0, 2.0, 3.0];

    let figure = Figure::builder()
        .panel(gs.at(0, 0), |p| {
            p.line(&x, &y, LineStyle::new())
                .axes(AxesStyle::new().title("A"));
        })
        .panel(gs.span(0, 1, 1, 2), |p| {
            p.bar(
                &[1.0, 2.0],
                &[3.0, 5.0],
                BarStyle::new().color(Color::TABLEAU[0]),
            )
            .axes(AxesStyle::new().title("wide panel"));
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_gridspec_span.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_constrained_colorbar() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let data = vec![0.0, 0.5, 1.0, 0.25, 0.75, 1.0, 0.0, 0.5, 1.0];
    let figure = Figure::builder()
        .constrained_layout(true)
        .panel(GridPos::new(1, 1, 1), |p| {
            p.image(
                data,
                3,
                3,
                ImageStyle::new()
                    .extent(0.0, 3.0, 0.0, 3.0)
                    .colormap(Colormap::Viridis)
                    .colorbar(true),
            )
            .axes(AxesStyle::new().title("constrained colorbar"));
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_constrained_colorbar.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_scatter() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [0.5, 1.8, 1.2, 3.5, 2.8, 4.5];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.scatter(
                &x,
                &y,
                ScatterStyle::new()
                    .color(Color::TABLEAU[0])
                    .size(50.0)
                    .marker(Marker::Diamond)
                    .edge_color(Color::hex("#000000"))
                    .edge_width(1.2)
                    .label("samples"),
            )
            .axes(
                AxesStyle::new()
                    .title("Scatter Plot")
                    .x_label("x")
                    .y_label("y")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_scatter.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_errorbar() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [1.0, 2.0, 3.0, 4.0];
    let y = [2.0, 3.5, 2.8, 4.2];
    let yerr = [0.3, 0.5, 0.4, 0.6];
    let xerr = [0.1, 0.15, 0.1, 0.2];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.errorbar(
                &x,
                &y,
                ErrorBarStyle::new()
                    .yerr(&yerr)
                    .xerr(&xerr)
                    .color(Color::TABLEAU[1])
                    .capsize(4.0)
                    .capthick(1.2)
                    .marker(Marker::Circle)
                    .label("measurements"),
            )
            .axes(
                AxesStyle::new()
                    .title("Error Bar Plot")
                    .x_label("condition")
                    .y_label("response")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_errorbar.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_step() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [1.0, 0.8, 0.6, 0.3, 0.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.step(
                &x,
                &y,
                StepStyle::new()
                    .where_step(StepWhere::Post)
                    .color(Color::TABLEAU[2])
                    .width(2.0)
                    .label("survival"),
            )
            .axes(
                AxesStyle::new()
                    .title("Kaplan-Meier Step Plot")
                    .x_label("time")
                    .y_label("survival probability")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_step.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_barh() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let y = [1.0, 2.0, 3.0, 4.0];
    let widths = [10.0, 25.0, 18.0, 32.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.barh(
                &y,
                &widths,
                BarHStyle::new()
                    .height(0.6)
                    .color(Color::TABLEAU[3])
                    .label("scores"),
            )
            .axes(
                AxesStyle::new()
                    .title("Horizontal Bar Plot")
                    .x_label("score")
                    .y_label("category")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_barh.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_axlines_and_spans() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [0.0, 2.0, 1.5, 3.0, 2.5];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.axhspan(
                1.0,
                2.0,
                FillBetweenStyle::new()
                    .color(Color::hex("#cccccc"))
                    .alpha(0.3)
                    .label("nominal range"),
            )
            .axvspan(
                1.5,
                2.5,
                FillBetweenStyle::new()
                    .color(Color::hex("#ddddff"))
                    .alpha(0.3),
            )
            .axhline(
                2.0,
                LineStyle::new()
                    .color(Color::hex("#d62728"))
                    .dash(LineDash::Dashed)
                    .label("threshold"),
            )
            .axvline(
                2.0,
                LineStyle::new()
                    .color(Color::hex("#2ca02c"))
                    .dash(LineDash::Dotted),
            )
            .line(&x, &y, LineStyle::new().color(Color::TABLEAU[0]))
            .axes(
                AxesStyle::new()
                    .title("Reference Lines and Spans")
                    .x_label("x")
                    .y_label("y")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_axlines_and_spans.png");
    figure.save(&path, SaveOptions::default())?;

    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_new_colormaps_image() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let data = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    for (name, cmap) in [
        ("plasma", Colormap::Plasma),
        ("inferno", Colormap::Inferno),
        ("magma", Colormap::Magma),
        ("cividis", Colormap::Cividis),
        ("coolwarm", Colormap::Coolwarm),
        ("turbo", Colormap::Turbo),
    ] {
        let figure = Figure::builder()
            .panel(GridPos::new(1, 1, 1), |p| {
                p.image(
                    data.clone(),
                    5,
                    1,
                    ImageStyle::new()
                        .extent(0.0, 5.0, 0.0, 1.0)
                        .colormap(cmap)
                        .colorbar(true),
                )
                .axes(AxesStyle::new().title(name));
            })
            .build()?;

        let path = Path::new(OUT_DIR).join(format!("test_cmap_{name}.png"));
        figure.save(&path, SaveOptions::default())?;
        let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
        assert!(metadata.len() > 1_000);
    }
    Ok(())
}

#[test]
fn test_annotate() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [1.0, 2.0, 5.0, 3.0, 2.0, 1.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(&x, &y, LineStyle::new())
                .annotate(
                    "Local Peak (x=2, y=5)",
                    (2.0, 5.0),
                    (3.0, 4.5),
                    AnnotationStyle::new()
                        .fontsize(11.0)
                        .ha(HorizontalAlignment::Left)
                        .va(VerticalAlignment::Center)
                        .arrow(ArrowStyle::new().color(Color::hex("#d62728")).width(1.5)),
                )
                .axes(
                    AxesStyle::new()
                        .title("Annotated Curve")
                        .x_label("x")
                        .y_label("y"),
                );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_annotate.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_inverted_axes() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let depths = [0.0, 50.0, 100.0, 150.0, 200.0];
    let temps = [25.0, 21.0, 16.0, 10.0, 4.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.line(&temps, &depths, LineStyle::new().color(Color::TABLEAU[0]))
                .axes(
                    AxesStyle::new()
                        .title("Depth Profile (invert_y)")
                        .x_label("temperature (C)")
                        .y_label("depth (m)")
                        .invert_y(true),
                );
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.line(&temps, &depths, LineStyle::new().color(Color::TABLEAU[1]))
                .axes(
                    AxesStyle::new()
                        .title("Reversed Range")
                        .x_label("temperature (C)")
                        .y_label("depth (m)")
                        .y_range(250.0, 0.0),
                );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_inverted_axes.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_grid_style() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [10.0, 15.0, 13.0, 18.0, 22.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(&x, &y, LineStyle::new().color(Color::TABLEAU[0]))
                .axes(
                    AxesStyle::new()
                        .title("Custom Grid")
                        .x_label("x")
                        .y_label("y")
                        .grid_style(
                            GridStyle::new()
                                .color(Color::hex("#1f77b4"))
                                .width(1.2)
                                .alpha(0.5),
                        ),
                );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_grid_style.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_violinplot() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let g1 = vec![1.0, 2.0, 2.5, 3.0, 3.5, 4.0, 5.0];
    let g2 = vec![2.0, 3.0, 3.5, 4.0, 4.5, 5.5, 6.0];
    let datasets = [g1, g2];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.violinplot(
                &datasets,
                ViolinStyle::new()
                    .show_means(true)
                    .show_medians(true)
                    .show_extrema(true),
            )
            .axes(
                AxesStyle::new()
                    .title("Vertical Violin")
                    .x_label("group")
                    .y_label("value"),
            );
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.violinplot(
                &datasets,
                ViolinStyle::new()
                    .horizontal(true)
                    .show_extrema(true)
                    .face_color(Color::hex("#2ca02c")),
            )
            .axes(
                AxesStyle::new()
                    .title("Horizontal Violin")
                    .x_label("value")
                    .y_label("group"),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_violinplot.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}
