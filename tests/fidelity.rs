//! Golden-image fidelity tests against matplotlib reference PNGs.

use mplot::{Boxplot, Curve, Plot, StrError};
use std::path::{Path, PathBuf};

const GOLDEN_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fidelity/golden");
const OUT_DIR: &str = "/tmp/mplot/fidelity";

fn render_simple_line(path: &Path) -> Result<(), StrError> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [0.0, 1.0, 4.0, 9.0, 16.0];

    let mut curve = Curve::new();
    curve.set_line_color("#1f77b4").set_label("y = x²");
    curve.draw(&x, &y);

    let mut plot = Plot::new();
    plot.add(&curve)
        .set_title("Simple line plot")
        .set_labels("x", "y");
    plot.save(path)
}

fn render_subplot_lines(path: &Path) -> Result<(), StrError> {
    let rows = 2;
    let cols = 2;
    let mut plot = Plot::new();
    plot.set_figure_size_inches(10.0, 8.0);

    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let panels: [(&str, &str, [f64; 5]); 4] = [
        ("Panel A", "#1f77b4", [0.0, 0.5, 1.5, 2.0, 2.5]),
        ("Panel B", "#ff7f0e", [0.0, 0.8, 1.2, 1.8, 2.2]),
        ("Panel C", "#2ca02c", [0.0, 0.3, 1.0, 1.4, 1.9]),
        ("Panel D", "#d62728", [0.0, 0.6, 1.1, 1.6, 2.4]),
    ];

    for (index, (title, color, ys)) in panels.iter().enumerate() {
        let mut curve = Curve::new();
        curve.set_line_color(color);
        curve.draw(&xs, ys);

        plot.set_subplot(rows, cols, index + 1)
            .add(&curve)
            .set_title(title)
            .set_labels("x", "y")
            .set_yrange(0.0, 3.0);
    }

    plot.save(path)
}

fn render_boxplot_linear(path: &Path) -> Result<(), StrError> {
    let groups = vec![
        vec![1.2, 1.5, 1.8, 2.0, 2.1],
        vec![2.0, 2.3, 2.5, 2.8, 3.0, 3.2],
        vec![0.8, 1.0, 1.1, 1.4],
    ];
    let ticks = [1, 2, 3];
    let labels = ["Group A\n(n=5)", "Group B\n(n=6)", "Group C\n(n=4)"];

    let mut boxes = Boxplot::new();
    boxes.draw(&groups);

    let mut plot = Plot::new();
    plot.add(&boxes)
        .set_title("Boxplot (linear y)")
        .set_labels("category", "value")
        .set_ticks_x_labels(&ticks, &labels);
    plot.save(path)
}

fn render_boxplot_log(path: &Path) -> Result<(), StrError> {
    let groups = vec![
        vec![12.0, 18.0, 25.0, 31.0],
        vec![8.0, 15.0, 22.0, 29.0, 35.0],
        vec![10.0, 20.0, 40.0, 55.0],
    ];
    let ticks = [1, 2, 3];
    let labels = ["Group A\n(n=4)", "Group B\n(n=5)", "Group C\n(n=4)"];

    let mut boxes = Boxplot::new();
    boxes.draw(&groups);

    let mut plot = Plot::new();
    plot.set_figure_size_inches(8.0, 6.0)
        .add(&boxes)
        .set_title("Boxplot (log y)")
        .set_labels("category", "value")
        .set_ticks_x_labels(&ticks, &labels)
        .set_log_y(true);
    plot.save(path)
}

fn render_gallery_line(path: &Path) -> Result<(), StrError> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [1.0, 1.4, 1.8, 2.2, 2.6, 3.0];

    let mut curve = Curve::new();
    curve.set_line_color("#2ca02c");
    curve.draw(&x, &y);

    let mut plot = Plot::new();
    plot.add(&curve)
        .set_title("Line plot")
        .set_labels("x", "f(x)");
    plot.save(path)
}

fn render_gallery_subplots(path: &Path) -> Result<(), StrError> {
    let xs = [0.0, 2.0, 4.0, 6.0, 8.0];
    let mut plot = Plot::new();
    plot.set_figure_size_inches(10.0, 4.0);

    for (index, (title, color, ys)) in [
        ("Sine-ish", "#1f77b4", [0.0, 0.9, 1.4, 1.2, 0.8]),
        ("Ramp", "#ff7f0e", [0.0, 0.5, 1.0, 1.5, 2.0]),
    ]
    .iter()
    .enumerate()
    {
        let mut curve = Curve::new();
        curve.set_line_color(color);
        curve.draw(&xs, ys);

        plot.set_subplot(1, 2, index + 1)
            .add(&curve)
            .set_title(title)
            .set_labels("x", "y");
    }

    plot.save(path)
}

fn render_gallery_boxplot(path: &Path) -> Result<(), StrError> {
    let groups = vec![vec![2.0, 3.0, 4.0, 5.0], vec![4.0, 5.0, 6.0, 7.0, 8.0]];
    let ticks = [1, 2];
    let labels = ["Low", "High"];

    let mut boxes = Boxplot::new();
    boxes.draw(&groups);

    let mut plot = Plot::new();
    plot.add(&boxes)
        .set_title("Two-group boxplot")
        .set_labels("group", "measurement")
        .set_ticks_x_labels(&ticks, &labels);
    plot.save(path)
}

fn compare_pngs(actual: &Path, golden: &Path, max_mean_delta: f64) -> Result<(), String> {
    let actual_img = image::open(actual).map_err(|err| err.to_string())?;
    let golden_img = image::open(golden).map_err(|err| err.to_string())?;
    let (aw, ah) = (actual_img.width(), actual_img.height());
    let (gw, gh) = (golden_img.width(), golden_img.height());
    if (aw, ah) != (gw, gh) {
        return Err(format!(
            "dimension mismatch for {}: actual ({aw}, {ah}) vs golden ({gw}, {gh})",
            golden.file_name().unwrap_or_default().to_string_lossy(),
        ));
    }

    let mut delta_sum = 0.0;
    let mut count = 0.0;
    for (a, g) in actual_img.to_rgb8().pixels().zip(golden_img.to_rgb8().pixels()) {
        for channel in 0..3 {
            delta_sum += (i32::from(a[channel]) - i32::from(g[channel])).unsigned_abs() as f64;
            count += 1.0;
        }
    }
    let mean_delta = delta_sum / count;
    if mean_delta > max_mean_delta {
        return Err(format!(
            "mean channel delta {:.2} exceeds {:.2} for {}",
            mean_delta,
            max_mean_delta,
            golden.file_name().unwrap_or_default().to_string_lossy()
        ));
    }
    Ok(())
}

fn fidelity_case(
    name: &str,
    render: fn(&Path) -> Result<(), StrError>,
    max_mean_delta: f64,
) -> Result<(), String> {
    std::fs::create_dir_all(OUT_DIR).map_err(|err| err.to_string())?;
    let out = PathBuf::from(OUT_DIR).join(name);
    let golden = PathBuf::from(GOLDEN_DIR).join(name);
    render(&out).map_err(|err| err.to_string())?;
    compare_pngs(&out, &golden, max_mean_delta)
}

#[test]
fn fidelity_simple_line() {
    fidelity_case("simple_line.png", render_simple_line, 35.0).unwrap();
}

#[test]
fn fidelity_subplot_lines() {
    fidelity_case("subplot_lines.png", render_subplot_lines, 35.0).unwrap();
}

#[test]
fn fidelity_boxplot_linear() {
    fidelity_case("boxplot_linear.png", render_boxplot_linear, 35.0).unwrap();
}

#[test]
fn fidelity_boxplot_log() {
    fidelity_case("boxplot_log.png", render_boxplot_log, 40.0).unwrap();
}

#[test]
fn fidelity_gallery_line() {
    fidelity_case("gallery_line.png", render_gallery_line, 35.0).unwrap();
}

#[test]
fn fidelity_gallery_subplots() {
    fidelity_case("gallery_subplots.png", render_gallery_subplots, 35.0).unwrap();
}

#[test]
fn fidelity_gallery_boxplot() {
    fidelity_case("gallery_boxplot.png", render_gallery_boxplot, 35.0).unwrap();
}
