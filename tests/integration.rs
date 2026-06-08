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
            p.boxplot(&data, BoxplotStyle::new())
                .axes(
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
                LineStyle::new()
                    .color(Color::TABLEAU[0])
                    .label("up"),
            )
            .line(
                &x,
                &y2,
                LineStyle::new()
                    .color(Color::TABLEAU[1])
                    .label("down"),
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
                BarStyle::new()
                    .color(Color::TABLEAU[0])
                    .label("counts"),
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
