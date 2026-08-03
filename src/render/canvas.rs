use plotters::chart::ChartBuilder;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::element::PathElement;
use plotters::prelude::*;
use plotters::style::text_anchor::{HPos, Pos, VPos};
use plotters::style::{FontDesc, FontTransform, IntoFont, RGBColor, ShapeStyle, TextStyle};

use crate::axes::{data_bounds, format_axis_tick, tick_plan, view_limits, PanelScales};
use crate::colormap::{Colormap, Normalize};
use crate::gridspec::SubplotSlot;
use crate::panel::TickLabelRotation;
use crate::render::layout::{pad_inches_px, panel_rect_for_slot};
use crate::render::legend::{collect_entries, draw_legend};
use crate::render::model::{CompiledFigure, CompiledPanel, CompiledSeries};
use crate::render::mpl_style::{
    chart_margin_px, label_area_bottom_px, label_area_left_px, pt_to_px, stroke_width_px,
    tick_size_px, title_pad_px, MPL_AXES_LINE_WIDTH, MPL_FONT, MPL_GRID, MPL_SPINE,
};
use crate::render::primitives::{draw_colorbar, draw_series, ClipRect};

type Chart<'a, DB> = ChartContext<'a, DB, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

pub fn figure_dimensions(figure: &CompiledFigure) -> Result<(u32, u32, u32), &'static str> {
    let pad_px = pad_inches_px(figure.save_pad_inches, figure.dpi);
    let width_px = (figure.width_in * figure.dpi as f64).round() as u32 + 2 * pad_px;
    let height_px = (figure.height_in * figure.dpi as f64).round() as u32 + 2 * pad_px;
    if width_px <= 2 * pad_px || height_px <= 2 * pad_px {
        return Err("figure size must be positive");
    }
    Ok((width_px, height_px, pad_px))
}

pub fn render_figure<DB: DrawingBackend>(
    figure: &CompiledFigure,
    root: &DrawingArea<DB, Shift>,
) -> Result<(), &'static str> {
    if figure.panels.is_empty() {
        return Err("figure has no panels");
    }

    let (width_px, height_px, pad_px) = figure_dimensions(figure)?;
    let inner_w = width_px - 2 * pad_px;
    let inner_h = height_px - 2 * pad_px;
    let canvas = root.margin(pad_px, pad_px, pad_px, pad_px);

    if uses_slot_layout(figure) {
        for panel in &figure.panels {
            if panel.hide_axes && panel.series.is_empty() {
                continue;
            }
            let slot = slot_from_panel(panel);
            let rect = panel_rect_for_slot(
                inner_w,
                inner_h,
                &slot,
                figure.h_gap,
                figure.v_gap,
                figure.save_tight,
                figure.constrained_layout,
            );
            // plotters::DrawingArea::margin is (top, bottom, left, right).
            let area = canvas.margin(
                rect.y,
                inner_h.saturating_sub(rect.y + rect.height),
                rect.x,
                inner_w.saturating_sub(rect.x + rect.width),
            );
            area.fill(&RGBColor(255, 255, 255))
                .map_err(|_| "failed to clear subplot")?;
            draw_panel_in_area(figure, panel, &area)?;
        }
    } else {
        for panel in &figure.panels {
            if panel.hide_axes && panel.series.is_empty() {
                continue;
            }
            draw_panel_in_area(figure, panel, &canvas)?;
        }
    }

    Ok(())
}

fn uses_slot_layout(figure: &CompiledFigure) -> bool {
    if figure.panels.len() > 1 {
        return true;
    }
    figure.panels.first().is_some_and(|panel| {
        panel.grid_rows > 1
            || panel.grid_cols > 1
            || panel.rowspan > 1
            || panel.colspan > 1
    })
}

fn slot_from_panel(panel: &CompiledPanel) -> SubplotSlot {
    SubplotSlot::from_parts(
        panel.grid_rows,
        panel.grid_cols,
        panel.row,
        panel.col,
        panel.rowspan,
        panel.colspan,
    )
}

fn draw_panel_in_area<DB: DrawingBackend>(
    figure: &CompiledFigure,
    panel: &CompiledPanel,
    area: &DrawingArea<DB, Shift>,
) -> Result<(), &'static str> {
    if panel.hide_axes {
        return Ok(());
    }

    let (area_width_px, area_height_px) = area.dim_in_pixel();
    let inset_frac = panel.layout.right_inset_frac;
    let bar_px = (area_width_px as f64 * inset_frac).round() as u32;
    // Leave a right inset for the colorbar: margin(top, bottom, left, right).
    let plot_area = if bar_px > 0 {
        area.margin(0, 0, 0, bar_px)
    } else {
        area.margin(0, 0, 0, 0)
    };

    let scales = PanelScales::from_panel(panel);
    let data = data_bounds(panel);
    let view = view_limits(panel, data);
    let (xmin, xmax) = view.x;
    let (ymin, ymax) = view.y;
    let ticks = tick_plan(panel, view);

    let label_px = pt_to_px(figure.label_fontsize, figure.dpi);
    let tick_px = pt_to_px(figure.tick_fontsize, figure.dpi);
    let title_px = pt_to_px(figure.title_fontsize, figure.dpi);
    let tick_mark = tick_size_px(figure.dpi);
    let spine = RGBColor(MPL_SPINE.0, MPL_SPINE.1, MPL_SPINE.2);

    // Cap label gutters so multi-panel + large fonts keep a usable axes region
    // (matplotlib tight_layout does this globally; we approximate per cell).
    let max_left = (area_width_px as f64 * 0.28).round().max(24.0) as u32;
    let max_bottom = (area_height_px as f64 * 0.42).round().max(24.0) as u32;
    let max_title = (area_height_px as f64 * 0.18).round().max(12.0) as i32;
    let label_left = label_area_left_px(figure.tick_fontsize, figure.dpi).min(max_left);
    let mut label_bottom =
        label_area_bottom_px(figure.label_fontsize, figure.tick_fontsize, figure.dpi);
    // Rotated category labels need extra bottom gutter for the projected text box.
    if let Some(deg) = panel.x_tick_label_rotation.degrees() {
        let max_chars = panel
            .ticks_x
            .as_ref()
            .map(|ticks| {
                ticks
                    .labels()
                    .iter()
                    .map(|label| label.chars().filter(|c| *c != '\n').count())
                    .max()
                    .unwrap_or(8)
            })
            .unwrap_or(10);
        let text_w = tick_px * max_chars as f64 * 0.58;
        let text_h = tick_px;
        let rad = (deg as f64).abs().to_radians();
        // Axis-aligned bounding box of a rotated rectangle.
        let rotated_h = (text_w * rad.sin() + text_h * rad.cos()
            + pt_to_px(figure.label_fontsize, figure.dpi) * 1.2
            + tick_mark as f64 * 2.0)
            .round() as u32;
        label_bottom = label_bottom.max(rotated_h);
    }
    let label_bottom = label_bottom.min(max_bottom);
    let title_band = title_pad_px(figure.title_fontsize, figure.dpi).min(max_title);
    // Outer chart margin: keep small so gutters dominate spacing, not double-padding.
    let chart_margin = chart_margin_px(figure.dpi).max(2);

    let mut chart = ChartBuilder::on(&plot_area)
        .margin_top(chart_margin + title_band)
        .margin_bottom(chart_margin)
        .margin_left(chart_margin)
        .margin_right(chart_margin)
        .set_label_area_size(LabelAreaPosition::Left, label_left)
        .set_label_area_size(LabelAreaPosition::Bottom, label_bottom)
        .caption(
            panel.title.as_deref().unwrap_or(""),
            (MPL_FONT, title_px as i32),
        )
        .build_cartesian_2d(xmin..xmax, ymin..ymax)
        .map_err(|_| "failed to build chart")?;

    let x_label_map = ticks.x_label_map.clone();
    let y_label_map = ticks.y_label_map.clone();
    let x_scale = scales.x;
    let y_scale = scales.y;
    let custom_x = ticks.custom_x;
    let custom_y = ticks.custom_y;
    let x_tick_format = ticks.x_tick_format;
    let y_tick_format = ticks.y_tick_format;
    // Arbitrary-angle labels are drawn after the mesh (plotters only does 90°).
    let custom_x_rotation = panel.x_tick_label_rotation.needs_custom_draw();
    let x_formatter = move |value: &f64| {
        if custom_x_rotation {
            // Keep tick marks; suppress plotters' horizontal/90° text.
            return String::new();
        }
        let key = (value * 1000.0).round() as i64;
        if let Some(label) = x_label_map.get(&key) {
            return label.clone();
        }
        format_axis_tick(x_scale, custom_x, *value, x_tick_format)
    };
    let y_formatter = move |value: &f64| {
        let key = (value * 1000.0).round() as i64;
        if let Some(label) = y_label_map.get(&key) {
            return label.clone();
        }
        format_axis_tick(y_scale, custom_y, *value, y_tick_format)
    };

    // Plotters only emits tick *labels* when axes are enabled (mesh.rs:
    // format_x/y return None if draw_x/y_axis is false). Never call
    // disable_axes() if numbers/tick marks are required.
    //
    // Mesh draws left + bottom (+ ticks). We add top + right after series so
    // the box is closed and series cannot erase those edges — without
    // re-stroking left/bottom (that made them look thicker).
    let axis_style = ShapeStyle {
        color: spine.to_rgba(),
        filled: false,
        stroke_width: stroke_width_px(MPL_AXES_LINE_WIDTH, figure.dpi),
    };
    let x_tick_style = x_tick_text_style(tick_px, panel.x_tick_label_rotation);
    let y_tick_style = TextStyle::from((MPL_FONT, tick_px as i32).into_font());
    let mut mesh = chart.configure_mesh();
    mesh.x_desc(panel.xlabel.as_deref().unwrap_or(""))
        .y_desc(panel.ylabel.as_deref().unwrap_or(""))
        .axis_desc_style((MPL_FONT, label_px as i32))
        .x_label_style(x_tick_style)
        .y_label_style(y_tick_style)
        .axis_style(axis_style)
        .set_all_tick_mark_size(tick_mark)
        .x_labels(ticks.x_count)
        .y_labels(ticks.y_count)
        .x_label_formatter(&x_formatter)
        .y_label_formatter(&y_formatter);

    if panel.show_grid {
        let grid = RGBColor(MPL_GRID.0, MPL_GRID.1, MPL_GRID.2);
        mesh.light_line_style(grid);
        mesh.bold_line_style(grid);
    } else {
        mesh.disable_mesh();
    }

    mesh.draw().map_err(|_| "failed to draw mesh")?;

    if custom_x_rotation {
        if let Some(deg) = panel.x_tick_label_rotation.degrees() {
            draw_angled_x_tick_labels(
                &chart,
                panel,
                deg as f64,
                tick_px,
                ymin,
                tick_mark,
            )?;
        }
    }

    let clip = ClipRect {
        xmin,
        xmax,
        ymin,
        ymax,
    };
    let mut colorbar: Option<(Normalize, Colormap)> = None;
    for series in &panel.series {
        draw_series(&mut chart, series, scales, tick_px, figure.dpi, clip)?;
        match series {
            CompiledSeries::Image(image) if image.show_colorbar => {
                colorbar = Some((image.normalize, image.colormap));
            }
            CompiledSeries::Contour(contour) if contour.show_colorbar => {
                colorbar = Some((contour.normalize, contour.colormap));
            }
            _ => {}
        }
    }

    // Close the box (top + right). Left/bottom already stroked by mesh with
    // the same axis_style — redrawing them would double the weight.
    draw_open_spines(&mut chart, xmin, xmax, ymin, ymax, spine, figure.dpi)?;

    if let Some((normalize, colormap)) = colorbar {
        if bar_px > 0 {
            // Right-hand colorbar strip: left margin is everything except bar_px.
            let bar_area = area.margin(0, 0, area_width_px.saturating_sub(bar_px), 0);
            draw_colorbar_in_area(
                &bar_area,
                normalize,
                colormap,
                tick_px,
                ymin,
                ymax,
                figure.dpi,
            )?;
        } else {
            draw_colorbar(&mut chart, normalize, colormap, xmin, xmax, ymin, ymax, figure.dpi)?;
        }
    }

    if panel.show_legend {
        let entries = collect_entries(&panel.series);
        draw_legend(
            &mut chart,
            &entries,
            tick_px as i32,
            figure.dpi,
            xmin,
            xmax,
            ymin,
            ymax,
        )?;
    }

    Ok(())
}

fn draw_colorbar_in_area<DB: DrawingBackend>(
    area: &DrawingArea<DB, Shift>,
    normalize: Normalize,
    colormap: Colormap,
    tick_px: f64,
    ymin: f64,
    ymax: f64,
    dpi: u32,
) -> Result<(), &'static str> {
    let mut chart = ChartBuilder::on(area)
        .margin(2)
        .build_cartesian_2d(0.0..1.0, ymin..ymax)
        .map_err(|_| "failed to build colorbar chart")?;
    draw_colorbar(&mut chart, normalize, colormap, 0.0, 1.0, ymin, ymax, dpi)?;
    let _ = tick_px;
    Ok(())
}

/// Top + right spines (left/bottom come from plotters mesh when axes enabled).
fn draw_open_spines<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    spine: RGBColor,
    dpi: u32,
) -> Result<(), &'static str> {
    let style = spine.stroke_width(stroke_width_px(MPL_AXES_LINE_WIDTH, dpi));
    chart
        .draw_series([
            PathElement::new(vec![(xmin, ymax), (xmax, ymax)], style), // top
            PathElement::new(vec![(xmax, ymin), (xmax, ymax)], style), // right
        ])
        .map_err(|_| "failed to draw spines")?;
    Ok(())
}

fn x_tick_text_style(tick_px: f64, rotation: TickLabelRotation) -> TextStyle<'static> {
    let base = TextStyle::from((MPL_FONT, tick_px as i32).into_font());
    match rotation {
        TickLabelRotation::None => base,
        // Bottom-axis labels reading upward (standard for crowded categories).
        TickLabelRotation::Ccw90 => base
            .transform(FontTransform::Rotate270)
            .pos(Pos::new(HPos::Right, VPos::Center)),
        TickLabelRotation::Cw90 => base
            .transform(FontTransform::Rotate90)
            .pos(Pos::new(HPos::Left, VPos::Center)),
        // Arbitrary angles: plotters mesh labels are suppressed; see
        // `draw_angled_x_tick_labels`. Map exact cardinals to FontTransform.
        TickLabelRotation::Degrees(deg) => match deg.rem_euclid(360) {
            0 => base,
            90 => base
                .transform(FontTransform::Rotate270)
                .pos(Pos::new(HPos::Right, VPos::Center)),
            180 => base.transform(FontTransform::Rotate180),
            270 => base
                .transform(FontTransform::Rotate90)
                .pos(Pos::new(HPos::Left, VPos::Center)),
            _ => base, // unused; labels drawn manually
        },
    }
}

/// Draw x-axis tick labels at an arbitrary angle (matplotlib `rotation=`).
///
/// Anchor matches the common category style: right + top, so mild negative
/// angles (e.g. −30°) hang below the axis like Observable Plot `tickRotate`.
fn draw_angled_x_tick_labels<DB: DrawingBackend>(
    chart: &Chart<'_, DB>,
    panel: &CompiledPanel,
    angle_deg: f64,
    tick_px: f64,
    ymin: f64,
    tick_mark: i32,
) -> Result<(), &'static str> {
    let Some(tick_labels) = panel.ticks_x.as_ref() else {
        return Ok(());
    };
    let font: FontDesc<'static> = (MPL_FONT, tick_px as i32).into_font();
    let color = RGBColor(0, 0, 0);
    // Same clearance as plotters' `label_dist = tick_size.abs() * 2`.
    let label_dist = tick_mark.abs() * 2;
    // Screen pixel space so we can paint into the bottom gutter below the axes.
    let screen = chart.plotting_area().use_screen_coord();

    for (value, label) in tick_labels.ticks().iter().zip(tick_labels.labels().iter()) {
        if label.is_empty() {
            continue;
        }
        let (gx, gy) = chart.backend_coord(&(*value, ymin));
        let anchor = (gx, gy + label_dist);
        draw_text_rotated(
            &screen,
            label,
            &font,
            &color,
            anchor,
            angle_deg,
            HPos::Right,
            VPos::Top,
        )?;
    }
    Ok(())
}

/// Rasterize `text` with `font`, rotate by `angle_deg` (CCW), and blit pixels.
///
/// Anchor semantics mirror plotters' `draw_text`: horizontal/vertical
/// alignment is applied in the unrotated frame, then the offset vector is
/// rotated around the anchor point. `pos` is in backend/screen pixels when
/// `area` was obtained via [`DrawingArea::use_screen_coord`].
fn draw_text_rotated<DB: DrawingBackend>(
    area: &DrawingArea<DB, Shift>,
    text: &str,
    font: &FontDesc<'_>,
    color: &RGBColor,
    pos: (i32, i32),
    angle_deg: f64,
    h_pos: HPos,
    v_pos: VPos,
) -> Result<(), &'static str> {
    let ((min_x, min_y), (max_x, max_y)) = font
        .layout_box(text)
        .map_err(|_| "failed to layout rotated tick label")?;
    let width = max_x - min_x;
    let height = max_y - min_y;
    let dx = match h_pos {
        HPos::Left => 0,
        HPos::Right => -width,
        HPos::Center => -width / 2,
    };
    let dy = match v_pos {
        VPos::Top => 0,
        VPos::Center => -height / 2,
        VPos::Bottom => -height,
    };

    let rad = angle_deg.to_radians();
    let cos = rad.cos();
    let sin = rad.sin();
    let (ax, ay) = pos;
    // Labels hang into the bottom gutter outside the plotting-area rect;
    // do not clip to area height — pixels still land on the shared bitmap.

    let draw_result = font.draw(text, (0, 0), |x, y, alpha| {
        if alpha <= 0.0 {
            return Ok::<(), &'static str>(());
        }
        let lx = (x + dx - min_x) as f64;
        let ly = (y + dy - min_y) as f64;
        let rx = (lx * cos - ly * sin).round() as i32;
        let ry = (lx * sin + ly * cos).round() as i32;
        let px = ax + rx;
        let py = ay + ry;
        let pixel = color.mix(f64::from(alpha));
        area.draw_pixel((px, py), &pixel)
            .map_err(|_| "failed to draw rotated tick pixel")?;
        Ok(())
    });

    match draw_result {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => Err(e),
        Err(_) => Err("failed to rasterize rotated tick label"),
    }
}
