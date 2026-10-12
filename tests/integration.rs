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

/// A real IQR is a thick translucent band. PathElement with stroke width 0
/// used to leave only the two boundary curves.
#[test]
fn fill_between_paints_a_band_not_an_outline() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let n = 192;
    let x: Vec<f64> = (0..n).map(|i| i as f64).collect();
    let y_lo: Vec<f64> = x.iter().map(|t| 1.0 + 0.15 * (t / 20.0).sin()).collect();
    let y_hi: Vec<f64> = x.iter().map(|t| 3.0 + 0.15 * (t / 17.0).sin()).collect();

    let path = Path::new(OUT_DIR).join("fill_between_band.png");
    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.fill_between(
                &x,
                &y_lo,
                &y_hi,
                FillBetweenStyle::new()
                    .color(Color::rgb(44, 160, 44))
                    .alpha(0.4)
                    .label("IQR"),
            )
            .axes(
                AxesStyle::new()
                    .x_range(0.0, (n - 1) as f64)
                    .y_range(0.0, 4.0)
                    .legend(LegendStyle::show()),
            );
        })
        .build()?
        .save(&path, SaveOptions::default())?;

    let image = image::open(&path).map_err(|err| Error::Io(err.to_string()))?;
    let rgb = image.to_rgb8();
    let thickest = thickest_tinted_column(&rgb);
    assert!(
        thickest > 12,
        "IQR band collapsed to an outline (thickest tinted run is {thickest}px)"
    );
    Ok(())
}

/// Longest vertical run of pixels that are green-tinted rather than white.
fn thickest_tinted_column(image: &image::RgbImage) -> u32 {
    let (width, height) = image.dimensions();
    let x0 = width / 4;
    let x1 = width.saturating_sub(width / 4).max(x0 + 1);
    let mut best = 0u32;
    for x in x0..x1 {
        let mut run = 0u32;
        for y in 0..height {
            let pixel = image.get_pixel(x, y);
            let tinted = pixel[1] > pixel[0].saturating_add(8)
                && pixel[1] > pixel[2].saturating_add(8)
                && pixel[1] < 250;
            if tinted {
                run += 1;
                best = best.max(run);
            } else {
                run = 0;
            }
        }
    }
    best
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

#[test]
fn test_stem() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [0.5, 1.8, -0.4, 2.3, 1.1, -1.5];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.stem(
                &x,
                &y,
                StemStyle::new()
                    .color(Color::hex("#1f77b4"))
                    .marker(Marker::Circle)
                    .marker_size(5.0)
                    .bottom(0.0)
                    .label("signals"),
            )
            .axes(
                AxesStyle::new()
                    .title("Vertical Stem")
                    .x_label("x")
                    .y_label("y")
                    .legend(LegendStyle::show()),
            );
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.stem(
                &x,
                &y,
                StemStyle::new()
                    .horizontal(true)
                    .color(Color::hex("#d62728"))
                    .marker(Marker::Diamond)
                    .marker_size(6.0)
                    .bottom(0.0)
                    .label("horizontal signals"),
            )
            .axes(
                AxesStyle::new()
                    .title("Horizontal Stem")
                    .x_label("y")
                    .y_label("x")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_stem.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_stackplot() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y1 = vec![1.0, 1.5, 2.0, 2.5, 3.0];
    let y2 = vec![2.0, 2.2, 1.8, 1.5, 1.0];
    let y3 = vec![0.5, 1.0, 1.5, 2.0, 2.5];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.stackplot(
                &x,
                &[&y1, &y2, &y3],
                StackplotStyle::new()
                    .labels(["Organic", "Paid", "Referral"])
                    .colors(vec![
                        Color::hex("#1f77b4"),
                        Color::hex("#ff7f0e"),
                        Color::hex("#2ca02c"),
                    ])
                    .alpha(0.8),
            )
            .axes(
                AxesStyle::new()
                    .title("Traffic Sources Stackplot")
                    .x_label("Day")
                    .y_label("Visits (k)")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_stackplot.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_fill_betweenx() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let y = [0.0, 1.0, 2.0, 3.0, 4.0];
    let x1 = [1.0, 2.0, 1.5, 3.0, 2.5];
    let x2 = [3.0, 4.5, 4.0, 5.5, 5.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.fill_betweenx(
                &y,
                &x1,
                &x2,
                FillBetweenStyle::new()
                    .color(Color::hex("#9467bd"))
                    .alpha(0.5)
                    .label("bounded range"),
            )
            .axes(
                AxesStyle::new()
                    .title("Fill Between X")
                    .x_label("x")
                    .y_label("y")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_fill_betweenx.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_stairs() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let values = [1.0, 3.0, 2.0, 4.0, 2.5];
    let edges = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.stairs(
                &values,
                &edges,
                StairsStyle::new()
                    .color(Color::hex("#1f77b4"))
                    .width(2.0)
                    .fill(true)
                    .fill_alpha(0.3)
                    .label("stair outline"),
            )
            .axes(
                AxesStyle::new()
                    .title("Stairs with Edges")
                    .x_label("bin")
                    .y_label("count")
                    .legend(LegendStyle::show()),
            );
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.stairs_values(
                &values,
                StairsStyle::new()
                    .color(Color::hex("#e377c2"))
                    .width(2.5)
                    .dash(LineDash::Dashed)
                    .label("values only"),
            )
            .axes(
                AxesStyle::new()
                    .title("Stairs Values Only")
                    .x_label("step")
                    .y_label("value")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_stairs.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_quiver() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 0.0, 1.0, 2.0];
    let y = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
    let u = [0.5, 0.2, -0.4, 0.6, 0.0, -0.5];
    let v = [0.5, 0.8, 0.3, -0.2, 0.7, -0.4];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.quiver(
                &x,
                &y,
                &u,
                &v,
                QuiverStyle::new()
                    .color(Color::hex("#17becf"))
                    .scale(0.8)
                    .arrow_size(5.0)
                    .width(1.5),
            )
            .axes(
                AxesStyle::new()
                    .title("2D Vector Field (Quiver)")
                    .x_label("x")
                    .y_label("y"),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_quiver.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_pie() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let values = [15.0, 30.0, 45.0, 10.0];
    let labels = ["Frogs", "Hogs", "Dogs", "Logs"];
    let explode = [0.0, 0.1, 0.0, 0.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.pie(
                &values,
                PieStyle::new()
                    .labels(&labels)
                    .explode(&explode)
                    .autopct("%1.1f%%")
                    .start_angle(90.0)
                    .edge_color(Color::rgb(255, 255, 255))
                    .edge_width(1.5),
            )
            .axes(
                AxesStyle::new()
                    .title("Pie Chart")
                    .hide_axes(true),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_pie.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_hist2d() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    // Generate two 2D clusters
    let mut x = Vec::new();
    let mut y = Vec::new();
    for i in 0..100 {
        let angle = i as f64 * 0.1;
        x.push(angle.cos() * 2.0);
        y.push(angle.sin() * 2.0);
        x.push(angle.cos() * 0.5 + 1.0);
        y.push(angle.sin() * 0.5 + 1.0);
    }

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.hist2d(
                &x,
                &y,
                Hist2DStyle::new()
                    .bins(15, 15)
                    .colormap(Colormap::Turbo)
                    .show_colorbar(true),
            )
            .axes(
                AxesStyle::new()
                    .title("2D Histogram (Hist2D)")
                    .x_label("x")
                    .y_label("y"),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_hist2d.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_stacked_bars() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [1.0, 2.0, 3.0, 4.0];
    let bottom_heights = [5.0, 7.0, 4.0, 6.0];
    let top_heights = [3.0, 2.0, 5.0, 4.0];

    let y = [1.0, 2.0, 3.0, 4.0];
    let left_widths = [10.0, 15.0, 8.0, 12.0];
    let right_widths = [5.0, 6.0, 7.0, 4.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.bar(
                &x,
                &bottom_heights,
                BarStyle::new()
                    .color(Color::hex("#1f77b4"))
                    .label("Base"),
            )
            .bar(
                &x,
                &top_heights,
                BarStyle::new()
                    .bottom(&bottom_heights)
                    .color(Color::hex("#ff7f0e"))
                    .label("Stacked"),
            )
            .axes(
                AxesStyle::new()
                    .title("Vertical Stacked Bars")
                    .legend(LegendStyle::show()),
            );
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.barh(
                &y,
                &left_widths,
                BarHStyle::new()
                    .color(Color::hex("#2ca02c"))
                    .label("Base H"),
            )
            .barh(
                &y,
                &right_widths,
                BarHStyle::new()
                    .left(&left_widths)
                    .color(Color::hex("#d62728"))
                    .label("Stacked H"),
            )
            .axes(
                AxesStyle::new()
                    .title("Horizontal Stacked Bars")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_stacked_bars.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_bar_errorbars() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [1.0, 2.0, 3.0, 4.0];
    let heights = [10.0, 15.0, 12.0, 18.0];
    let yerr = [1.0, 1.5, 0.8, 2.0];

    let y = [1.0, 2.0, 3.0];
    let widths = [5.0, 8.0, 6.0];
    let xerr = [0.5, 1.0, 0.7];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.bar(
                &x,
                &heights,
                BarStyle::new()
                    .yerr(&yerr)
                    .capsize(4.0)
                    .color(Color::hex("#4c72b0"))
                    .edge_color(Color::rgb(0, 0, 0))
                    .edge_width(1.0),
            )
            .axes(AxesStyle::new().title("Bar with yerr"));
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.barh(
                &y,
                &widths,
                BarHStyle::new()
                    .xerr(&xerr)
                    .capsize(4.0)
                    .color(Color::hex("#dd8452")),
            )
            .axes(AxesStyle::new().title("BarH with xerr"));
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_bar_errorbars.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_scatter_colormapped_and_sized() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let y = [2.0, 3.5, 2.8, 5.0, 4.2, 6.1, 5.8, 7.2];
    let sizes = [30.0, 80.0, 150.0, 60.0, 200.0, 120.0, 40.0, 180.0];
    let values = [10.0, 25.0, 30.0, 15.0, 50.0, 35.0, 12.0, 48.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.scatter(
                &x,
                &y,
                ScatterStyle::new()
                    .sizes(&sizes)
                    .values(&values)
                    .colormap(Colormap::Spectral)
                    .marker(Marker::Circle)
                    .edge_color(Color::rgb(0, 0, 0))
                    .edge_width(1.0)
                    .alpha(0.85),
            )
            .axes(
                AxesStyle::new()
                    .title("Scatter with Variable Sizes and Colormap")
                    .x_label("X Coordinate")
                    .y_label("Y Coordinate"),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_scatter_colormapped.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_spines_and_despine() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [1.0, 2.0, 3.0, 4.0];
    let y = [10.0, 20.0, 15.0, 25.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.line(&x, &y, LineStyle::new().color(Color::hex("#1f77b4")))
                .axes(
                    AxesStyle::new()
                        .title("Despined (Top & Right Hidden)")
                        .despine(),
                );
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.line(&x, &y, LineStyle::new().color(Color::hex("#ff7f0e")))
                .axes(
                    AxesStyle::new()
                        .title("Custom Spines (Left & Bottom Only)")
                        .spines_visible(false, false, true, true),
                );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_spines.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_legend_locations_and_columns() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [1.0, 2.0, 3.0];
    let y1 = [1.0, 2.0, 3.0];
    let y2 = [2.0, 3.0, 1.0];
    let y3 = [3.0, 1.0, 2.0];
    let y4 = [1.5, 2.5, 1.8];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.line(&x, &y1, LineStyle::new().label("Series 1"))
                .line(&x, &y2, LineStyle::new().label("Series 2"))
                .line(&x, &y3, LineStyle::new().label("Series 3"))
                .line(&x, &y4, LineStyle::new().label("Series 4"))
                .axes(
                    AxesStyle::new()
                        .title("Inside Upper Right (2 cols)")
                        .legend(
                            LegendStyle::show()
                                .loc(LegendLoc::UpperRight)
                                .ncols(2)
                                .title("Metrics")
                                .frameon(true),
                        ),
                );
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.line(&x, &y1, LineStyle::new().label("Alpha"))
                .line(&x, &y2, LineStyle::new().label("Beta"))
                .axes(
                    AxesStyle::new()
                        .title("Inside Lower Left (frameon=false)")
                        .legend(
                            LegendStyle::show()
                                .loc(LegendLoc::LowerLeft)
                                .frameon(false),
                        ),
                );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_legend_styling.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_tick_formatters() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.1, 0.25, 0.5, 0.75, 1.0];
    let y = [10.0, 25.0, 50.0, 75.0, 100.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(&x, &y, LineStyle::new().color(Color::hex("#2ca02c")))
                .axes(
                    AxesStyle::new()
                        .title("Percent Formatted Ticks")
                        .x_tick_format(TickFormat::Percent)
                        .x_label("Completion Rate")
                        .y_label("Score"),
                );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_percent_ticks.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_reversed_and_extended_colormaps() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let cmaps = [
        Colormap::Greys,
        Colormap::Blues,
        Colormap::Reds,
        Colormap::Spectral,
        Colormap::RdBu,
        Colormap::YlGnBu,
        Colormap::Viridis.reversed(),
    ];

    let mut builder = Figure::builder();
    for (i, cmap) in cmaps.into_iter().enumerate() {
        let grid_pos = GridPos::new(1, 7, i + 1);
        let data: Vec<f64> = (0..100).map(|v| v as f64).collect();
        builder = builder.panel(grid_pos, move |p| {
            p.image(data, 10, 10, ImageStyle::new().colormap(cmap))
                .axes(AxesStyle::new().hide_axes(true));
        });
    }

    let figure = builder.build()?;
    let path = Path::new(OUT_DIR).join("test_colormaps_palette.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_additional_markers() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let markers = [
        Marker::TriangleLeft,
        Marker::TriangleRight,
        Marker::Pentagon,
        Marker::Hexagon,
        Marker::Point,
        Marker::VLine,
        Marker::HLine,
    ];

    let mut builder = Figure::builder();
    builder = builder.panel(GridPos::new(1, 1, 1), |p| {
        for (i, &marker) in markers.iter().enumerate() {
            let x = [i as f64 + 1.0];
            let y = [i as f64 + 1.0];
            p.scatter(
                &x,
                &y,
                ScatterStyle::new()
                    .marker(marker)
                    .size(100.0)
                    .color(Color::TABLEAU[i % Color::TABLEAU.len()])
                    .label(format!("Marker {i}")),
            );
        }
        p.axes(
            AxesStyle::new()
                .title("New Marker Shapes")
                .legend(LegendStyle::show().ncols(2)),
        );
    });

    let figure = builder.build()?;
    let path = Path::new(OUT_DIR).join("test_new_markers.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_axline() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(&[0.0, 10.0], &[0.0, 10.0], LineStyle::new().label("reference"))
                .axline(
                    (2.0, 2.0),
                    (8.0, 5.0),
                    LineStyle::new()
                        .color(Color::hex("#ff7f0e"))
                        .dash(LineDash::Dashed)
                        .width(2.0)
                        .label("axline (2 pts)"),
                )
                .axline_slope(
                    (0.0, 8.0),
                    -0.8,
                    LineStyle::new()
                        .color(Color::hex("#2ca02c"))
                        .dash(LineDash::Dotted)
                        .width(2.0)
                        .label("axline (slope)"),
                )
                .axes(
                    AxesStyle::new()
                        .title("AxLine (infinite reference lines)")
                        .x_label("x")
                        .y_label("y")
                        .x_range(0.0, 10.0)
                        .y_range(0.0, 10.0)
                        .legend(LegendStyle::show()),
                );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_axline.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_hlines_vlines() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let ys = [1.0, 2.0, 3.0, 4.0];
    let xmins = [0.5, 1.0, 1.5, 2.0];
    let xmaxs = [4.5, 4.0, 3.5, 3.0];

    let xs = [1.5, 2.5, 3.5];
    let ymins = [0.5, 0.8, 1.0];
    let ymaxs = [4.5, 4.2, 4.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 2, 1), |p| {
            p.hlines(
                &ys,
                &xmins,
                &xmaxs,
                LineStyle::new()
                    .color(Color::TABLEAU[0])
                    .width(2.0)
                    .label("hlines"),
            )
            .hlines_scalar(
                &[0.5, 4.5],
                0.0,
                5.0,
                LineStyle::new()
                    .color(Color::hex("#d62728"))
                    .dash(LineDash::Dashed),
            )
            .axes(
                AxesStyle::new()
                    .title("Horizontal Lines (hlines)")
                    .x_label("x")
                    .y_label("y")
                    .legend(LegendStyle::show()),
            );
        })
        .panel(GridPos::new(1, 2, 2), |p| {
            p.vlines(
                &xs,
                &ymins,
                &ymaxs,
                LineStyle::new()
                    .color(Color::TABLEAU[1])
                    .width(2.0)
                    .label("vlines"),
            )
            .vlines_scalar(
                &[0.5, 4.5],
                0.0,
                5.0,
                LineStyle::new()
                    .color(Color::hex("#2ca02c"))
                    .dash(LineDash::Dotted),
            )
            .axes(
                AxesStyle::new()
                    .title("Vertical Lines (vlines)")
                    .x_label("x")
                    .y_label("y")
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_hlines_vlines.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_eventplot() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let spikes1 = [0.2, 0.5, 1.2, 1.8, 2.5, 3.1, 4.0, 4.7];
    let spikes2 = [0.4, 0.9, 1.5, 2.2, 2.9, 3.6, 4.2];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.eventplot(
                &spikes1,
                EventplotStyle::new()
                    .orientation(EventplotOrientation::Horizontal)
                    .lineoffset(1.0)
                    .linelength(0.8)
                    .color(Color::TABLEAU[0])
                    .linewidth(2.0)
                    .label("Neuron 1"),
            )
            .eventplot(
                &spikes2,
                EventplotStyle::new()
                    .orientation(EventplotOrientation::Horizontal)
                    .lineoffset(2.0)
                    .linelength(0.8)
                    .color(Color::TABLEAU[1])
                    .linewidth(2.0)
                    .label("Neuron 2"),
            )
            .axes(
                AxesStyle::new()
                    .title("Raster Plot (eventplot)")
                    .x_label("Time (s)")
                    .y_label("Neuron Index")
                    .y_range(0.0, 3.0)
                    .legend(LegendStyle::show()),
            );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_eventplot.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_text_and_annotation_bbox() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [0.0, 1.0, 4.0, 9.0, 16.0];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(&x, &y, LineStyle::new().color(Color::TABLEAU[0]))
                .text(
                    1.0,
                    12.0,
                    "Info Box: y = x²",
                    TextStyle::new().fontsize(12.0).bbox(
                        BboxStyle::new()
                            .face_color(Color::hex("#fffae6"))
                            .edge_color(Color::hex("#b38600"))
                            .edge_width(1.5)
                            .pad(5.0),
                    ),
                )
                .annotate(
                    "Turning Point",
                    (2.0, 4.0),
                    (2.5, 8.0),
                    AnnotationStyle::new()
                        .fontsize(11.0)
                        .arrow(ArrowStyle::new().color(Color::hex("#d62728")).width(1.5))
                        .bbox(
                            BboxStyle::new()
                                .face_color(Color::hex("#e8f4f8"))
                                .edge_color(Color::hex("#1f77b4"))
                                .edge_width(1.2)
                                .pad(4.0),
                        ),
                )
                .axes(
                    AxesStyle::new()
                        .title("Text and Annotation Bounding Boxes")
                        .x_label("x")
                        .y_label("y"),
                );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_bbox.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_engineering_tick_format() -> Result<()> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let freqs = [100.0, 1_000.0, 10_000.0, 100_000.0, 1_000_000.0];
    let volts = [0.001, 0.005, 0.02, 0.05, 0.1];

    let figure = Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(&freqs, &volts, LineStyle::new().color(Color::TABLEAU[0]))
                .axes(
                    AxesStyle::new()
                        .title("Frequency Response")
                        .x_label("Frequency (Hz)")
                        .y_label("Amplitude (V)")
                        .x_tick_format(TickFormat::Engineering)
                        .y_tick_format(TickFormat::Engineering),
                );
        })
        .build()?;

    let path = Path::new(OUT_DIR).join("test_eng_format.png");
    figure.save(&path, SaveOptions::default())?;
    let metadata = std::fs::metadata(path).map_err(|_| Error::Io("output missing".into()))?;
    assert!(metadata.len() > 1_000);
    Ok(())
}
