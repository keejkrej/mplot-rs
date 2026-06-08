use plotters::coord::types::RangedCoordf64;
use plotters::element::{PathElement, Rectangle, Text};
use plotters::prelude::*;
use plotters::style::{Color, RGBColor, ShapeStyle};

use crate::color::Color as MplotColor;
use crate::render::model::{CompiledSeries, LineSeries};
use crate::render::mpl_style::{stroke_width_px, MPL_FONT, MPL_SPINE};
use crate::series::LineDash;

type Chart<'a, DB> = ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

#[derive(Clone, Debug, PartialEq)]
pub struct LegendEntry {
    pub label: String,
    pub color: MplotColor,
    pub dash: LineDash,
    pub width: f64,
    pub patch: bool,
}

const LEGEND_INSET_FRAC: f64 = 0.02;
const LEGEND_LINE_FRAC: f64 = 0.08;
const LEGEND_ROW_FRAC: f64 = 0.09;
const LEGEND_PAD_FRAC: f64 = 0.015;

pub fn collect_entries(series: &[CompiledSeries]) -> Vec<LegendEntry> {
    series
        .iter()
        .filter_map(|item| match item {
            CompiledSeries::Line(curve) if !curve.label.is_empty() => {
                Some(entry_from_line(curve))
            }
            CompiledSeries::Bar(bar) if !bar.label.is_empty() => Some(LegendEntry {
                label: bar.label.clone(),
                color: bar.color,
                dash: LineDash::Solid,
                width: 1.0,
                patch: true,
            }),
            CompiledSeries::Histogram(hist) if !hist.label.is_empty() => Some(LegendEntry {
                label: hist.label.clone(),
                color: hist.color,
                dash: LineDash::Solid,
                width: 1.0,
                patch: true,
            }),
            CompiledSeries::FillBetween(fill) if !fill.label.is_empty() => Some(LegendEntry {
                label: fill.label.clone(),
                color: fill.color,
                dash: LineDash::Solid,
                width: 1.0,
                patch: true,
            }),
            _ => None,
        })
        .collect()
}

fn entry_from_line(curve: &LineSeries) -> LegendEntry {
    LegendEntry {
        label: curve.label.clone(),
        color: curve.color,
        dash: curve.dash,
        width: curve.width,
        patch: false,
    }
}

pub fn draw_legend<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    entries: &[LegendEntry],
    font_px: i32,
    dpi: u32,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
) -> Result<(), &'static str> {
    if entries.is_empty() {
        return Ok(());
    }

    let x_span = (xmax - xmin).max(1e-12);
    let y_span = (ymax - ymin).max(1e-12);
    let inset_x = x_span * LEGEND_INSET_FRAC;
    let inset_y = y_span * LEGEND_INSET_FRAC;
    let line_len = x_span * LEGEND_LINE_FRAC;
    let row_h = y_span * LEGEND_ROW_FRAC;
    let pad = x_span * LEGEND_PAD_FRAC;

    let frame_right = xmax - inset_x;
    let frame_top = ymax - inset_y;
    let frame_left = frame_right - line_len - pad * 6.0;
    let frame_bottom = frame_top - row_h * entries.len() as f64 - pad;

    let frame = RGBColor(255, 255, 255);
    let edge = RGBColor(MPL_SPINE.0, MPL_SPINE.1, MPL_SPINE.2);
    chart
        .draw_series(std::iter::once(Rectangle::new(
            [(frame_left, frame_bottom), (frame_right, frame_top)],
            ShapeStyle {
                color: frame.to_rgba(),
                filled: true,
                stroke_width: 0,
            },
        )))
        .map_err(|_| "failed to draw legend frame")?;
    chart
        .draw_series(std::iter::once(PathElement::new(
            vec![
                (frame_left, frame_bottom),
                (frame_right, frame_bottom),
                (frame_right, frame_top),
                (frame_left, frame_top),
                (frame_left, frame_bottom),
            ],
            edge.stroke_width(stroke_width_px(1.0, dpi)),
        )))
        .map_err(|_| "failed to draw legend border")?;

    for (index, entry) in entries.iter().enumerate() {
        let row_center = frame_top - pad - row_h * (index as f64 + 0.5);
        let line_x0 = frame_left + pad;
        let line_x1 = line_x0 + line_len;
        if entry.patch {
            draw_legend_patch(chart, entry, line_x0, line_x1, row_center, dpi)?;
        } else {
            draw_legend_line(chart, entry, line_x0, line_x1, row_center, dpi)?;
        }
        let text_x = line_x1 + pad;
        let label = entry.label.clone();
        chart
            .draw_series(std::iter::once(Text::new(
                label,
                (text_x, row_center),
                (MPL_FONT, font_px),
            )))
            .map_err(|_| "failed to draw legend label")?;
    }

    Ok(())
}

fn draw_legend_patch<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    entry: &LegendEntry,
    x0: f64,
    x1: f64,
    y: f64,
    dpi: u32,
) -> Result<(), &'static str> {
    let rgb = entry.color.to_rgb();
    let half = (x1 - x0) * 0.15;
    chart
        .draw_series(std::iter::once(Rectangle::new(
            [(x0, y - half), (x1, y + half)],
            ShapeStyle {
                color: rgb.to_rgba(),
                filled: true,
                stroke_width: stroke_width_px(1.0, dpi),
            },
        )))
        .map_err(|_| "failed to draw legend patch")?;
    Ok(())
}

fn draw_legend_line<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    entry: &LegendEntry,
    x0: f64,
    x1: f64,
    y: f64,
    dpi: u32,
) -> Result<(), &'static str> {
    let rgb = entry.color.to_rgb();
    let stroke_width = stroke_width_px(entry.width, dpi);
    let style = ShapeStyle {
        color: rgb.to_rgba(),
        filled: false,
        stroke_width,
    };
    let points = vec![(x0, y), (x1, y)];
    match entry.dash {
        LineDash::Dashed => chart
            .draw_series(DashedLineSeries::new(points, 5, 5, style))
            .map_err(|_| "failed to draw legend line")?,
        LineDash::DashDot => chart
            .draw_series(DashedLineSeries::new(points, 8, 4, style))
            .map_err(|_| "failed to draw legend line")?,
        LineDash::Dotted => chart
            .draw_series(DashedLineSeries::new(points, 1, 3, style))
            .map_err(|_| "failed to draw legend line")?,
        LineDash::Solid => chart
            .draw_series(plotters::series::LineSeries::new(points, style))
            .map_err(|_| "failed to draw legend line")?,
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::series::LineDash;

    #[test]
    fn collect_entries_skips_unlabeled_lines() {
        let series = vec![
            CompiledSeries::Line(LineSeries {
                x: vec![1.0],
                y: vec![2.0],
                label: "A".into(),
                color: MplotColor::TABLEAU[0],
                dash: LineDash::Solid,
                marker: crate::series::Marker::None,
                width: 1.5,
            }),
            CompiledSeries::Line(LineSeries {
                x: vec![1.0],
                y: vec![2.0],
                label: String::new(),
                color: MplotColor::TABLEAU[1],
                dash: LineDash::Solid,
                marker: crate::series::Marker::None,
                width: 1.5,
            }),
        ];
        let entries = collect_entries(&series);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].label, "A");
        assert!(!entries[0].patch);
    }
}
