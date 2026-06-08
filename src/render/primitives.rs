use plotters::coord::types::RangedCoordf64;
use plotters::element::{Circle, PathElement, Rectangle, Text};
use plotters::prelude::*;
use plotters::series::LineSeries as PlotLineSeries;
use plotters::style::{Color, RGBColor, ShapeStyle};

use crate::axes::PanelScales;
use crate::colormap::{Colormap, Normalize};
use crate::render::model::{
    BarSeries, BoxplotSeries, CompiledSeries, ContourSeries, FillBetweenSeries, HistSeries,
    ImageSeries, LineSeries, TextSeries,
};
use crate::render::mpl_style::{
    marker_radius_px, stroke_width_px, MPL_FONT, MPL_LINE_WIDTH, MPL_MARKER_SIZE,
};
use crate::render::scene::{box_positions, box_stats, default_box_width};
use crate::series::{LineDash, Marker};

type Chart<'a, DB> = ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

pub fn draw_series<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    series: &CompiledSeries,
    scales: PanelScales,
    tick_fontsize_px: f64,
    dpi: u32,
) -> Result<(), &'static str> {
    match series {
        CompiledSeries::Line(curve) => draw_curve(chart, curve, scales, dpi),
        CompiledSeries::Boxplot(boxes) => draw_boxplot(chart, boxes, scales, dpi),
        CompiledSeries::Bar(bar) => draw_bar(chart, bar, scales, dpi),
        CompiledSeries::Histogram(hist) => draw_histogram(chart, hist, scales, dpi),
        CompiledSeries::FillBetween(fill) => draw_fill_between(chart, fill, scales, dpi),
        CompiledSeries::Image(image) => draw_image(chart, image, scales),
        CompiledSeries::Contour(contour) => draw_contour(chart, contour, scales, dpi),
        CompiledSeries::Text(text) => draw_text(chart, text, scales, tick_fontsize_px),
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

    let rgb = curve.color.to_rgb();
    let width = if curve.width > 0.0 { curve.width } else { MPL_LINE_WIDTH };
    let stroke_width = stroke_width_px(width, dpi);
    let style = ShapeStyle {
        color: rgb.to_rgba(),
        filled: false,
        stroke_width,
    };

    if points.len() >= 2 {
        match curve.dash {
            LineDash::Dashed => chart
                .draw_series(DashedLineSeries::new(points.clone(), 5, 5, style))
                .map_err(|_| "failed to draw curve")?,
            LineDash::DashDot => chart
                .draw_series(DashedLineSeries::new(points.clone(), 8, 4, style))
                .map_err(|_| "failed to draw curve")?,
            LineDash::Dotted => chart
                .draw_series(DashedLineSeries::new(points.clone(), 1, 3, style))
                .map_err(|_| "failed to draw curve")?,
            LineDash::Solid => chart
                .draw_series(PlotLineSeries::new(points.clone(), style))
                .map_err(|_| "failed to draw curve")?,
        };
    }
    draw_markers(chart, &points, curve.marker, rgb, dpi)
}

fn draw_markers<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    points: &[(f64, f64)],
    marker: Marker,
    color: RGBColor,
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
        match marker {
            Marker::None => {}
            Marker::Circle => {
                chart
                    .draw_series(std::iter::once(Circle::new((x, y), size, style)))
                    .map_err(|_| "failed to draw marker")?;
            }
            Marker::Square => {
                let half = size as f64;
                chart
                    .draw_series(std::iter::once(PathElement::new(
                        vec![
                            (x - half, y - half),
                            (x + half, y - half),
                            (x + half, y + half),
                            (x - half, y + half),
                            (x - half, y - half),
                        ],
                        style,
                    )))
                    .map_err(|_| "failed to draw marker")?;
            }
            Marker::Cross => {
                let half = size as f64;
                chart
                    .draw_series([
                        PathElement::new(vec![(x - half, y - half), (x + half, y + half)], style),
                        PathElement::new(vec![(x - half, y + half), (x + half, y - half)], style),
                    ])
                    .map_err(|_| "failed to draw marker")?;
            }
        }
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
        let (y0, y1) = if y_base <= y_val { (y_base, y_val) } else { (y_val, y_base) };
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
        let y1 = scales.y.data_to_axis(*count as f64);
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
    let edges: Vec<f64> = (0..=bins)
        .map(|i| min + step * i as f64)
        .collect();
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
    let rgb = fill.color.to_rgb();
    chart
        .draw_series(std::iter::once(PathElement::new(
            polygon,
            ShapeStyle {
                color: rgb.to_rgba(),
                filled: true,
                stroke_width: 0,
            },
        )))
        .map_err(|_| "failed to draw fill_between")?;
    Ok(())
}

fn draw_image<DB: DrawingBackend>(chart: &mut Chart<'_, DB>, image: &ImageSeries, scales: PanelScales) -> Result<(), &'static str> {
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
    if z00 >= level { case |= 1; }
    if z10 >= level { case |= 2; }
    if z11 >= level { case |= 4; }
    if z01 >= level { case |= 8; }

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
    let (x, y) = scales.map_xy(text.x, text.y);
    let size = if text.fontsize > 0.0 {
        text.fontsize
    } else {
        tick_fontsize_px
    };
    let label = text.text.clone();
    chart
        .draw_series(std::iter::once(Text::new(
            label,
            (x, y),
            (MPL_FONT, size.round() as i32),
        )))
        .map_err(|_| "failed to draw text")?;
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
                chart, scales, pos, width, cap_width, &stats, boxes, edge, fill, median_color,
                edge_style, flier_size, dpi, thin_stroke,
            )?;
        } else {
            draw_vertical_box(
                chart, scales, pos, width, cap_width, &stats, boxes, edge, fill, median_color,
                edge_style, flier_size, dpi, thin_stroke,
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

    draw_box_rect(chart, boxes.patch_artist, x0, x1, y_q1, y_q3, fill, edge_style, thin_stroke)?;
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

    draw_box_rect(chart, boxes.patch_artist, x_q1, x_q3, y0, y1, fill, edge_style, thin_stroke)?;
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
}
