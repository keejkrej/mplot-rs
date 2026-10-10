use plotters::coord::Shift;
use plotters::element::{PathElement, Rectangle};
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use plotters::style::{Color, FontDesc, IntoFont, RGBColor, ShapeStyle, TextStyle};

use crate::color::Color as MplotColor;
use crate::render::model::{CompiledSeries, LineSeries};
use crate::render::mpl_style::{
    stroke_width_px, MPL_FONT, MPL_LEGEND_BORDER_AXES_PAD, MPL_LEGEND_BORDER_PAD,
    MPL_LEGEND_HANDLE_LENGTH, MPL_LEGEND_HANDLE_TEXT_PAD, MPL_LEGEND_LABEL_SPACING, MPL_SPINE,
};
use crate::series::LineDash;

#[derive(Clone, Debug, PartialEq)]
pub struct LegendEntry {
    pub label: String,
    pub color: MplotColor,
    pub dash: LineDash,
    pub width: f64,
    pub patch: bool,
}

pub fn collect_entries(series: &[CompiledSeries]) -> Vec<LegendEntry> {
    series
        .iter()
        .filter_map(|item| match item {
            CompiledSeries::Line(curve) if !curve.label.is_empty() => Some(entry_from_line(curve)),
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

struct MeasuredLegend {
    label_widths: Vec<i32>,
    text_h: i32,
    horizontal: bool,
    frame_w: i32,
    frame_h: i32,
}

/// Extra top margin so the legend sits above the title and never covers a curve.
///
/// `plot_w` is the axes width. One row is used when the labels fit; otherwise
/// the entries stack. The height includes a gap between the legend and the title.
pub(crate) fn legend_reserve_px(plot_w: i32, font_px: i32, entries: &[LegendEntry]) -> i32 {
    let Some(measured) = measure_legend(plot_w, font_px, entries) else {
        return 0;
    };
    measured.frame_h + font_units(MPL_LEGEND_BORDER_AXES_PAD, font_px.max(1))
}

fn measure_legend(plot_w: i32, font_px: i32, entries: &[LegendEntry]) -> Option<MeasuredLegend> {
    if entries.is_empty() || plot_w <= 1 {
        return None;
    }
    let font_px = font_px.max(1);
    let font: FontDesc<'static> = (MPL_FONT, font_px).into_font();
    let mut label_widths = Vec::with_capacity(entries.len());
    let mut text_h = font_px;
    for entry in entries {
        match font.layout_box(&entry.label) {
            Ok(((min_x, min_y), (max_x, max_y))) => {
                label_widths.push((max_x - min_x).max(1));
                text_h = text_h.max(max_y - min_y);
            }
            Err(_) => label_widths.push(font_px * entry.label.chars().count().max(1) as i32 / 2),
        }
    }
    let border_pad = font_units(MPL_LEGEND_BORDER_PAD, font_px);
    let label_spacing = font_units(MPL_LEGEND_LABEL_SPACING, font_px);
    let handle_len = font_units(MPL_LEGEND_HANDLE_LENGTH, font_px);
    let handle_text_pad = font_units(MPL_LEGEND_HANDLE_TEXT_PAD, font_px);
    let rows = label_widths.len() as i32;
    let text_w = label_widths.iter().copied().max().unwrap_or(0);
    let item_gap = label_spacing.max(font_units(0.8, font_px));
    let horizontal_w = border_pad * 2
        + label_widths
            .iter()
            .map(|width| handle_len + handle_text_pad + *width)
            .sum::<i32>()
        + item_gap * (rows - 1).max(0);
    let vertical_w = border_pad * 2 + handle_len + handle_text_pad + text_w;
    let horizontal = horizontal_w <= plot_w;
    let frame_w = if horizontal { horizontal_w } else { vertical_w };
    let frame_h = if horizontal {
        border_pad * 2 + text_h
    } else {
        border_pad * 2 + rows * text_h + (rows - 1).max(0) * label_spacing
    };
    Some(MeasuredLegend {
        label_widths,
        text_h,
        horizontal,
        frame_w,
        frame_h,
    })
}

/// `left, top, right, bottom` in plot-local pixels. Negative y is above the axes.
/// `clearance_px` is the title band: the legend's lower edge sits just above it.
fn legend_frame_above(
    plot_w: i32,
    measured: &MeasuredLegend,
    clearance_px: i32,
) -> (i32, i32, i32, i32) {
    let left = ((plot_w - measured.frame_w) / 2).max(0);
    let right = left + measured.frame_w;
    let bottom = -clearance_px.max(0);
    let top = bottom - measured.frame_h;
    (left, top, right, bottom)
}

fn font_units(fraction: f64, font_px: i32) -> i32 {
    (fraction * font_px as f64).round().max(1.0) as i32
}

pub fn draw_legend<DB: DrawingBackend>(
    area: &DrawingArea<DB, Shift>,
    entries: &[LegendEntry],
    font_px: i32,
    dpi: u32,
    axes_left: i32,
    axes_top: i32,
    axes_w: i32,
    clearance_px: i32,
) -> Result<(), &'static str> {
    // Shapes drawn on the axes area are clipped, so a legend above the title
    // has to be painted on the panel in cell pixels. Text would otherwise
    // escape the clip and leave the handles stuck on the top spine.
    let font_px = font_px.max(1);
    let Some(measured) = measure_legend(axes_w, font_px, entries) else {
        return Ok(());
    };
    let gap = font_units(MPL_LEGEND_BORDER_AXES_PAD, font_px);
    let (rel_left, rel_top, rel_right, rel_bottom) =
        legend_frame_above(axes_w, &measured, clearance_px + gap);
    let left = axes_left + rel_left;
    let right = axes_left + rel_right;
    let top = axes_top + rel_top;
    let bottom = axes_top + rel_bottom;
    let plot = area;
    let border_pad = font_units(MPL_LEGEND_BORDER_PAD, font_px);
    let label_spacing = font_units(MPL_LEGEND_LABEL_SPACING, font_px);
    let handle_len = font_units(MPL_LEGEND_HANDLE_LENGTH, font_px);
    let handle_text_pad = font_units(MPL_LEGEND_HANDLE_TEXT_PAD, font_px);
    let item_gap = label_spacing.max(font_units(0.8, font_px));
    let text_h = measured.text_h;
    let fill = RGBColor(255, 255, 255);
    let edge = RGBColor(MPL_SPINE.0, MPL_SPINE.1, MPL_SPINE.2);
    plot.draw(&Rectangle::new(
        [(left, top), (right, bottom)],
        ShapeStyle {
            color: fill.to_rgba(),
            filled: true,
            stroke_width: 0,
        },
    ))
    .map_err(|_| "failed to draw legend frame")?;
    plot.draw(&PathElement::new(
        vec![
            (left, top),
            (right, top),
            (right, bottom),
            (left, bottom),
            (left, top),
        ],
        edge.stroke_width(stroke_width_px(0.8, dpi)),
    ))
    .map_err(|_| "failed to draw legend border")?;

    let font: FontDesc<'static> = (MPL_FONT, font_px).into_font();
    let text_style = TextStyle::from(font).pos(Pos::new(HPos::Left, VPos::Center));
    if measured.horizontal {
        let mut cursor = left + border_pad;
        let row_cy = top + border_pad + text_h / 2;
        for (entry, width) in entries.iter().zip(measured.label_widths.iter()) {
            let handle_x1 = cursor + handle_len;
            if entry.patch {
                draw_legend_patch(&plot, entry, cursor, handle_x1, row_cy, font_px, dpi)?;
            } else {
                draw_legend_line(&plot, entry, cursor, handle_x1, row_cy, dpi)?;
            }
            plot.draw_text(
                &entry.label,
                &text_style,
                (handle_x1 + handle_text_pad, row_cy),
            )
            .map_err(|_| "failed to draw legend label")?;
            cursor = handle_x1 + handle_text_pad + *width + item_gap;
        }
        return Ok(());
    }

    for (index, entry) in entries.iter().enumerate() {
        let row_top = top + border_pad + index as i32 * (text_h + label_spacing);
        let row_cy = row_top + text_h / 2;
        let handle_x0 = left + border_pad;
        let handle_x1 = handle_x0 + handle_len;
        if entry.patch {
            draw_legend_patch(&plot, entry, handle_x0, handle_x1, row_cy, font_px, dpi)?;
        } else {
            draw_legend_line(&plot, entry, handle_x0, handle_x1, row_cy, dpi)?;
        }
        plot.draw_text(
            &entry.label,
            &text_style,
            (handle_x1 + handle_text_pad, row_cy),
        )
        .map_err(|_| "failed to draw legend label")?;
    }
    Ok(())
}

fn draw_legend_patch<DB: DrawingBackend>(
    plot: &DrawingArea<DB, Shift>,
    entry: &LegendEntry,
    x0: i32,
    x1: i32,
    y: i32,
    font_px: i32,
    dpi: u32,
) -> Result<(), &'static str> {
    let rgb = entry.color.to_rgb();
    let half = font_units(0.35, font_px);
    plot.draw(&Rectangle::new(
        [(x0, y - half), (x1, y + half)],
        ShapeStyle {
            color: rgb.to_rgba(),
            filled: true,
            stroke_width: stroke_width_px(0.8, dpi),
        },
    ))
    .map_err(|_| "failed to draw legend patch")?;
    Ok(())
}

fn draw_legend_line<DB: DrawingBackend>(
    plot: &DrawingArea<DB, Shift>,
    entry: &LegendEntry,
    x0: i32,
    x1: i32,
    y: i32,
    dpi: u32,
) -> Result<(), &'static str> {
    let style = ShapeStyle {
        color: entry.color.to_rgba(),
        filled: false,
        stroke_width: stroke_width_px(entry.width, dpi),
    };
    match entry.dash {
        LineDash::Solid => {
            plot.draw(&PathElement::new(vec![(x0, y), (x1, y)], style))
                .map_err(|_| "failed to draw legend line")?;
        }
        LineDash::Dashed => draw_dashed_px(plot, x0, x1, y, 5, 3, style)?,
        LineDash::DashDot => draw_dashed_px(plot, x0, x1, y, 6, 3, style)?,
        LineDash::Dotted => draw_dashed_px(plot, x0, x1, y, 1, 2, style)?,
    }
    Ok(())
}

fn draw_dashed_px<DB: DrawingBackend>(
    plot: &DrawingArea<DB, Shift>,
    x0: i32,
    x1: i32,
    y: i32,
    on: i32,
    off: i32,
    style: ShapeStyle,
) -> Result<(), &'static str> {
    let on = on.max(1);
    let off = off.max(1);
    let mut x = x0;
    let mut draw = true;
    while x < x1 {
        let step = if draw { on } else { off };
        let next = (x + step).min(x1);
        if draw {
            plot.draw(&PathElement::new(vec![(x, y), (next, y)], style))
                .map_err(|_| "failed to draw legend line")?;
        }
        if next == x {
            break;
        }
        x = next;
        draw = !draw;
    }
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

    #[test]
    fn legend_sits_above_the_title_and_outside_the_axes() {
        let entries = ["IQR", "median", "mean"]
            .into_iter()
            .map(|label| LegendEntry {
                label: label.into(),
                color: MplotColor::TABLEAU[0],
                dash: LineDash::Solid,
                width: 1.5,
                patch: label == "IQR",
            })
            .collect::<Vec<_>>();
        let measured = measure_legend(280, 12, &entries).expect("legend");
        let clearance = 28;
        let gap = 7;
        let (left, top, right, bottom) = legend_frame_above(280, &measured, clearance + gap);
        assert!(bottom <= -(clearance + gap));
        assert!(top < bottom);
        assert!(left >= 0);
        assert!(right <= 280);
        assert!(bottom - top < 40 || !measured.horizontal);
        let reserve = legend_reserve_px(280, 12, &entries);
        assert!(reserve > measured.frame_h);
        assert!(reserve < 80);
    }
}
