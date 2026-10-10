use plotters::coord::types::RangedCoordf64;
use plotters::element::{Circle, EmptyElement, PathElement, Polygon, Rectangle, Text};
use plotters::prelude::*;
use plotters::series::LineSeries as PlotLineSeries;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use plotters::style::{Color, RGBColor, ShapeStyle, TextStyle};

use crate::axes::PanelScales;
use crate::colormap::{Colormap, Normalize};
use crate::render::model::{
    AnnotationSeries, AxHLineSeries, AxHSpanSeries, AxVLineSeries, AxVSpanSeries, BarHSeries,
    BarSeries, BoxplotSeries, CompiledSeries, ContourSeries, ErrorBarSeries, FillBetweenSeries,
    HistSeries, ImageSeries, LineSeries, ScatterSeries, StepSeries, TextSeries, ViolinSeries,
};
use crate::render::mpl_style::{
    marker_radius_px, stroke_width_px, MPL_FONT, MPL_LINE_WIDTH, MPL_MARKER_SIZE,
};
use crate::render::scene::{box_positions, box_stats, default_box_width, violin_stats};
use crate::series::{HorizontalAlignment, LineDash, Marker, StepWhere, VerticalAlignment};

type Chart<'a, DB> = ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

/// Axis-space rectangle used to clip series so artists cannot paint over spines.
#[derive(Clone, Copy, Debug)]
pub struct ClipRect {
    pub xmin: f64,
    pub xmax: f64,
    pub ymin: f64,
    pub ymax: f64,
}

pub fn draw_series<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    series: &CompiledSeries,
    scales: PanelScales,
    tick_fontsize_px: f64,
    dpi: u32,
    clip: ClipRect,
) -> Result<(), &'static str> {
    match series {
        CompiledSeries::Line(curve) => draw_curve(chart, curve, scales, dpi, clip),
        CompiledSeries::Scatter(scatter) => draw_scatter(chart, scatter, scales, dpi, clip),
        CompiledSeries::ErrorBar(err) => draw_errorbar(chart, err, scales, dpi, clip),
        CompiledSeries::Step(step) => draw_step(chart, step, scales, dpi, clip),
        CompiledSeries::Boxplot(boxes) => draw_boxplot(chart, boxes, scales, dpi),
        CompiledSeries::Violin(violin) => draw_violin(chart, violin, scales, dpi),
        CompiledSeries::Bar(bar) => draw_bar(chart, bar, scales, dpi),
        CompiledSeries::BarH(barh) => draw_barh(chart, barh, scales, dpi),
        CompiledSeries::Histogram(hist) => draw_histogram(chart, hist, scales, dpi),
        CompiledSeries::FillBetween(fill) => draw_fill_between(chart, fill, scales, dpi),
        CompiledSeries::Image(image) => draw_image(chart, image, scales),
        CompiledSeries::Contour(contour) => draw_contour(chart, contour, scales, dpi),
        CompiledSeries::Text(text) => draw_text(chart, text, scales, tick_fontsize_px),
        CompiledSeries::Annotation(ann) => {
            draw_annotation(chart, ann, scales, tick_fontsize_px, dpi)
        }
        CompiledSeries::AxHLine(line) => draw_axhline(chart, line, scales, dpi, clip),
        CompiledSeries::AxVLine(line) => draw_axvline(chart, line, scales, dpi, clip),
        CompiledSeries::AxHSpan(span) => draw_axhspan(chart, span, scales, dpi, clip),
        CompiledSeries::AxVSpan(span) => draw_axvspan(chart, span, scales, dpi, clip),
    }
}

pub fn draw_colorbar<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    normalize: Normalize,
    colormap: Colormap,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    dpi: u32,
) -> Result<(), &'static str> {
    let x_span = (xmax - xmin).max(1e-12);
    let y_span = (ymax - ymin).max(1e-12);
    let bar_w = x_span * 0.025;
    let x0 = xmax - bar_w * 3.0;
    let x1 = xmax - bar_w;
    let steps = 48usize;
    for step in 0..steps {
        let t0 = step as f64 / steps as f64;
        let t1 = (step + 1) as f64 / steps as f64;
        let y0 = ymin + t0 * y_span;
        let y1 = ymin + t1 * y_span;
        let color = colormap.map(t1).to_rgb();
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x0, y0), (x1, y1)],
                ShapeStyle {
                    color: color.to_rgba(),
                    filled: true,
                    stroke_width: 0,
                },
            )))
            .map_err(|_| "failed to draw colorbar segment")?;
    }
    let edge = RGBColor(0, 0, 0);
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(x0, ymin), (x1, ymin), (x1, ymax), (x0, ymax), (x0, ymin)],
            edge.stroke_width(stroke_width_px(1.0, dpi)),
        )))
        .map_err(|_| "failed to draw colorbar border")?;
    let _ = normalize;
    Ok(())
}

fn draw_curve<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    curve: &LineSeries,
    scales: PanelScales,
    dpi: u32,
    clip: ClipRect,
) -> Result<(), &'static str> {
    if curve.x.len() != curve.y.len() || curve.x.is_empty() {
        return Ok(());
    }
    let points: Vec<(f64, f64)> = curve
        .x
        .iter()
        .zip(curve.y.iter())
        .filter(|(x, y)| scales.x.usable(**x) && scales.y.usable(**y))
        .map(|(x, y)| scales.map_xy(*x, *y))
        .collect();
    if points.is_empty() {
        return Ok(());
    }

    let width = if curve.width > 0.0 {
        curve.width
    } else {
        MPL_LINE_WIDTH
    };
    let stroke_width = stroke_width_px(width, dpi);
    // Use true RGBA so overlapping traces darken (per-line alpha, not baked gray).
    let style = ShapeStyle {
        color: curve.color.to_rgba(),
        filled: false,
        stroke_width,
    };

    // Clip to axes so out-of-range traces cannot paint over spines/labels
    // (matplotlib default clip_on=True).
    let clipped = clip_polyline(&points, clip);
    for segment in &clipped {
        if segment.len() < 2 {
            continue;
        }
        match curve.dash {
            LineDash::Dashed => chart
                .draw_series(DashedLineSeries::new(segment.clone(), 5, 5, style))
                .map_err(|_| "failed to draw curve")?,
            LineDash::DashDot => chart
                .draw_series(DashedLineSeries::new(segment.clone(), 8, 4, style))
                .map_err(|_| "failed to draw curve")?,
            LineDash::Dotted => chart
                .draw_series(DashedLineSeries::new(segment.clone(), 1, 3, style))
                .map_err(|_| "failed to draw curve")?,
            LineDash::Solid => chart
                .draw_series(PlotLineSeries::new(segment.clone(), style))
                .map_err(|_| "failed to draw curve")?,
        };
    }
    let markers: Vec<(f64, f64)> = points
        .into_iter()
        .filter(|(x, y)| point_in_clip(*x, *y, clip))
        .collect();
    draw_markers(chart, &markers, curve.marker, curve.color, dpi)
}

fn point_in_clip(x: f64, y: f64, clip: ClipRect) -> bool {
    x >= clip.xmin && x <= clip.xmax && y >= clip.ymin && y <= clip.ymax
}

/// Liang–Barsky segment clip against an axis-aligned rectangle.
fn clip_segment(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    clip: ClipRect,
) -> Option<((f64, f64), (f64, f64))> {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut t0 = 0.0;
    let mut t1 = 1.0;
    let edges = [
        (-dx, x0 - clip.xmin),
        (dx, clip.xmax - x0),
        (-dy, y0 - clip.ymin),
        (dy, clip.ymax - y0),
    ];
    for (p, q) in edges {
        if p.abs() < f64::EPSILON {
            if q < 0.0 {
                return None;
            }
            continue;
        }
        let r = q / p;
        if p < 0.0 {
            if r > t1 {
                return None;
            }
            if r > t0 {
                t0 = r;
            }
        } else {
            if r < t0 {
                return None;
            }
            if r < t1 {
                t1 = r;
            }
        }
    }
    Some(((x0 + t0 * dx, y0 + t0 * dy), (x0 + t1 * dx, y0 + t1 * dy)))
}

/// Clip a polyline to the axes box; may split into multiple visible segments.
fn clip_polyline(points: &[(f64, f64)], clip: ClipRect) -> Vec<Vec<(f64, f64)>> {
    if points.is_empty() {
        return Vec::new();
    }
    let mut out: Vec<Vec<(f64, f64)>> = Vec::new();
    let mut current: Vec<(f64, f64)> = Vec::new();

    let push_current = |current: &mut Vec<(f64, f64)>, out: &mut Vec<Vec<(f64, f64)>>| {
        if current.len() >= 2 {
            out.push(std::mem::take(current));
        } else {
            current.clear();
        }
    };

    for window in points.windows(2) {
        let (x0, y0) = window[0];
        let (x1, y1) = window[1];
        let Some(((cx0, cy0), (cx1, cy1))) = clip_segment(x0, y0, x1, y1, clip) else {
            push_current(&mut current, &mut out);
            continue;
        };
        if current.is_empty() {
            current.push((cx0, cy0));
            current.push((cx1, cy1));
        } else {
            let last = *current.last().unwrap();
            // Continue segment if this piece attaches to the open polyline.
            if (last.0 - cx0).abs() <= 1e-9 && (last.1 - cy0).abs() <= 1e-9 {
                current.push((cx1, cy1));
            } else {
                push_current(&mut current, &mut out);
                current.push((cx0, cy0));
                current.push((cx1, cy1));
            }
        }
    }
    push_current(&mut current, &mut out);
    out
}

pub(crate) fn draw_marker_shape<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    x: f64,
    y: f64,
    size: i32,
    style: ShapeStyle,
    marker: Marker,
) -> Result<(), &'static str> {
    let s = size;
    match marker {
        Marker::None => {}
        Marker::Circle => {
            chart
                .draw_series(std::iter::once(Circle::new((x, y), size, style)))
                .map_err(|_| "failed to draw marker")?;
        }
        Marker::Square => {
            let pts = vec![(-s, -s), (s, -s), (s, s), (-s, s)];
            if style.filled {
                chart
                    .draw_series(std::iter::once(
                        EmptyElement::at((x, y)) + Polygon::new(pts, style),
                    ))
                    .map_err(|_| "failed to draw marker")?;
            } else {
                let mut stroke_pts = pts;
                stroke_pts.push((-s, -s));
                chart
                    .draw_series(std::iter::once(
                        EmptyElement::at((x, y)) + PathElement::new(stroke_pts, style),
                    ))
                    .map_err(|_| "failed to draw marker")?;
            }
        }
        Marker::Cross => {
            chart
                .draw_series([
                    EmptyElement::at((x, y)) + PathElement::new(vec![(-s, -s), (s, s)], style),
                    EmptyElement::at((x, y)) + PathElement::new(vec![(-s, s), (s, -s)], style),
                ])
                .map_err(|_| "failed to draw marker")?;
        }
        Marker::Plus => {
            chart
                .draw_series([
                    EmptyElement::at((x, y)) + PathElement::new(vec![(-s, 0), (s, 0)], style),
                    EmptyElement::at((x, y)) + PathElement::new(vec![(0, -s), (0, s)], style),
                ])
                .map_err(|_| "failed to draw marker")?;
        }
        Marker::TriangleUp => {
            let h_top = (s as f64 * 1.2).round() as i32;
            let h_bot = (s as f64 * 0.7).round() as i32;
            let pts = vec![(0, -h_top), (s, h_bot), (-s, h_bot)];
            if style.filled {
                chart
                    .draw_series(std::iter::once(
                        EmptyElement::at((x, y)) + Polygon::new(pts, style),
                    ))
                    .map_err(|_| "failed to draw marker")?;
            } else {
                let mut stroke_pts = pts;
                stroke_pts.push((0, -h_top));
                chart
                    .draw_series(std::iter::once(
                        EmptyElement::at((x, y)) + PathElement::new(stroke_pts, style),
                    ))
                    .map_err(|_| "failed to draw marker")?;
            }
        }
        Marker::TriangleDown => {
            let h_bot = (s as f64 * 1.2).round() as i32;
            let h_top = (s as f64 * 0.7).round() as i32;
            let pts = vec![(0, h_bot), (s, -h_top), (-s, -h_top)];
            if style.filled {
                chart
                    .draw_series(std::iter::once(
                        EmptyElement::at((x, y)) + Polygon::new(pts, style),
                    ))
                    .map_err(|_| "failed to draw marker")?;
            } else {
                let mut stroke_pts = pts;
                stroke_pts.push((0, h_bot));
                chart
                    .draw_series(std::iter::once(
                        EmptyElement::at((x, y)) + PathElement::new(stroke_pts, style),
                    ))
                    .map_err(|_| "failed to draw marker")?;
            }
        }
        Marker::Diamond => {
            let sy = (s as f64 * 1.3).round() as i32;
            let sx = (s as f64 * 0.9).round() as i32;
            let pts = vec![(0, -sy), (sx, 0), (0, sy), (-sx, 0)];
            if style.filled {
                chart
                    .draw_series(std::iter::once(
                        EmptyElement::at((x, y)) + Polygon::new(pts, style),
                    ))
                    .map_err(|_| "failed to draw marker")?;
            } else {
                let mut stroke_pts = pts;
                stroke_pts.push((0, -sy));
                chart
                    .draw_series(std::iter::once(
                        EmptyElement::at((x, y)) + PathElement::new(stroke_pts, style),
                    ))
                    .map_err(|_| "failed to draw marker")?;
            }
        }
        Marker::Star => {
            let mut pts = Vec::with_capacity(10);
            let r_outer = s as f64 * 1.3;
            let r_inner = s as f64 * 0.52;
            for i in 0..10 {
                let angle = (i as f64) * std::f64::consts::PI / 5.0 - std::f64::consts::FRAC_PI_2;
                let r = if i % 2 == 0 { r_outer } else { r_inner };
                pts.push((
                    (r * angle.cos()).round() as i32,
                    (r * angle.sin()).round() as i32,
                ));
            }
            if style.filled {
                chart
                    .draw_series(std::iter::once(
                        EmptyElement::at((x, y)) + Polygon::new(pts, style),
                    ))
                    .map_err(|_| "failed to draw marker")?;
            } else {
                let mut stroke_pts = pts;
                if let Some(first) = stroke_pts.first().copied() {
                    stroke_pts.push(first);
                }
                chart
                    .draw_series(std::iter::once(
                        EmptyElement::at((x, y)) + PathElement::new(stroke_pts, style),
                    ))
                    .map_err(|_| "failed to draw marker")?;
            }
        }
    }
    Ok(())
}

fn draw_markers<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    points: &[(f64, f64)],
    marker: Marker,
    color: crate::color::Color,
    dpi: u32,
) -> Result<(), &'static str> {
    if matches!(marker, Marker::None) {
        return Ok(());
    }
    let size = marker_radius_px(MPL_MARKER_SIZE, dpi);
    let style = ShapeStyle {
        color: color.to_rgba(),
        filled: false,
        stroke_width: stroke_width_px(1.0, dpi),
    };
    for &(x, y) in points {
        draw_marker_shape(chart, x, y, size, style, marker)?;
    }
    Ok(())
}

fn draw_bar<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    bar: &BarSeries,
    scales: PanelScales,
    dpi: u32,
) -> Result<(), &'static str> {
    let rgb = bar.color.to_rgb();
    let half = bar.width / 2.0;
    let y_base = scales.y.data_to_axis(bar.baseline);
    for (x, height) in bar.x.iter().zip(bar.heights.iter()) {
        let x0 = *x - half;
        let x1 = *x + half;
        let y_val = scales.y.data_to_axis(*height);
        let (y0, y1) = if y_base <= y_val {
            (y_base, y_val)
        } else {
            (y_val, y_base)
        };
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x0, y0), (x1, y1)],
                ShapeStyle {
                    color: rgb.to_rgba(),
                    filled: true,
                    stroke_width: stroke_width_px(1.0, dpi),
                },
            )))
            .map_err(|_| "failed to draw bar")?;
    }
    Ok(())
}

fn draw_histogram<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    hist: &HistSeries,
    scales: PanelScales,
    dpi: u32,
) -> Result<(), &'static str> {
    let (edges, counts) = histogram_bins(&hist.data, hist.bins);
    if counts.is_empty() {
        return Ok(());
    }
    let rgb = hist.color.to_rgb();
    let edge = RGBColor(0, 0, 0);
    for (count, window) in counts.iter().zip(edges.windows(2)) {
        let x0 = window[0];
        let x1 = window[1];
        let y0 = scales.y.data_to_axis(0.0);
        let y1 = scales.y.data_to_axis(*count);
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x0, y0.min(y1)), (x1, y0.max(y1))],
                ShapeStyle {
                    color: rgb.to_rgba(),
                    filled: true,
                    stroke_width: stroke_width_px(1.0, dpi),
                },
            )))
            .map_err(|_| "failed to draw histogram bar")?;
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![
                    (x0, y0.min(y1)),
                    (x1, y0.min(y1)),
                    (x1, y0.max(y1)),
                    (x0, y0.max(y1)),
                    (x0, y0.min(y1)),
                ],
                edge.stroke_width(stroke_width_px(1.0, dpi)),
            )))
            .map_err(|_| "failed to draw histogram edge")?;
    }
    Ok(())
}

fn histogram_bins(data: &[f64], bins: usize) -> (Vec<f64>, Vec<f64>) {
    let mut values: Vec<f64> = data.iter().copied().filter(|v| v.is_finite()).collect();
    if values.is_empty() {
        return (Vec::new(), Vec::new());
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let min = *values.first().unwrap();
    let max = *values.last().unwrap();
    if (max - min).abs() < 1e-12 {
        let edges = vec![min, min + 1.0];
        return (edges, vec![values.len() as f64]);
    }
    let step = (max - min) / bins as f64;
    let edges: Vec<f64> = (0..=bins).map(|i| min + step * i as f64).collect();
    let mut counts = vec![0.0; bins];
    for value in values {
        let mut idx = ((value - min) / step).floor() as usize;
        if idx >= bins {
            idx = bins - 1;
        }
        counts[idx] += 1.0;
    }
    (edges, counts)
}

fn draw_fill_between<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    fill: &FillBetweenSeries,
    scales: PanelScales,
    _dpi: u32,
) -> Result<(), &'static str> {
    if fill.x.len() != fill.y1.len() || fill.x.len() != fill.y2.len() || fill.x.is_empty() {
        return Ok(());
    }
    let mut polygon = Vec::with_capacity(fill.x.len() * 2);
    for (x, y) in fill.x.iter().zip(fill.y1.iter()) {
        if scales.x.usable(*x) && scales.y.usable(*y) {
            polygon.push(scales.map_xy(*x, *y));
        }
    }
    for (x, y) in fill.x.iter().zip(fill.y2.iter()).rev() {
        if scales.x.usable(*x) && scales.y.usable(*y) {
            polygon.push(scales.map_xy(*x, *y));
        }
    }
    if polygon.len() < 3 {
        return Ok(());
    }
    polygon.push(polygon[0]);
    // Prefer explicit fill.alpha when set; otherwise color's own alpha channel.
    let fill_color = if fill.alpha < 1.0 {
        fill.color.with_alpha(fill.alpha)
    } else {
        fill.color
    };
    chart
        .draw_series(std::iter::once(PathElement::new(
            polygon,
            ShapeStyle {
                color: fill_color.to_rgba(),
                filled: true,
                stroke_width: 0,
            },
        )))
        .map_err(|_| "failed to draw fill_between")?;
    Ok(())
}

fn draw_image<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    image: &ImageSeries,
    scales: PanelScales,
) -> Result<(), &'static str> {
    let (x0, x1, y0, y1) = image.extent;
    let ax_x0 = scales.x.data_to_axis(x0);
    let ax_x1 = scales.x.data_to_axis(x1);
    let ax_y0 = scales.y.data_to_axis(y0);
    let ax_y1 = scales.y.data_to_axis(y1);
    let dx = (ax_x1 - ax_x0) / image.width as f64;
    let dy = (ax_y1 - ax_y0) / image.height as f64;

    for row in 0..image.height {
        for col in 0..image.width {
            let idx = row * image.width + col;
            let Some(value) = image.data.get(idx).copied() else {
                continue;
            };
            if !value.is_finite() {
                continue;
            }
            let t = image.normalize.apply(value);
            let color = image.colormap.map(t).to_rgb();
            let px0 = ax_x0 + col as f64 * dx;
            let px1 = px0 + dx;
            let py0 = ax_y0 + row as f64 * dy;
            let py1 = py0 + dy;
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(px0, py0), (px1, py1)],
                    ShapeStyle {
                        color: color.to_rgba(),
                        filled: true,
                        stroke_width: 0,
                    },
                )))
                .map_err(|_| "failed to draw image cell")?;
        }
    }
    Ok(())
}

fn draw_contour<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    contour: &ContourSeries,
    scales: PanelScales,
    dpi: u32,
) -> Result<(), &'static str> {
    let (x0, x1, y0, y1) = contour.extent;
    let w = contour.width;
    let h = contour.height;
    if w < 2 || h < 2 {
        return Ok(());
    }

    let value_at = |col: usize, row: usize| -> f64 {
        contour.data.get(row * w + col).copied().unwrap_or(f64::NAN)
    };

    for &level in &contour.levels {
        draw_contour_level(
            chart, contour, scales, x0, x1, y0, y1, w, h, value_at, level, dpi,
        )?;
    }
    Ok(())
}

fn draw_contour_level<DB: DrawingBackend, F>(
    chart: &mut Chart<'_, DB>,
    contour: &ContourSeries,
    scales: PanelScales,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    w: usize,
    h: usize,
    value_at: F,
    level: f64,
    dpi: u32,
) -> Result<(), &'static str>
where
    F: Fn(usize, usize) -> f64,
{
    let rgb = contour.line_color.to_rgb();
    let style = ShapeStyle {
        color: rgb.to_rgba(),
        filled: false,
        stroke_width: stroke_width_px(MPL_LINE_WIDTH, dpi),
    };

    for row in 0..(h - 1) {
        for col in 0..(w - 1) {
            let z00 = value_at(col, row);
            let z10 = value_at(col + 1, row);
            let z01 = value_at(col, row + 1);
            let z11 = value_at(col + 1, row + 1);
            let segments = marching_squares_segments(z00, z10, z11, z01, level);
            for [(fx0, fy0), (fx1, fy1)] in segments {
                let dx = (x1 - x0) / (w - 1) as f64;
                let dy = (y1 - y0) / (h - 1) as f64;
                let px0 = x0 + (col as f64 + fx0) * dx;
                let py0 = y0 + (row as f64 + fy0) * dy;
                let px1 = x0 + (col as f64 + fx1) * dx;
                let py1 = y0 + (row as f64 + fy1) * dy;
                let p0 = scales.map_xy(px0, py0);
                let p1 = scales.map_xy(px1, py1);
                chart
                    .draw_series(std::iter::once(PathElement::new(vec![p0, p1], style)))
                    .map_err(|_| "failed to draw contour segment")?;
            }
        }
    }
    Ok(())
}

fn marching_squares_segments(
    z00: f64,
    z10: f64,
    z11: f64,
    z01: f64,
    level: f64,
) -> Vec<[(f64, f64); 2]> {
    let mut segments = Vec::new();
    let mut case = 0usize;
    if z00 >= level {
        case |= 1;
    }
    if z10 >= level {
        case |= 2;
    }
    if z11 >= level {
        case |= 4;
    }
    if z01 >= level {
        case |= 8;
    }

    let interp = |a: f64, b: f64, t: f64| -> f64 {
        if (b - a).abs() < 1e-12 {
            0.5
        } else {
            ((t - a) / (b - a)).clamp(0.0, 1.0)
        }
    };

    let e_bottom = interp(z00, z10, level);
    let e_top = interp(z01, z11, level);
    let e_left = interp(z00, z01, level);
    let e_right = interp(z10, z11, level);

    match case {
        1 | 14 => segments.push([(e_left, 0.0), (e_bottom, 0.0)]),
        2 | 13 => segments.push([(e_bottom, 0.0), (1.0, e_right)]),
        3 | 12 => segments.push([(e_left, 0.0), (1.0, e_right)]),
        4 | 11 => segments.push([(1.0, e_right), (1.0, e_top)]),
        5 => {
            segments.push([(e_left, 0.0), (e_bottom, 0.0)]);
            segments.push([(1.0, e_right), (1.0, e_top)]);
        }
        6 | 9 => segments.push([(e_bottom, 0.0), (1.0, e_top)]),
        7 | 8 => segments.push([(e_left, 0.0), (1.0, e_top)]),
        10 => {
            segments.push([(e_bottom, 0.0), (1.0, e_right)]);
            segments.push([(e_left, 0.0), (1.0, e_top)]);
        }
        _ => {}
    }
    segments
}

fn draw_text<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    text: &TextSeries,
    scales: PanelScales,
    tick_fontsize_px: f64,
) -> Result<(), &'static str> {
    if !scales.x.usable(text.x) || !scales.y.usable(text.y) {
        return Ok(());
    }
    let (x, y) = scales.map_xy(text.x, text.y);
    let size = if text.fontsize > 0.0 {
        text.fontsize
    } else {
        tick_fontsize_px
    };
    let h_pos = match text.ha {
        HorizontalAlignment::Left => HPos::Left,
        HorizontalAlignment::Center => HPos::Center,
        HorizontalAlignment::Right => HPos::Right,
    };
    let v_pos = match text.va {
        VerticalAlignment::Top => VPos::Top,
        VerticalAlignment::Center => VPos::Center,
        VerticalAlignment::Bottom => VPos::Bottom,
        VerticalAlignment::Baseline => VPos::Bottom,
    };
    let rgb = text.color.to_rgb();
    let rgba = rgb.to_rgba();
    let style = TextStyle::from((MPL_FONT, size.round() as i32).into_font())
        .color(&rgba)
        .pos(Pos::new(h_pos, v_pos));
    chart
        .draw_series(std::iter::once(Text::new(text.text.clone(), (x, y), style)))
        .map_err(|_| "failed to draw text")?;
    Ok(())
}

fn draw_annotation<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    ann: &AnnotationSeries,
    scales: PanelScales,
    tick_fontsize_px: f64,
    dpi: u32,
) -> Result<(), &'static str> {
    let (tx, ty) = ann.xy;
    let (sx, sy) = ann.xytext;
    if !scales.x.usable(tx) || !scales.y.usable(ty) || !scales.x.usable(sx) || !scales.y.usable(sy)
    {
        return Ok(());
    }
    let target_axis = scales.map_xy(tx, ty);
    let text_axis = scales.map_xy(sx, sy);

    if let Some(arrow) = ann.arrow {
        let p_target = chart.backend_coord(&target_axis);
        let p_source = chart.backend_coord(&text_axis);
        let dx = (p_target.0 - p_source.0) as f64;
        let dy = (p_target.1 - p_source.1) as f64;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 3.0 {
            let ux = dx / dist;
            let uy = dy / dist;
            let shrink_px = arrow.shrink_value() * dpi as f64 / 72.0;
            let head_len = arrow.head_length_value() * dpi as f64 / 72.0;
            let head_half_w = (arrow.head_width_value() / 2.0) * dpi as f64 / 72.0;

            let x0 = (p_source.0 as f64 + ux * shrink_px).round() as i32;
            let y0 = (p_source.1 as f64 + uy * shrink_px).round() as i32;
            let x1 = (p_target.0 as f64 - ux * shrink_px).round() as i32;
            let y1 = (p_target.1 as f64 - uy * shrink_px).round() as i32;

            let base_x = x1 as f64 - ux * head_len;
            let base_y = y1 as f64 - uy * head_len;
            let w1_x = (base_x - uy * head_half_w).round() as i32;
            let w1_y = (base_y + ux * head_half_w).round() as i32;
            let w2_x = (base_x + uy * head_half_w).round() as i32;
            let w2_y = (base_y - ux * head_half_w).round() as i32;

            let screen = chart.plotting_area().use_screen_coord();
            let shaft_style = ShapeStyle {
                color: arrow.color_value().to_rgba(),
                filled: false,
                stroke_width: stroke_width_px(arrow.width_value(), dpi),
            };
            let head_style = ShapeStyle {
                color: arrow.color_value().to_rgba(),
                filled: true,
                stroke_width: 0,
            };

            screen
                .draw(&PathElement::new(
                    vec![(x0, y0), (base_x.round() as i32, base_y.round() as i32)],
                    shaft_style,
                ))
                .map_err(|_| "failed to draw arrow shaft")?;
            screen
                .draw(&Polygon::new(
                    vec![(x1, y1), (w1_x, w1_y), (w2_x, w2_y)],
                    head_style,
                ))
                .map_err(|_| "failed to draw arrowhead")?;
        }
    }

    let t_style = ann.text_style;
    let text_series = TextSeries {
        x: sx,
        y: sy,
        text: ann.text.clone(),
        color: t_style.color_value(),
        fontsize: t_style.fontsize_value(),
        ha: t_style.ha_value(),
        va: t_style.va_value(),
        rotation: t_style.rotation_value(),
    };
    draw_text(chart, &text_series, scales, tick_fontsize_px)?;
    Ok(())
}

pub fn draw_violin<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    violin: &ViolinSeries,
    scales: PanelScales,
    dpi: u32,
) -> Result<(), &'static str> {
    if violin.groups.is_empty() {
        return Ok(());
    }
    let positions: Vec<f64> = if !violin.style.positions_value().is_empty() {
        violin.style.positions_value().to_vec()
    } else {
        (1..=violin.groups.len()).map(|i| i as f64).collect()
    };

    let width = violin.style.width_value();
    let vert = violin.style.vert_value();
    let pts = violin.style.points_value();
    let default_color = crate::color::Color::hex("#1f77b4");
    let face = violin.style.face_color_value().unwrap_or(default_color);
    let alpha = (face.a as f64 / 255.0) * violin.style.alpha_value();
    let fill_style = ShapeStyle {
        color: RGBAColor(face.r, face.g, face.b, alpha),
        filled: true,
        stroke_width: 0,
    };
    let edge = violin.style.edge_color_value().unwrap_or(default_color);
    let line_style = ShapeStyle {
        color: edge.to_rgba(),
        filled: false,
        stroke_width: stroke_width_px(1.5, dpi),
    };

    for (group, &pos) in violin.groups.iter().zip(positions.iter()) {
        let Some(stats) = violin_stats(group, pts) else {
            continue;
        };

        let mut poly: Vec<(f64, f64)> = Vec::with_capacity(stats.coords.len() * 2 + 1);
        if vert {
            for (&coord, &val) in stats.coords.iter().zip(stats.vals.iter()) {
                let x = scales.x.data_to_axis(pos + 0.5 * width * val);
                let y = scales.y.data_to_axis(coord);
                poly.push((x, y));
            }
            for (&coord, &val) in stats.coords.iter().zip(stats.vals.iter()).rev() {
                let x = scales.x.data_to_axis(pos - 0.5 * width * val);
                let y = scales.y.data_to_axis(coord);
                poly.push((x, y));
            }
        } else {
            for (&coord, &val) in stats.coords.iter().zip(stats.vals.iter()) {
                let x = scales.x.data_to_axis(coord);
                let y = scales.y.data_to_axis(pos + 0.5 * width * val);
                poly.push((x, y));
            }
            for (&coord, &val) in stats.coords.iter().zip(stats.vals.iter()).rev() {
                let x = scales.x.data_to_axis(coord);
                let y = scales.y.data_to_axis(pos - 0.5 * width * val);
                poly.push((x, y));
            }
        }
        if !poly.is_empty() {
            chart
                .draw_series(std::iter::once(Polygon::new(poly, fill_style)))
                .map_err(|_| "failed to draw violin body")?;
        }

        let half_tick = 0.25 * width;
        let tick_lo = pos - half_tick;
        let tick_hi = pos + half_tick;

        if violin.style.show_extrema_value() {
            if vert {
                let x = scales.x.data_to_axis(pos);
                let y0 = scales.y.data_to_axis(stats.min);
                let y1 = scales.y.data_to_axis(stats.max);
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(x, y0), (x, y1)],
                        line_style,
                    )))
                    .map_err(|_| "failed to draw violin cbars")?;

                let x0 = scales.x.data_to_axis(tick_lo);
                let x1 = scales.x.data_to_axis(tick_hi);
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(x0, y0), (x1, y0)],
                        line_style,
                    )))
                    .map_err(|_| "failed to draw violin cmins")?;
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(x0, y1), (x1, y1)],
                        line_style,
                    )))
                    .map_err(|_| "failed to draw violin cmaxes")?;
            } else {
                let y = scales.y.data_to_axis(pos);
                let x0 = scales.x.data_to_axis(stats.min);
                let x1 = scales.x.data_to_axis(stats.max);
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(x0, y), (x1, y)],
                        line_style,
                    )))
                    .map_err(|_| "failed to draw violin cbars")?;

                let y0 = scales.y.data_to_axis(tick_lo);
                let y1 = scales.y.data_to_axis(tick_hi);
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(x0, y0), (x0, y1)],
                        line_style,
                    )))
                    .map_err(|_| "failed to draw violin cmins")?;
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(x1, y0), (x1, y1)],
                        line_style,
                    )))
                    .map_err(|_| "failed to draw violin cmaxes")?;
            }
        }

        if violin.style.show_means_value() {
            if vert {
                let x0 = scales.x.data_to_axis(tick_lo);
                let x1 = scales.x.data_to_axis(tick_hi);
                let y = scales.y.data_to_axis(stats.mean);
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(x0, y), (x1, y)],
                        line_style,
                    )))
                    .map_err(|_| "failed to draw violin cmeans")?;
            } else {
                let y0 = scales.y.data_to_axis(tick_lo);
                let y1 = scales.y.data_to_axis(tick_hi);
                let x = scales.x.data_to_axis(stats.mean);
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(x, y0), (x, y1)],
                        line_style,
                    )))
                    .map_err(|_| "failed to draw violin cmeans")?;
            }
        }

        if violin.style.show_medians_value() {
            if vert {
                let x0 = scales.x.data_to_axis(tick_lo);
                let x1 = scales.x.data_to_axis(tick_hi);
                let y = scales.y.data_to_axis(stats.median);
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(x0, y), (x1, y)],
                        line_style,
                    )))
                    .map_err(|_| "failed to draw violin cmedians")?;
            } else {
                let y0 = scales.y.data_to_axis(tick_lo);
                let y1 = scales.y.data_to_axis(tick_hi);
                let x = scales.x.data_to_axis(stats.median);
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(x, y0), (x, y1)],
                        line_style,
                    )))
                    .map_err(|_| "failed to draw violin cmedians")?;
            }
        }
    }

    Ok(())
}

// Boxplot drawing moved here from png.rs for reuse.
use crate::render::mpl_style::{
    MPL_BOX_EDGE, MPL_BOX_FACE, MPL_BOX_LINE_WIDTH, MPL_FLIER_SIZE, MPL_MEDIAN,
    MPL_MEDIAN_LINE_WIDTH, MPL_WHISKER_LINE_WIDTH,
};

pub fn draw_boxplot<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    boxes: &BoxplotSeries,
    scales: PanelScales,
    dpi: u32,
) -> Result<(), &'static str> {
    let whisker = boxes.whisker;
    let positions = box_positions(boxes);
    let width = boxes.width.unwrap_or_else(|| default_box_width(&positions));
    let cap_width = 0.5 * width;
    let edge = RGBColor(MPL_BOX_EDGE.0, MPL_BOX_EDGE.1, MPL_BOX_EDGE.2);
    let fill = RGBColor(MPL_BOX_FACE.0, MPL_BOX_FACE.1, MPL_BOX_FACE.2);
    let median_color = RGBColor(MPL_MEDIAN.0, MPL_MEDIAN.1, MPL_MEDIAN.2);
    let flier_size = marker_radius_px(MPL_FLIER_SIZE, dpi);
    let thin_stroke = stroke_width_px(1.0, dpi);

    for (idx, group) in boxes.groups.iter().enumerate() {
        let pos = positions.get(idx).copied().unwrap_or((idx + 1) as f64);
        let Some(stats) = box_stats(group, whisker) else {
            continue;
        };

        let edge_style = ShapeStyle {
            color: edge.to_rgba(),
            filled: false,
            stroke_width: stroke_width_px(MPL_BOX_LINE_WIDTH, dpi),
        };

        if boxes.horizontal {
            draw_horizontal_box(
                chart,
                scales,
                pos,
                width,
                cap_width,
                &stats,
                boxes,
                edge,
                fill,
                median_color,
                edge_style,
                flier_size,
                dpi,
                thin_stroke,
            )?;
        } else {
            draw_vertical_box(
                chart,
                scales,
                pos,
                width,
                cap_width,
                &stats,
                boxes,
                edge,
                fill,
                median_color,
                edge_style,
                flier_size,
                dpi,
                thin_stroke,
            )?;
        }
    }
    Ok(())
}

fn draw_vertical_box<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    scales: PanelScales,
    pos: f64,
    width: f64,
    cap_width: f64,
    stats: &crate::render::scene::BoxStats,
    boxes: &BoxplotSeries,
    edge: RGBColor,
    fill: RGBColor,
    median_color: RGBColor,
    edge_style: ShapeStyle,
    flier_size: i32,
    dpi: u32,
    thin_stroke: u32,
) -> Result<(), &'static str> {
    let y_low = scales.y.data_to_axis(stats.whislo);
    let y_q1 = scales.y.data_to_axis(stats.q1);
    let y_med = scales.y.data_to_axis(stats.med);
    let y_q3 = scales.y.data_to_axis(stats.q3);
    let y_high = scales.y.data_to_axis(stats.whishi);
    let x0 = pos - width / 2.0;
    let x1 = pos + width / 2.0;
    let cap_x0 = pos - cap_width / 2.0;
    let cap_x1 = pos + cap_width / 2.0;

    draw_box_rect(
        chart,
        boxes.patch_artist,
        x0,
        x1,
        y_q1,
        y_q3,
        fill,
        edge_style,
        thin_stroke,
    )?;
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(x0, y_med), (x1, y_med)],
            median_color.stroke_width(stroke_width_px(MPL_MEDIAN_LINE_WIDTH, dpi)),
        )))
        .map_err(|_| "failed to draw median")?;
    let whisker_style = edge.stroke_width(stroke_width_px(MPL_WHISKER_LINE_WIDTH, dpi));
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(pos, y_low), (pos, y_q1)],
            whisker_style,
        )))
        .map_err(|_| "failed to draw lower whisker")?;
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(pos, y_q3), (pos, y_high)],
            whisker_style,
        )))
        .map_err(|_| "failed to draw upper whisker")?;
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(cap_x0, y_low), (cap_x1, y_low)],
            whisker_style,
        )))
        .map_err(|_| "failed to draw lower cap")?;
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(cap_x0, y_high), (cap_x1, y_high)],
            whisker_style,
        )))
        .map_err(|_| "failed to draw upper cap")?;
    if !boxes.no_fliers {
        for flier in &stats.fliers {
            let y = scales.y.data_to_axis(*flier);
            chart
                .draw_series(std::iter::once(Circle::new(
                    (pos, y),
                    flier_size,
                    ShapeStyle {
                        color: edge.to_rgba(),
                        filled: false,
                        stroke_width: thin_stroke,
                    },
                )))
                .map_err(|_| "failed to draw flier")?;
        }
    }
    Ok(())
}

fn draw_horizontal_box<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    scales: PanelScales,
    pos: f64,
    width: f64,
    cap_width: f64,
    stats: &crate::render::scene::BoxStats,
    boxes: &BoxplotSeries,
    edge: RGBColor,
    fill: RGBColor,
    median_color: RGBColor,
    edge_style: ShapeStyle,
    flier_size: i32,
    dpi: u32,
    thin_stroke: u32,
) -> Result<(), &'static str> {
    let x_low = scales.x.data_to_axis(stats.whislo);
    let x_q1 = scales.x.data_to_axis(stats.q1);
    let x_med = scales.x.data_to_axis(stats.med);
    let x_q3 = scales.x.data_to_axis(stats.q3);
    let x_high = scales.x.data_to_axis(stats.whishi);
    let y0 = pos - width / 2.0;
    let y1 = pos + width / 2.0;
    let cap_y0 = pos - cap_width / 2.0;
    let cap_y1 = pos + cap_width / 2.0;

    draw_box_rect(
        chart,
        boxes.patch_artist,
        x_q1,
        x_q3,
        y0,
        y1,
        fill,
        edge_style,
        thin_stroke,
    )?;
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(x_med, y0), (x_med, y1)],
            median_color.stroke_width(stroke_width_px(MPL_MEDIAN_LINE_WIDTH, dpi)),
        )))
        .map_err(|_| "failed to draw median")?;
    let whisker_style = edge.stroke_width(stroke_width_px(MPL_WHISKER_LINE_WIDTH, dpi));
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(x_low, pos), (x_q1, pos)],
            whisker_style,
        )))
        .map_err(|_| "failed to draw lower whisker")?;
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(x_q3, pos), (x_high, pos)],
            whisker_style,
        )))
        .map_err(|_| "failed to draw upper whisker")?;
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(x_low, cap_y0), (x_low, cap_y1)],
            whisker_style,
        )))
        .map_err(|_| "failed to draw lower cap")?;
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![(x_high, cap_y0), (x_high, cap_y1)],
            whisker_style,
        )))
        .map_err(|_| "failed to draw upper cap")?;
    if !boxes.no_fliers {
        for flier in &stats.fliers {
            let x = scales.x.data_to_axis(*flier);
            chart
                .draw_series(std::iter::once(Circle::new(
                    (x, pos),
                    flier_size,
                    ShapeStyle {
                        color: edge.to_rgba(),
                        filled: false,
                        stroke_width: thin_stroke,
                    },
                )))
                .map_err(|_| "failed to draw flier")?;
        }
    }
    Ok(())
}

fn draw_box_rect<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    patch_artist: bool,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    fill: RGBColor,
    edge_style: ShapeStyle,
    thin_stroke: u32,
) -> Result<(), &'static str> {
    if patch_artist {
        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x0, y0), (x1, y1)],
                ShapeStyle {
                    color: fill.to_rgba(),
                    filled: true,
                    stroke_width: thin_stroke,
                },
            )))
            .map_err(|_| "failed to draw box")?;
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1), (x0, y0)],
                edge_style,
            )))
            .map_err(|_| "failed to draw box edge")?;
    } else {
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1), (x0, y0)],
                edge_style,
            )))
            .map_err(|_| "failed to draw box")?;
    }
    Ok(())
}

fn draw_scatter<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    scatter: &ScatterSeries,
    scales: PanelScales,
    dpi: u32,
    clip: ClipRect,
) -> Result<(), &'static str> {
    if scatter.x.len() != scatter.y.len() || scatter.x.is_empty() {
        return Ok(());
    }
    let radius = (scatter.size.sqrt() * dpi as f64 / 72.0 / 2.0)
        .round()
        .max(1.0) as i32;
    let fill_style = ShapeStyle {
        color: scatter.color.to_rgba(),
        filled: true,
        stroke_width: 0,
    };
    let edge_color = scatter.edge_color.unwrap_or(scatter.color);
    let edge_style = ShapeStyle {
        color: edge_color.to_rgba(),
        filled: false,
        stroke_width: stroke_width_px(scatter.edge_width, dpi),
    };

    for (&x, &y) in scatter.x.iter().zip(scatter.y.iter()) {
        if !scales.x.usable(x) || !scales.y.usable(y) {
            continue;
        }
        let (mx, my) = scales.map_xy(x, y);
        if !point_in_clip(mx, my, clip) {
            continue;
        }
        draw_marker_shape(chart, mx, my, radius, fill_style, scatter.marker)?;
        if scatter.edge_width > 0.0 {
            draw_marker_shape(chart, mx, my, radius, edge_style, scatter.marker)?;
        }
    }
    Ok(())
}

fn draw_step<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    step: &StepSeries,
    scales: PanelScales,
    dpi: u32,
    clip: ClipRect,
) -> Result<(), &'static str> {
    if step.x.len() != step.y.len() || step.x.is_empty() {
        return Ok(());
    }
    let mut step_pts = Vec::with_capacity(step.x.len() * 2);
    for (i, (&x, &y)) in step.x.iter().zip(step.y.iter()).enumerate() {
        if i == 0 {
            step_pts.push((x, y));
        } else {
            let (prev_x, prev_y) = (step.x[i - 1], step.y[i - 1]);
            match step.where_step {
                StepWhere::Pre => {
                    step_pts.push((prev_x, y));
                    step_pts.push((x, y));
                }
                StepWhere::Post => {
                    step_pts.push((x, prev_y));
                    step_pts.push((x, y));
                }
                StepWhere::Mid => {
                    let xm = 0.5 * (prev_x + x);
                    step_pts.push((xm, prev_y));
                    step_pts.push((xm, y));
                    step_pts.push((x, y));
                }
            }
        }
    }

    let line = LineSeries {
        x: step_pts.iter().map(|p| p.0).collect(),
        y: step_pts.iter().map(|p| p.1).collect(),
        label: step.label.clone(),
        color: step.color,
        dash: step.dash,
        marker: Marker::None,
        width: step.width,
    };
    draw_curve(chart, &line, scales, dpi, clip)
}

fn draw_errorbar<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    err: &ErrorBarSeries,
    scales: PanelScales,
    dpi: u32,
    clip: ClipRect,
) -> Result<(), &'static str> {
    if err.x.len() != err.y.len() || err.x.is_empty() {
        return Ok(());
    }
    if err.line_width > 0.0 {
        let curve = LineSeries {
            x: err.x.clone(),
            y: err.y.clone(),
            label: String::new(),
            color: err.color,
            dash: err.dash,
            marker: Marker::None,
            width: err.line_width,
        };
        draw_curve(chart, &curve, scales, dpi, clip)?;
    }

    let e_style = ShapeStyle {
        color: err.ecolor.to_rgba(),
        filled: false,
        stroke_width: stroke_width_px(err.elinewidth, dpi),
    };
    let cap_style = ShapeStyle {
        color: err.ecolor.to_rgba(),
        filled: false,
        stroke_width: stroke_width_px(err.capthick, dpi),
    };
    let cap_hw = (err.capsize * dpi as f64 / 72.0 / 2.0).round().max(1.0) as i32;

    for (i, (&x, &y)) in err.x.iter().zip(err.y.iter()).enumerate() {
        if !scales.x.usable(x) || !scales.y.usable(y) {
            continue;
        }
        let (mx, my) = scales.map_xy(x, y);

        // Vertical error bar
        let y_lo = err
            .yerr_lower
            .as_ref()
            .and_then(|e| e.get(i))
            .or_else(|| err.yerr.as_ref().and_then(|e| e.get(i)))
            .copied();
        let y_hi = err.yerr.as_ref().and_then(|e| e.get(i)).copied();

        if let (Some(lo_val), Some(hi_val)) = (y_lo, y_hi) {
            let y_bottom = scales.y.data_to_axis(y - lo_val);
            let y_top = scales.y.data_to_axis(y + hi_val);
            if let Some(((bx0, by0), (bx1, by1))) = clip_segment(mx, y_bottom, mx, y_top, clip) {
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(bx0, by0), (bx1, by1)],
                        e_style,
                    )))
                    .map_err(|_| "failed to draw errorbar stem")?;
            }
            if err.capsize > 0.0 {
                if y_top >= clip.ymin && y_top <= clip.ymax && mx >= clip.xmin && mx <= clip.xmax {
                    chart
                        .draw_series(std::iter::once(
                            EmptyElement::at((mx, y_top))
                                + PathElement::new(vec![(-cap_hw, 0), (cap_hw, 0)], cap_style),
                        ))
                        .map_err(|_| "failed to draw errorbar cap")?;
                }
                if y_bottom >= clip.ymin
                    && y_bottom <= clip.ymax
                    && mx >= clip.xmin
                    && mx <= clip.xmax
                {
                    chart
                        .draw_series(std::iter::once(
                            EmptyElement::at((mx, y_bottom))
                                + PathElement::new(vec![(-cap_hw, 0), (cap_hw, 0)], cap_style),
                        ))
                        .map_err(|_| "failed to draw errorbar cap")?;
                }
            }
        }

        // Horizontal error bar
        let x_lo = err
            .xerr_lower
            .as_ref()
            .and_then(|e| e.get(i))
            .or_else(|| err.xerr.as_ref().and_then(|e| e.get(i)))
            .copied();
        let x_hi = err.xerr.as_ref().and_then(|e| e.get(i)).copied();

        if let (Some(lo_val), Some(hi_val)) = (x_lo, x_hi) {
            let x_left = scales.x.data_to_axis(x - lo_val);
            let x_right = scales.x.data_to_axis(x + hi_val);
            if let Some(((bx0, by0), (bx1, by1))) = clip_segment(x_left, my, x_right, my, clip) {
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![(bx0, by0), (bx1, by1)],
                        e_style,
                    )))
                    .map_err(|_| "failed to draw errorbar stem")?;
            }
            if err.capsize > 0.0 {
                if x_left >= clip.xmin && x_left <= clip.xmax && my >= clip.ymin && my <= clip.ymax
                {
                    chart
                        .draw_series(std::iter::once(
                            EmptyElement::at((x_left, my))
                                + PathElement::new(vec![(0, -cap_hw), (0, cap_hw)], cap_style),
                        ))
                        .map_err(|_| "failed to draw errorbar cap")?;
                }
                if x_right >= clip.xmin
                    && x_right <= clip.xmax
                    && my >= clip.ymin
                    && my <= clip.ymax
                {
                    chart
                        .draw_series(std::iter::once(
                            EmptyElement::at((x_right, my))
                                + PathElement::new(vec![(0, -cap_hw), (0, cap_hw)], cap_style),
                        ))
                        .map_err(|_| "failed to draw errorbar cap")?;
                }
            }
        }
    }

    let markers: Vec<(f64, f64)> = err
        .x
        .iter()
        .zip(err.y.iter())
        .filter(|(x, y)| scales.x.usable(**x) && scales.y.usable(**y))
        .map(|(x, y)| scales.map_xy(*x, *y))
        .filter(|(x, y)| point_in_clip(*x, *y, clip))
        .collect();
    draw_markers(chart, &markers, err.marker, err.color, dpi)
}

fn draw_barh<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    barh: &BarHSeries,
    scales: PanelScales,
    dpi: u32,
) -> Result<(), &'static str> {
    if barh.y.len() != barh.widths.len() || barh.y.is_empty() {
        return Ok(());
    }
    let stroke_width = stroke_width_px(0.8, dpi);
    let edge = RGBColor(0, 0, 0);
    let half_h = barh.height / 2.0;

    for (y_val, width) in barh.y.iter().zip(barh.widths.iter()) {
        if !scales.y.usable(*y_val - half_h) || !scales.y.usable(*y_val + half_h) {
            continue;
        }
        let (y0, y1) = (
            scales.y.data_to_axis(*y_val - half_h),
            scales.y.data_to_axis(*y_val + half_h),
        );
        let (x0, x1) = (
            scales.x.data_to_axis(barh.baseline),
            scales.x.data_to_axis(barh.baseline + *width),
        );
        let left = x0.min(x1);
        let right = x0.max(x1);
        let bottom = y0.min(y1);
        let top = y0.max(y1);

        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(left, bottom), (right, top)],
                ShapeStyle {
                    color: barh.color.to_rgba(),
                    filled: true,
                    stroke_width: 0,
                },
            )))
            .map_err(|_| "failed to draw barh rect")?;

        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(left, bottom), (right, top)],
                ShapeStyle {
                    color: edge.to_rgba(),
                    filled: false,
                    stroke_width,
                },
            )))
            .map_err(|_| "failed to draw barh border")?;
    }
    Ok(())
}

fn draw_axhline<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    line: &AxHLineSeries,
    scales: PanelScales,
    dpi: u32,
    clip: ClipRect,
) -> Result<(), &'static str> {
    if !scales.y.usable(line.y) {
        return Ok(());
    }
    let y = scales.y.data_to_axis(line.y);
    if y < clip.ymin || y > clip.ymax {
        return Ok(());
    }
    let curve = LineSeries {
        x: vec![clip.xmin, clip.xmax],
        y: vec![y, y],
        label: line.label.clone(),
        color: line.color,
        dash: line.dash,
        marker: Marker::None,
        width: line.width,
    };
    draw_curve(chart, &curve, scales, dpi, clip)
}

fn draw_axvline<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    line: &AxVLineSeries,
    scales: PanelScales,
    dpi: u32,
    clip: ClipRect,
) -> Result<(), &'static str> {
    if !scales.x.usable(line.x) {
        return Ok(());
    }
    let x = scales.x.data_to_axis(line.x);
    if x < clip.xmin || x > clip.xmax {
        return Ok(());
    }
    let curve = LineSeries {
        x: vec![x, x],
        y: vec![clip.ymin, clip.ymax],
        label: line.label.clone(),
        color: line.color,
        dash: line.dash,
        marker: Marker::None,
        width: line.width,
    };
    draw_curve(chart, &curve, scales, dpi, clip)
}

fn draw_axhspan<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    span: &AxHSpanSeries,
    scales: PanelScales,
    _dpi: u32,
    clip: ClipRect,
) -> Result<(), &'static str> {
    let y0 = scales.y.data_to_axis(span.ymin).clamp(clip.ymin, clip.ymax);
    let y1 = scales.y.data_to_axis(span.ymax).clamp(clip.ymin, clip.ymax);
    let bottom = y0.min(y1);
    let top = y0.max(y1);
    chart
        .draw_series(std::iter::once(Rectangle::new(
            [(clip.xmin, bottom), (clip.xmax, top)],
            ShapeStyle {
                color: span.color.to_rgba(),
                filled: true,
                stroke_width: 0,
            },
        )))
        .map_err(|_| "failed to draw axhspan")?;
    Ok(())
}

fn draw_axvspan<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    span: &AxVSpanSeries,
    scales: PanelScales,
    _dpi: u32,
    clip: ClipRect,
) -> Result<(), &'static str> {
    let x0 = scales.x.data_to_axis(span.xmin).clamp(clip.xmin, clip.xmax);
    let x1 = scales.x.data_to_axis(span.xmax).clamp(clip.xmin, clip.xmax);
    let left = x0.min(x1);
    let right = x0.max(x1);
    chart
        .draw_series(std::iter::once(Rectangle::new(
            [(left, clip.ymin), (right, clip.ymax)],
            ShapeStyle {
                color: span.color.to_rgba(),
                filled: true,
                stroke_width: 0,
            },
        )))
        .map_err(|_| "failed to draw axvspan")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn histogram_bins_counts_data() {
        let (edges, counts) = histogram_bins(&[1.0, 2.0, 2.5, 3.0], 2);
        assert_eq!(edges.len(), 3);
        assert_eq!(counts.len(), 2);
        assert!((counts[0] + counts[1] - 4.0).abs() < 1e-9);
    }

    #[test]
    fn clip_segment_keeps_interior() {
        let clip = ClipRect {
            xmin: 0.0,
            xmax: 10.0,
            ymin: 0.0,
            ymax: 10.0,
        };
        let ((x0, y0), (x1, y1)) = clip_segment(1.0, 1.0, 9.0, 9.0, clip).unwrap();
        assert!((x0 - 1.0).abs() < 1e-9 && (y0 - 1.0).abs() < 1e-9);
        assert!((x1 - 9.0).abs() < 1e-9 && (y1 - 9.0).abs() < 1e-9);
    }

    #[test]
    fn clip_segment_trims_outside() {
        let clip = ClipRect {
            xmin: 0.0,
            xmax: 1.0,
            ymin: 0.0,
            ymax: 1.0,
        };
        let ((x0, y0), (x1, y1)) = clip_segment(-1.0, 0.5, 2.0, 0.5, clip).unwrap();
        assert!((x0 - 0.0).abs() < 1e-9 && (y0 - 0.5).abs() < 1e-9);
        assert!((x1 - 1.0).abs() < 1e-9 && (y1 - 0.5).abs() < 1e-9);
    }

    #[test]
    fn clip_polyline_splits_on_exit() {
        let clip = ClipRect {
            xmin: 0.0,
            xmax: 1.0,
            ymin: 0.0,
            ymax: 1.0,
        };
        // Goes out of range between segments, then returns.
        let points = [(0.2, 0.2), (0.5, 0.5), (2.0, 2.0), (0.8, 0.2)];
        let segs = clip_polyline(&points, clip);
        assert!(segs.len() >= 2);
        for seg in &segs {
            for &(x, y) in seg {
                assert!(point_in_clip(x, y, clip));
            }
        }
    }
}
