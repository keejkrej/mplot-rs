use plotters::coord::Shift;
use plotters::element::{PathElement, Polygon, Rectangle};
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use plotters::style::{Color, FontDesc, IntoFont, RGBColor, ShapeStyle, TextStyle};

use crate::color::Color as MplotColor;
use crate::panel::{LegendLoc, LegendStyle};
use crate::render::model::{CompiledSeries, LineSeries};
use crate::render::mpl_style::{
    stroke_width_px, MPL_FONT, MPL_LEGEND_BORDER_AXES_PAD, MPL_LEGEND_BORDER_PAD,
    MPL_LEGEND_HANDLE_LENGTH, MPL_LEGEND_HANDLE_TEXT_PAD, MPL_LEGEND_LABEL_SPACING, MPL_SPINE,
};
use crate::series::{LineDash, Marker};

#[derive(Clone, Debug, PartialEq)]
pub struct LegendEntry {
    pub label: String,
    pub color: MplotColor,
    pub dash: LineDash,
    pub width: f64,
    pub patch: bool,
    pub marker: Marker,
}

pub fn collect_entries(series: &[CompiledSeries]) -> Vec<LegendEntry> {
    series
        .iter()
        .flat_map(|item| match item {
            CompiledSeries::Stackplot(stack) => {
                let default_colors = crate::color::Color::TABLEAU;
                let custom_colors = stack.style.colors_value();
                stack
                    .style
                    .labels_value()
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, label)| {
                        if label.is_empty() {
                            None
                        } else {
                            let color = if !custom_colors.is_empty() {
                                custom_colors[idx % custom_colors.len()]
                            } else {
                                default_colors[idx % default_colors.len()]
                            };
                            Some(LegendEntry {
                                label: label.clone(),
                                color,
                                dash: LineDash::Solid,
                                width: 1.0,
                                patch: true,
                                marker: Marker::None,
                            })
                        }
                    })
                    .collect::<Vec<_>>()
            }
            CompiledSeries::Line(curve) if !curve.label.is_empty() => vec![entry_from_line(curve)],
            CompiledSeries::Scatter(scatter) if !scatter.label.is_empty() => vec![LegendEntry {
                label: scatter.label.clone(),
                color: scatter.color,
                dash: LineDash::Solid,
                width: 0.0,
                patch: false,
                marker: scatter.marker,
            }],
            CompiledSeries::ErrorBar(err) if !err.label.is_empty() => vec![LegendEntry {
                label: err.label.clone(),
                color: err.color,
                dash: err.dash,
                width: err.line_width,
                patch: false,
                marker: err.marker,
            }],
            CompiledSeries::Step(step) if !step.label.is_empty() => vec![LegendEntry {
                label: step.label.clone(),
                color: step.color,
                dash: step.dash,
                width: step.width,
                patch: false,
                marker: Marker::None,
            }],
            CompiledSeries::Violin(violin)
                if violin.style.label_value().is_some_and(|l| !l.is_empty()) =>
            {
                let color = violin
                    .style
                    .face_color_value()
                    .unwrap_or(crate::color::Color::hex("#1f77b4"));
                vec![LegendEntry {
                    label: violin.style.label_value().unwrap().to_string(),
                    color,
                    dash: LineDash::Solid,
                    width: 1.0,
                    patch: true,
                    marker: Marker::None,
                }]
            }
            CompiledSeries::Bar(bar) if !bar.label.is_empty() => vec![LegendEntry {
                label: bar.label.clone(),
                color: bar.color,
                dash: LineDash::Solid,
                width: 1.0,
                patch: true,
                marker: Marker::None,
            }],
            CompiledSeries::BarH(barh) if !barh.label.is_empty() => vec![LegendEntry {
                label: barh.label.clone(),
                color: barh.color,
                dash: LineDash::Solid,
                width: 1.0,
                patch: true,
                marker: Marker::None,
            }],
            CompiledSeries::Histogram(hist) if !hist.label.is_empty() => vec![LegendEntry {
                label: hist.label.clone(),
                color: hist.color,
                dash: LineDash::Solid,
                width: 1.0,
                patch: true,
                marker: Marker::None,
            }],
            CompiledSeries::FillBetween(fill) if !fill.label.is_empty() => vec![LegendEntry {
                label: fill.label.clone(),
                color: fill.color,
                dash: LineDash::Solid,
                width: 1.0,
                patch: true,
                marker: Marker::None,
            }],
            CompiledSeries::FillBetweenX(fill) if !fill.label.is_empty() => vec![LegendEntry {
                label: fill.label.clone(),
                color: fill.color,
                dash: LineDash::Solid,
                width: 1.0,
                patch: true,
                marker: Marker::None,
            }],
            CompiledSeries::Stem(stem)
                if stem.style.label_value().is_some_and(|l| !l.is_empty()) =>
            {
                vec![LegendEntry {
                    label: stem.style.label_value().unwrap().to_string(),
                    color: stem.style.color_value(),
                    dash: stem.style.line_dash_value(),
                    width: stem.style.line_width_value(),
                    patch: false,
                    marker: stem.style.marker_value(),
                }]
            }
            CompiledSeries::Stairs(stairs)
                if stairs.style.label_value().is_some_and(|l| !l.is_empty()) =>
            {
                vec![LegendEntry {
                    label: stairs.style.label_value().unwrap().to_string(),
                    color: stairs.style.color_value(),
                    dash: stairs.style.dash_value(),
                    width: stairs.style.width_value(),
                    patch: stairs.style.fill_value(),
                    marker: Marker::None,
                }]
            }
            CompiledSeries::Quiver(quiver)
                if quiver.style.label_value().is_some_and(|l| !l.is_empty()) =>
            {
                vec![LegendEntry {
                    label: quiver.style.label_value().unwrap().to_string(),
                    color: quiver.style.color_value(),
                    dash: LineDash::Solid,
                    width: quiver.style.width_value(),
                    patch: false,
                    marker: Marker::None,
                }]
            }
            CompiledSeries::AxHLine(line) if !line.label.is_empty() => vec![LegendEntry {
                label: line.label.clone(),
                color: line.color,
                dash: line.dash,
                width: line.width,
                patch: false,
                marker: Marker::None,
            }],
            CompiledSeries::AxVLine(line) if !line.label.is_empty() => vec![LegendEntry {
                label: line.label.clone(),
                color: line.color,
                dash: line.dash,
                width: line.width,
                patch: false,
                marker: Marker::None,
            }],
            CompiledSeries::AxHSpan(span) if !span.label.is_empty() => vec![LegendEntry {
                label: span.label.clone(),
                color: span.color,
                dash: LineDash::Solid,
                width: 1.0,
                patch: true,
                marker: Marker::None,
            }],
            CompiledSeries::AxVSpan(span) if !span.label.is_empty() => vec![LegendEntry {
                label: span.label.clone(),
                color: span.color,
                dash: LineDash::Solid,
                width: 1.0,
                patch: true,
                marker: Marker::None,
            }],
            CompiledSeries::AxLine(line) if !line.style.label_value().unwrap_or("").is_empty() => vec![LegendEntry {
                label: line.style.label_value().unwrap_or("").to_string(),
                color: line.style.color_value(),
                dash: line.style.dash_value(),
                width: line.style.width_value(),
                patch: false,
                marker: Marker::None,
            }],
            CompiledSeries::HLines(lines) if !lines.style.label_value().unwrap_or("").is_empty() => vec![LegendEntry {
                label: lines.style.label_value().unwrap_or("").to_string(),
                color: lines.style.color_value(),
                dash: lines.style.dash_value(),
                width: lines.style.width_value(),
                patch: false,
                marker: Marker::None,
            }],
            CompiledSeries::VLines(lines) if !lines.style.label_value().unwrap_or("").is_empty() => vec![LegendEntry {
                label: lines.style.label_value().unwrap_or("").to_string(),
                color: lines.style.color_value(),
                dash: lines.style.dash_value(),
                width: lines.style.width_value(),
                patch: false,
                marker: Marker::None,
            }],
            CompiledSeries::EventPlot(ev) if !ev.style.label_value().is_empty() => vec![LegendEntry {
                label: ev.style.label_value().to_string(),
                color: ev.style.color_value(),
                dash: ev.style.dash_value(),
                width: ev.style.linewidth_value(),
                patch: false,
                marker: Marker::None,
            }],
            _ => Vec::new(),
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
        marker: curve.marker,
    }
}

pub(crate) struct MeasuredLegend {
    pub label_widths: Vec<i32>,
    pub text_h: i32,
    pub horizontal: bool,
    pub frame_w: i32,
    pub frame_h: i32,
    pub cols: usize,
    pub col_widths: Vec<i32>,
    pub title_h: i32,
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

fn measure_legend_styled(
    plot_w: i32,
    font_px: i32,
    entries: &[LegendEntry],
    ncols: usize,
    title: Option<&str>,
) -> Option<MeasuredLegend> {
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
    let item_gap = label_spacing.max(font_units(0.8, font_px));

    let title_h = if let Some(t) = title {
        if !t.is_empty() {
            text_h + label_spacing
        } else {
            0
        }
    } else {
        0
    };

    let title_w = if let Some(t) = title {
        match font.layout_box(t) {
            Ok(((min_x, _), (max_x, _))) => (max_x - min_x).max(1),
            Err(_) => font_px * t.chars().count() as i32 / 2,
        }
    } else {
        0
    };

    if ncols > 1 {
        let cols = ncols.min(entries.len()).max(1);
        let rows = (entries.len() + cols - 1) / cols;
        let mut col_widths = vec![0; cols];
        for (i, &w) in label_widths.iter().enumerate() {
            let col = i % cols;
            col_widths[col] = col_widths[col].max(w);
        }
        let content_w: i32 = col_widths
            .iter()
            .map(|w| handle_len + handle_text_pad + w)
            .sum::<i32>()
            + item_gap * (cols as i32 - 1).max(0);
        let frame_w = (border_pad * 2 + content_w).max(title_w + border_pad * 2);
        let frame_h = border_pad * 2
            + rows as i32 * text_h
            + (rows as i32 - 1).max(0) * label_spacing
            + title_h;
        Some(MeasuredLegend {
            label_widths,
            text_h,
            horizontal: false,
            frame_w,
            frame_h,
            cols,
            col_widths,
            title_h,
        })
    } else {
        let rows = label_widths.len() as i32;
        let text_w = label_widths.iter().copied().max().unwrap_or(0);
        let horizontal_w = border_pad * 2
            + label_widths
                .iter()
                .map(|width| handle_len + handle_text_pad + *width)
                .sum::<i32>()
            + item_gap * (rows - 1).max(0);
        let vertical_w = border_pad * 2 + handle_len + handle_text_pad + text_w;
        let horizontal = horizontal_w <= plot_w && title.is_none();
        let frame_w =
            (if horizontal { horizontal_w } else { vertical_w }).max(title_w + border_pad * 2);
        let frame_h = if horizontal {
            border_pad * 2 + text_h
        } else {
            border_pad * 2 + rows * text_h + (rows - 1).max(0) * label_spacing + title_h
        };
        let col_widths = if horizontal {
            label_widths.clone()
        } else {
            vec![text_w]
        };
        Some(MeasuredLegend {
            label_widths,
            text_h,
            horizontal,
            frame_w,
            frame_h,
            cols: if horizontal { entries.len() } else { 1 },
            col_widths,
            title_h,
        })
    }
}

fn measure_legend(plot_w: i32, font_px: i32, entries: &[LegendEntry]) -> Option<MeasuredLegend> {
    measure_legend_styled(plot_w, font_px, entries, 1, None)
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

fn legend_frame_inside(
    loc: LegendLoc,
    axes_w: i32,
    axes_h: i32,
    measured: &MeasuredLegend,
    pad: i32,
) -> (i32, i32, i32, i32) {
    let w = measured.frame_w;
    let h = measured.frame_h;
    let (left, top) = match loc {
        LegendLoc::OutsideTop => ((axes_w - w) / 2, -h - pad),
        LegendLoc::UpperRight | LegendLoc::Best => (axes_w - w - pad, pad),
        LegendLoc::UpperLeft => (pad, pad),
        LegendLoc::LowerLeft => (pad, axes_h - h - pad),
        LegendLoc::LowerRight => (axes_w - w - pad, axes_h - h - pad),
        LegendLoc::CenterLeft => (pad, (axes_h - h) / 2),
        LegendLoc::CenterRight => (axes_w - w - pad, (axes_h - h) / 2),
        LegendLoc::LowerCenter => ((axes_w - w) / 2, axes_h - h - pad),
        LegendLoc::UpperCenter => ((axes_w - w) / 2, pad),
        LegendLoc::Center => ((axes_w - w) / 2, (axes_h - h) / 2),
    };
    (left, top, left + w, top + h)
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
    axes_h: i32,
    clearance_px: i32,
    style: &LegendStyle,
) -> Result<(), &'static str> {
    let font_px = font_px.max(1);
    let Some(measured) = measure_legend_styled(
        axes_w,
        font_px,
        entries,
        style.ncols,
        style.title.as_deref(),
    ) else {
        return Ok(());
    };
    let gap = font_units(MPL_LEGEND_BORDER_AXES_PAD, font_px);
    let (rel_left, rel_top, rel_right, rel_bottom) = if style.loc == LegendLoc::OutsideTop {
        legend_frame_above(axes_w, &measured, clearance_px + gap)
    } else {
        legend_frame_inside(style.loc, axes_w, axes_h, &measured, gap)
    };
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

    if style.frameon {
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
    }

    let font: FontDesc<'static> = (MPL_FONT, font_px).into_font();
    let text_style = TextStyle::from(font).pos(Pos::new(HPos::Left, VPos::Center));

    let mut content_top = top + border_pad;
    if let Some(ref title_str) = style.title {
        if !title_str.is_empty() {
            let title_font: FontDesc<'static> = (MPL_FONT, font_px).into_font();
            let title_style =
                TextStyle::from(title_font).pos(Pos::new(HPos::Center, VPos::Center));
            let center_x = (left + right) / 2;
            let center_y = content_top + text_h / 2;
            plot.draw_text(title_str, &title_style, (center_x, center_y))
                .map_err(|_| "failed to draw legend title")?;
            content_top += measured.title_h;
        }
    }

    if measured.horizontal {
        let mut cursor = left + border_pad;
        let row_cy = content_top + text_h / 2;
        for (entry, width) in entries.iter().zip(measured.label_widths.iter()) {
            let handle_x1 = cursor + handle_len;
            if entry.patch {
                draw_legend_patch(plot, entry, cursor, handle_x1, row_cy, font_px, dpi)?;
            } else {
                draw_legend_line(plot, entry, cursor, handle_x1, row_cy, dpi)?;
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

    let cols = measured.cols;
    let mut col_offsets = Vec::with_capacity(cols);
    let mut running_x = left + border_pad;
    for &cw in &measured.col_widths {
        col_offsets.push(running_x);
        running_x += handle_len + handle_text_pad + cw + item_gap;
    }

    for (index, entry) in entries.iter().enumerate() {
        let col = index % cols;
        let row = index / cols;
        let row_top = content_top + row as i32 * (text_h + label_spacing);
        let row_cy = row_top + text_h / 2;
        let handle_x0 = col_offsets[col];
        let handle_x1 = handle_x0 + handle_len;
        if entry.patch {
            draw_legend_patch(plot, entry, handle_x0, handle_x1, row_cy, font_px, dpi)?;
        } else {
            draw_legend_line(plot, entry, handle_x0, handle_x1, row_cy, dpi)?;
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
    if entry.width > 0.0 {
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
    }
    if !matches!(entry.marker, Marker::None) {
        let cx = (x0 + x1) / 2;
        let r = ((entry.width * 2.0).max(4.0) * dpi as f64 / 72.0)
            .round()
            .max(3.0) as i32;
        let m_style = ShapeStyle {
            color: entry.color.to_rgba(),
            filled: true,
            stroke_width: 0,
        };
        let s = r;
        match entry.marker {
            Marker::None => {}
            Marker::Circle => {
                plot.draw(&Circle::new((cx, y), r, m_style))
                    .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::Square => {
                plot.draw(&Polygon::new(
                    vec![
                        (cx - s, y - s),
                        (cx + s, y - s),
                        (cx + s, y + s),
                        (cx - s, y + s),
                    ],
                    m_style,
                ))
                .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::Cross => {
                let cross_style = ShapeStyle {
                    color: entry.color.to_rgba(),
                    filled: false,
                    stroke_width: stroke_width_px(1.2, dpi),
                };
                plot.draw(&PathElement::new(
                    vec![(cx - s, y - s), (cx + s, y + s)],
                    cross_style,
                ))
                .map_err(|_| "failed to draw legend marker")?;
                plot.draw(&PathElement::new(
                    vec![(cx - s, y + s), (cx + s, y - s)],
                    cross_style,
                ))
                .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::Plus => {
                let plus_style = ShapeStyle {
                    color: entry.color.to_rgba(),
                    filled: false,
                    stroke_width: stroke_width_px(1.2, dpi),
                };
                plot.draw(&PathElement::new(
                    vec![(cx - s, y), (cx + s, y)],
                    plus_style,
                ))
                .map_err(|_| "failed to draw legend marker")?;
                plot.draw(&PathElement::new(
                    vec![(cx, y - s), (cx, y + s)],
                    plus_style,
                ))
                .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::TriangleUp => {
                let h_top = (s as f64 * 1.2).round() as i32;
                let h_bot = (s as f64 * 0.7).round() as i32;
                plot.draw(&Polygon::new(
                    vec![(cx, y - h_top), (cx + s, y + h_bot), (cx - s, y + h_bot)],
                    m_style,
                ))
                .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::TriangleDown => {
                let h_bot = (s as f64 * 1.2).round() as i32;
                let h_top = (s as f64 * 0.7).round() as i32;
                plot.draw(&Polygon::new(
                    vec![(cx, y + h_bot), (cx + s, y - h_top), (cx - s, y - h_top)],
                    m_style,
                ))
                .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::TriangleLeft => {
                let h_top = (s as f64 * 1.2).round() as i32;
                let h_bot = (s as f64 * 0.7).round() as i32;
                plot.draw(&Polygon::new(
                    vec![(cx - h_top, y), (cx + h_bot, y - s), (cx + h_bot, y + s)],
                    m_style,
                ))
                .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::TriangleRight => {
                let h_top = (s as f64 * 1.2).round() as i32;
                let h_bot = (s as f64 * 0.7).round() as i32;
                plot.draw(&Polygon::new(
                    vec![(cx + h_top, y), (cx - h_bot, y - s), (cx - h_bot, y + s)],
                    m_style,
                ))
                .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::Diamond => {
                let sy = (s as f64 * 1.3).round() as i32;
                let sx = (s as f64 * 0.9).round() as i32;
                plot.draw(&Polygon::new(
                    vec![(cx, y - sy), (cx + sx, y), (cx, y + sy), (cx - sx, y)],
                    m_style,
                ))
                .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::Star => {
                let mut pts = Vec::with_capacity(10);
                let r_outer = s as f64 * 1.3;
                let r_inner = s as f64 * 0.52;
                for i in 0..10 {
                    let angle =
                        (i as f64) * std::f64::consts::PI / 5.0 - std::f64::consts::FRAC_PI_2;
                    let r = if i % 2 == 0 { r_outer } else { r_inner };
                    pts.push((
                        cx + (r * angle.cos()).round() as i32,
                        y + (r * angle.sin()).round() as i32,
                    ));
                }
                plot.draw(&Polygon::new(pts, m_style))
                    .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::Pentagon => {
                let mut pts = Vec::with_capacity(5);
                for i in 0..5 {
                    let angle = (i as f64) * 2.0 * std::f64::consts::PI / 5.0
                        - std::f64::consts::FRAC_PI_2;
                    pts.push((
                        cx + (s as f64 * angle.cos()).round() as i32,
                        y + (s as f64 * angle.sin()).round() as i32,
                    ));
                }
                plot.draw(&Polygon::new(pts, m_style))
                    .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::Hexagon => {
                let mut pts = Vec::with_capacity(6);
                for i in 0..6 {
                    let angle =
                        (i as f64) * std::f64::consts::PI / 3.0 - std::f64::consts::FRAC_PI_2;
                    pts.push((
                        cx + (s as f64 * angle.cos()).round() as i32,
                        y + (s as f64 * angle.sin()).round() as i32,
                    ));
                }
                plot.draw(&Polygon::new(pts, m_style))
                    .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::Point => {
                let r_pt = (r / 2).max(1);
                plot.draw(&Circle::new((cx, y), r_pt, m_style))
                    .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::VLine => {
                let v_style = ShapeStyle {
                    color: entry.color.to_rgba(),
                    filled: false,
                    stroke_width: stroke_width_px(1.2, dpi),
                };
                plot.draw(&PathElement::new(vec![(cx, y - s), (cx, y + s)], v_style))
                    .map_err(|_| "failed to draw legend marker")?;
            }
            Marker::HLine => {
                let h_style = ShapeStyle {
                    color: entry.color.to_rgba(),
                    filled: false,
                    stroke_width: stroke_width_px(1.2, dpi),
                };
                plot.draw(&PathElement::new(vec![(cx - s, y), (cx + s, y)], h_style))
                    .map_err(|_| "failed to draw legend marker")?;
            }
        }
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
                marker: Marker::None,
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
