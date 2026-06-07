use mplot::{Boxplot, Curve, Plot, StrError};
use std::path::Path;

const OUT_DIR: &str = "/tmp/mplot/tests";

#[test]
fn test_subplot() -> Result<(), StrError> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    let x = &[1.0, 2.0, 3.0, 4.0];
    let y = &[1.0, 1.424, 1.732, 2.0];
    let z = &[1.0, 4.0, 9.0, 16.0];
    curve1.draw(x, y);
    curve2.draw(x, z);

    let mut plot = Plot::new();
    plot.set_subplot(1, 2, 1).add(&curve1).grid_and_labels("x", "y");
    plot.set_subplot(1, 2, 2).add(&curve2).grid_and_labels("x", "y");

    let path = Path::new(OUT_DIR).join("test_subplot.png");
    plot.save(&path)?;

    let metadata = std::fs::metadata(path).map_err(|_| "output missing")?;
    assert!(metadata.len() > 1_000);
    Ok(())
}

#[test]
fn test_boxplot() -> Result<(), StrError> {
    std::fs::create_dir_all(OUT_DIR).ok();

    let data = vec![
        vec![1.0, 2.0, 3.0, 4.0, 5.0],
        vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        vec![3.0, 4.0, 5.0, 6.0],
    ];
    let ticks = [1, 2, 3];
    let labels = ["A", "B", "C"];

    let mut boxes = Boxplot::new();
    boxes.draw(&data);

    let mut plot = Plot::new();
    plot.add(&boxes)
        .set_title("boxplot test")
        .set_ticks_x_labels(&ticks, &labels);

    let path = Path::new(OUT_DIR).join("test_boxplot.png");
    plot.save(&path)?;

    let metadata = std::fs::metadata(path).map_err(|_| "output missing")?;
    assert!(metadata.len() > 1_000);
    Ok(())
}
