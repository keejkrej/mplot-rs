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
use crate::render::legend::{collect_entries, draw_legend, legend_reserve_px};
use crate::render::model::{CompiledFigure, CompiledPanel, CompiledSeries};
use crate::render::mpl_style::{
    chart_margin_px, label_area_bottom_px, label_area_left_px, pt_to_px, stroke_width_px,
    tick_size_px, title_pad_px, MPL_AXES_LINE_WIDTH, MPL_FONT, MPL_GRID, MPL_SPINE, MPL_TITLE_PAD,
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
        panel.grid_rows > 1 || panel.grid_cols > 1 || panel.rowspan > 1 || panel.colspan > 1
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
        let rotated_h = (text_w * rad.sin()
            + text_h * rad.cos()
            + pt_to_px(figure.label_fontsize, figure.dpi) * 1.2
            + tick_mark as f64 * 2.0)
            .round() as u32;
        label_bottom = label_bottom.max(rotated_h);
    }
    let label_bottom = label_bottom.min(max_bottom);
    // Matplotlib: title sits above the Axes box with axes.titlepad — reserve
    // vertical room only when a title is set. Do not use plotters `.caption`
    // (that centers on the full panel cell, including y-tick gutters).
    let has_title = panel.title.as_ref().is_some_and(|title| !title.is_empty());
    let title_band = if has_title {
        title_pad_px(figure.title_fontsize, figure.dpi).min(max_title)
    } else {
        0
    };
    // Outer chart margin: keep small so gutters dominate spacing, not double-padding.
    let chart_margin = chart_margin_px(figure.dpi).max(2);
    // Legend sits above the title, outside the axes, so it cannot cover a curve.
    let legend_entries = if panel.show_legend {
        collect_entries(&panel.series)
    } else {
        Vec::new()
    };
    let legend_width = (area_width_px as i32 - label_left as i32 - chart_margin * 2).max(1);
    let legend_gap = if legend_entries.is_empty() {
        0
    } else {
        legend_reserve_px(legend_width, tick_px as i32, &legend_entries)
    };

    let mut chart = ChartBuilder::on(&plot_area)
        .margin_top(chart_margin + title_band + legend_gap)
        .margin_bottom(chart_margin)
        .margin_left(chart_margin)
        .margin_right(chart_margin)
        .set_label_area_size(LabelAreaPosition::Left, label_left)
        .set_label_area_size(LabelAreaPosition::Bottom, label_bottom)
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
        if let Some(gs) = panel.grid_style {
            if gs.color.is_some() || (gs.width - 0.8).abs() > 1e-6 || (gs.alpha - 1.0).abs() > 1e-6
            {
                let base = gs
                    .color
                    .unwrap_or(crate::color::Color::rgb(MPL_GRID.0, MPL_GRID.1, MPL_GRID.2));
                let alpha = (base.a as f64 / 255.0) * gs.alpha.clamp(0.0, 1.0);
                let style = ShapeStyle {
                    color: RGBAColor(base.r, base.g, base.b, alpha),
                    filled: false,
                    stroke_width: stroke_width_px(gs.width, figure.dpi),
                };
                mesh.light_line_style(style);
                mesh.bold_line_style(style);
            } else {
                let grid = RGBColor(MPL_GRID.0, MPL_GRID.1, MPL_GRID.2);
                mesh.light_line_style(grid);
                mesh.bold_line_style(grid);
            }
        } else {
            let grid = RGBColor(MPL_GRID.0, MPL_GRID.1, MPL_GRID.2);
            mesh.light_line_style(grid);
            mesh.bold_line_style(grid);
        }
    } else {
        mesh.disable_mesh();
    }

    mesh.draw().map_err(|_| "failed to draw mesh")?;

    if custom_x_rotation {
        if let Some(deg) = panel.x_tick_label_rotation.degrees() {
            draw_angled_x_tick_labels(&chart, panel, deg as f64, tick_px, ymin, tick_mark)?;
        }
    }

    let clip = ClipRect {
        xmin: xmin.min(xmax),
        xmax: xmin.max(xmax),
        ymin: ymin.min(ymax),
        ymax: ymin.max(ymax),
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
                &bar_area, normalize, colormap, tick_px, ymin, ymax, figure.dpi,
            )?;
        } else {
            draw_colorbar(
                &mut chart, normalize, colormap, xmin, xmax, ymin, ymax, figure.dpi,
            )?;
        }
    }

    if !legend_entries.is_empty() {
        let (axes_w, _) = chart.plotting_area().dim_in_pixel();
        draw_legend(
            &plot_area,
            &legend_entries,
            tick_px as i32,
            figure.dpi,
            chart_margin + label_left as i32,
            chart_margin + title_band + legend_gap,
            axes_w as i32,
            title_band,
        )?;
    }

    // Axes title after geometry is known so we can center on the plot box
    // (matplotlib transAxes x=0.5), not the full panel including tick gutters.
    if let Some(title) = panel.title.as_deref().filter(|t| !t.is_empty()) {
        draw_axes_title(&chart, title, title_px, figure.dpi)?;
    }

    Ok(())
}

/// Draw the panel title centered on the Axes box (matplotlib default).
///
/// Position: horizontal center of the plotting area; baseline just above the
/// top spine by `axes.titlepad` (`MPL_TITLE_PAD`). Matches
/// `ax.set_title(..., loc='center')` with `transAxes` + titlepad offset.
///
/// Coordinates are **plot-local** (plotters `draw_text` adds the area origin).
fn draw_axes_title<DB: DrawingBackend>(
    chart: &Chart<'_, DB>,
    title: &str,
    title_px: f64,
    dpi: u32,
) -> Result<(), &'static str> {
    let plot = chart.plotting_area().strip_coord_spec();
    let (plot_w, _) = plot.dim_in_pixel();
    if plot_w == 0 {
        return Ok(());
    }
    let pad_px = pt_to_px(MPL_TITLE_PAD, dpi).round().max(1.0) as i32;
    // Local: (0,0) is the top-left of the Axes box. y increases downward, so
    // above the top spine is negative y. Center x = half the plot width.
    // VPos::Bottom ≈ matplotlib verticalalignment='baseline' at the anchor.
    let cx = plot_w as i32 / 2;
    let cy = -pad_px;
    let style = TextStyle::from((MPL_FONT, title_px as i32).into_font())
        .pos(Pos::new(HPos::Center, VPos::Bottom));
    plot.draw_text(title, &style, (cx, cy))
        .map_err(|_| "failed to draw axes title")?;
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
///
/// Uses inverse-mapping + bilinear alpha sampling so edges stay anti-aliased
/// (nearest-neighbour glyph-pixel rotation looks jaggy at mild angles).
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
    if width <= 0 || height <= 0 {
        return Ok(());
    }
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

    // Dense alpha buffer in content-relative coords (not anchor-relative), so
    // right-aligned labels with negative local x still land in-bounds.
    //   buf_x = (glyph_x - min_x) + pad  ∈ [pad, width+pad]
    //   local_x (anchor frame) = (glyph_x - min_x) + dx
    //   ⇒ buf_x = local_x - dx + pad
    let pad = 1i32;
    let sw = (width + 2 * pad) as usize;
    let sh = (height + 2 * pad) as usize;
    let mut src = vec![0f32; sw * sh];

    let collect = font.draw(text, (0, 0), |x, y, alpha| {
        if alpha <= 0.0 {
            return Ok::<(), &'static str>(());
        }
        let bx = x - min_x + pad;
        let by = y - min_y + pad;
        if bx >= 0 && by >= 0 && (bx as usize) < sw && (by as usize) < sh {
            let idx = by as usize * sw + bx as usize;
            // Glyph canvases can slightly overlap; keep the stronger coverage.
            if alpha > src[idx] {
                src[idx] = alpha;
            }
        }
        Ok(())
    });
    match collect {
        Ok(Ok(())) => {}
        Ok(Err(e)) => return Err(e),
        Err(_) => return Err("failed to rasterize rotated tick label"),
    }

    let rad = angle_deg.to_radians();
    let cos = rad.cos();
    let sin = rad.sin();
    let (ax, ay) = pos;

    // Local (anchor-relative) corners of the content box.
    let x0 = dx as f64;
    let x1 = (dx + width) as f64;
    let y0 = dy as f64;
    let y1 = (dy + height) as f64;
    let corners = [(x0, y0), (x1, y0), (x0, y1), (x1, y1)];
    let mut dest_min_x = i32::MAX;
    let mut dest_max_x = i32::MIN;
    let mut dest_min_y = i32::MAX;
    let mut dest_max_y = i32::MIN;
    for &(lx, ly) in &corners {
        let rx = lx * cos - ly * sin;
        let ry = lx * sin + ly * cos;
        dest_min_x = dest_min_x.min(rx.floor() as i32 - 1);
        dest_max_x = dest_max_x.max(rx.ceil() as i32 + 1);
        dest_min_y = dest_min_y.min(ry.floor() as i32 - 1);
        dest_max_y = dest_max_y.max(ry.ceil() as i32 + 1);
    }

    // Inverse-map each dest pixel → local → buffer, bilinear sample alpha.
    for ry in dest_min_y..=dest_max_y {
        for rx in dest_min_x..=dest_max_x {
            let dx_f = rx as f64;
            let dy_f = ry as f64;
            // R^{-1} = [ cos  sin; -sin  cos ]
            let local_x = dx_f * cos + dy_f * sin;
            let local_y = -dx_f * sin + dy_f * cos;
            let bx = local_x - dx as f64 + pad as f64;
            let by = local_y - dy as f64 + pad as f64;
            let alpha = bilinear_sample_alpha(&src, sw, sh, bx, by);
            if alpha <= 0.02 {
                continue;
            }
            let pixel = color.mix(f64::from(alpha));
            area.draw_pixel((ax + rx, ay + ry), &pixel)
                .map_err(|_| "failed to draw rotated tick pixel")?;
        }
    }
    Ok(())
}

/// Bilinear sample of a dense alpha buffer. Coordinates are continuous pixel
/// centres; out-of-bounds samples return 0.
fn bilinear_sample_alpha(src: &[f32], width: usize, height: usize, x: f64, y: f64) -> f32 {
    if width == 0 || height == 0 {
        return 0.0;
    }
    // Sample at continuous coords; floor for integer cell.
    let x0 = x.floor();
    let y0 = y.floor();
    let fx = (x - x0) as f32;
    let fy = (y - y0) as f32;
    let ix0 = x0 as i32;
    let iy0 = y0 as i32;

    let sample = |ix: i32, iy: i32| -> f32 {
        if ix < 0 || iy < 0 || ix as usize >= width || iy as usize >= height {
            0.0
        } else {
            src[iy as usize * width + ix as usize]
        }
    };

    let a00 = sample(ix0, iy0);
    let a10 = sample(ix0 + 1, iy0);
    let a01 = sample(ix0, iy0 + 1);
    let a11 = sample(ix0 + 1, iy0 + 1);
    let a0 = a00 * (1.0 - fx) + a10 * fx;
    let a1 = a01 * (1.0 - fx) + a11 * fx;
    a0 * (1.0 - fy) + a1 * fy
}
