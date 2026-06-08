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
