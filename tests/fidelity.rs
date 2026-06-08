//! Golden-image fidelity tests against matplotlib reference PNGs.

use mplot::prelude::{
    AxesStyle, BarStyle, BoxplotStyle, Colormap, Color, ContourStyle, Figure, FillBetweenStyle,
    GridPos, HistStyle, ImageStyle, LineStyle, SaveOptions, Scale, Size,
};
use mplot::Result;
use std::path::{Path, PathBuf};

const GOLDEN_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fidelity/golden");
const OUT_DIR: &str = "/tmp/mplot/fidelity";

const FIDELITY_DPI: u32 = 100;

fn fidelity_save_options() -> SaveOptions {
    SaveOptions::new().dpi(FIDELITY_DPI)
}

fn render_simple_line(path: &Path) -> Result<()> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y = [0.0, 1.0, 4.0, 9.0, 16.0];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(
                &x,
                &y,
                LineStyle::new()
                    .color(Color::hex("#1f77b4"))
                    .label("y = x²"),
            )
            .axes(
                AxesStyle::new()
                    .title("Simple line plot")
                    .x_label("x")
                    .y_label("y"),
            );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn render_subplot_lines(path: &Path) -> Result<()> {
    let rows = 2;
    let cols = 2;
    let xs = [0.0, 1.0, 2.0, 3.0, 4.0];
    let panels: [(&str, Color, [f64; 5]); 4] = [
        ("Panel A", Color::hex("#1f77b4"), [0.0, 0.5, 1.5, 2.0, 2.5]),
        ("Panel B", Color::hex("#ff7f0e"), [0.0, 0.8, 1.2, 1.8, 2.2]),
        ("Panel C", Color::hex("#2ca02c"), [0.0, 0.3, 1.0, 1.4, 1.9]),
        ("Panel D", Color::hex("#d62728"), [0.0, 0.6, 1.1, 1.6, 2.4]),
    ];

    let mut builder = Figure::builder().size(Size::inches(10.0, 8.0));
    for (index, (title, color, ys)) in panels.iter().enumerate() {
        builder = builder.panel(GridPos::new(rows, cols, index + 1), |p| {
            p.line(&xs, ys, LineStyle::new().color(*color))
                .axes(
                    AxesStyle::new()
                        .title(*title)
                        .x_label("x")
                        .y_label("y")
                        .y_range(0.0, 3.0),
                );
        });
    }

    builder.build()?.save(path, fidelity_save_options())
}

fn render_boxplot_linear(path: &Path) -> Result<()> {
    let groups = vec![
        vec![1.2, 1.5, 1.8, 2.0, 2.1],
        vec![2.0, 2.3, 2.5, 2.8, 3.0, 3.2],
        vec![0.8, 1.0, 1.1, 1.4],
    ];
    let ticks = [1, 2, 3];
    let labels = ["Group A\n(n=5)", "Group B\n(n=6)", "Group C\n(n=4)"];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.boxplot(&groups, BoxplotStyle::new())
                .axes(
                    AxesStyle::new()
                        .title("Boxplot (linear y)")
                        .x_label("category")
                        .y_label("value")
                        .x_tick_labels(&ticks, &labels),
                );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn render_boxplot_log(path: &Path) -> Result<()> {
    let groups = vec![
        vec![12.0, 18.0, 25.0, 31.0],
        vec![8.0, 15.0, 22.0, 29.0, 35.0],
        vec![10.0, 20.0, 40.0, 55.0],
    ];
    let ticks = [1, 2, 3];
    let labels = ["Group A\n(n=4)", "Group B\n(n=5)", "Group C\n(n=4)"];

    Figure::builder()
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
        .build()?
        .save(path, fidelity_save_options())
}

fn render_gallery_line(path: &Path) -> Result<()> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
    let y = [1.0, 1.4, 1.8, 2.2, 2.6, 3.0];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(&x, &y, LineStyle::new().color(Color::hex("#2ca02c")))
                .axes(
                    AxesStyle::new()
                        .title("Line plot")
                        .x_label("x")
                        .y_label("f(x)"),
                );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn render_gallery_subplots(path: &Path) -> Result<()> {
    let xs = [0.0, 2.0, 4.0, 6.0, 8.0];
    let panels = [
        ("Sine-ish", Color::hex("#1f77b4"), [0.0, 0.9, 1.4, 1.2, 0.8]),
        ("Ramp", Color::hex("#ff7f0e"), [0.0, 0.5, 1.0, 1.5, 2.0]),
    ];

    let mut builder = Figure::builder().size(Size::inches(10.0, 4.0));
    for (index, (title, color, ys)) in panels.iter().enumerate() {
        builder = builder.panel(GridPos::new(1, 2, index + 1), |p| {
            p.line(&xs, ys, LineStyle::new().color(*color))
                .axes(
                    AxesStyle::new()
                        .title(*title)
                        .x_label("x")
                        .y_label("y"),
                );
        });
    }

    builder.build()?.save(path, fidelity_save_options())
}

fn render_gallery_boxplot(path: &Path) -> Result<()> {
    let groups = vec![vec![2.0, 3.0, 4.0, 5.0], vec![4.0, 5.0, 6.0, 7.0, 8.0]];
    let ticks = [1, 2];
    let labels = ["Low", "High"];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.boxplot(&groups, BoxplotStyle::new())
                .axes(
                    AxesStyle::new()
                        .title("Two-group boxplot")
                        .x_label("group")
                        .y_label("measurement")
                        .x_tick_labels(&ticks, &labels),
                );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn render_bar_chart(path: &Path) -> Result<()> {
    let x = [1.0, 2.0, 3.0, 4.0];
    let heights = [3.0, 7.0, 5.0, 9.0];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.bar(
                &x,
                &heights,
                BarStyle::new()
                    .width(0.8)
                    .color(Color::hex("#1f77b4")),
            )
            .axes(
                AxesStyle::new()
                    .title("Bar chart")
                    .x_label("x")
                    .y_label("height"),
            );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn render_histogram(path: &Path) -> Result<()> {
    let data = [1.0, 1.5, 2.0, 2.2, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.histogram(
                &data,
                HistStyle::new()
                    .bins(10)
                    .color(Color::hex("#ff7f0e")),
            )
            .axes(
                AxesStyle::new()
                    .title("Histogram")
                    .x_label("value")
                    .y_label("count"),
            );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn render_fill_between(path: &Path) -> Result<()> {
    let x = [0.0, 1.0, 2.0, 3.0, 4.0];
    let y1 = [0.0, 1.0, 2.0, 1.0, 0.0];
    let y2 = [0.5, 1.5, 2.5, 1.5, 0.5];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.fill_between(
                &x,
                &y1,
                &y2,
                FillBetweenStyle::new()
                    .color(Color::hex("#2ca02c"))
                    .alpha(0.3),
            )
            .axes(
                AxesStyle::new()
                    .title("Fill between")
                    .x_label("x")
                    .y_label("y"),
            );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn render_image_viridis(path: &Path) -> Result<()> {
    let data = vec![
        0.0, 0.5, 1.0, 0.25, 0.75, 1.0, 0.0, 0.5, 1.0,
    ];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.image(
                data,
                3,
                3,
                ImageStyle::new()
                    .extent(0.0, 3.0, 0.0, 3.0)
                    .colormap(Colormap::Viridis),
            )
            .axes(
                AxesStyle::new()
                    .title("Image (viridis)")
                    .x_label("x")
                    .y_label("y"),
            );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn render_contour(path: &Path) -> Result<()> {
    let width = 4usize;
    let height = 4usize;
    let mut data = Vec::with_capacity(width * height);
    for row in 0..height {
        for col in 0..width {
            data.push(0.25 * (col as f64 + row as f64));
        }
    }

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.contour(
                data,
                width,
                height,
                ContourStyle::new()
                    .extent(0.0, 3.0, 0.0, 3.0)
                    .levels(&[0.25, 0.5, 0.75])
                    .line_color(Color::hex("#1f77b4")),
            )
            .axes(
                AxesStyle::new()
                    .title("Contour plot")
                    .x_label("x")
                    .y_label("y"),
            );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn render_line_log_x(path: &Path) -> Result<()> {
    let x = [1.0, 10.0, 100.0, 1000.0];
    let y = [1.0, 2.0, 3.0, 4.0];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(
                &x,
                &y,
                LineStyle::new().color(Color::hex("#1f77b4")),
            )
            .axes(
                AxesStyle::new()
                    .title("Log x line plot")
                    .x_label("x")
                    .y_label("y")
                    .x_scale(Scale::Log),
            );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn render_line_log_log(path: &Path) -> Result<()> {
    let x = [1.0, 10.0, 100.0];
    let y = [10.0, 100.0, 1000.0];

    Figure::builder()
        .panel(GridPos::new(1, 1, 1), |p| {
            p.line(
                &x,
                &y,
                LineStyle::new().color(Color::hex("#ff7f0e")),
            )
            .axes(
                AxesStyle::new()
                    .title("Log-log line plot")
                    .x_label("x")
                    .y_label("y")
                    .x_scale(Scale::Log)
                    .y_scale(Scale::Log),
            );
        })
        .build()?
        .save(path, fidelity_save_options())
}

fn compare_pngs(actual: &Path, golden: &Path, max_mean_delta: f64) -> std::result::Result<(), String> {
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
    render: fn(&Path) -> Result<()>,
    max_mean_delta: f64,
) -> std::result::Result<(), String> {
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

#[test]
fn fidelity_bar_chart() {
    fidelity_case("bar_chart.png", render_bar_chart, 35.0).unwrap();
}

#[test]
fn fidelity_histogram() {
    fidelity_case("histogram.png", render_histogram, 35.0).unwrap();
}

#[test]
fn fidelity_fill_between() {
    fidelity_case("fill_between.png", render_fill_between, 35.0).unwrap();
}

#[test]
fn fidelity_image_viridis() {
    fidelity_case("image_viridis.png", render_image_viridis, 40.0).unwrap();
}

#[test]
fn fidelity_contour() {
    fidelity_case("contour.png", render_contour, 40.0).unwrap();
}

#[test]
fn fidelity_line_log_x() {
    fidelity_case("line_log_x.png", render_line_log_x, 40.0).unwrap();
}

#[test]
fn fidelity_line_log_log() {
    fidelity_case("line_log_log.png", render_line_log_log, 40.0).unwrap();
}
