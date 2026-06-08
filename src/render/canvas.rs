use plotters::chart::ChartBuilder;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::element::PathElement;
use plotters::prelude::*;
use plotters::style::{RGBColor, ShapeStyle};

use crate::axes::{data_bounds, format_axis_tick, tick_plan, view_limits, PanelScales};
use crate::colormap::{Colormap, Normalize};
use crate::gridspec::SubplotSlot;
use crate::render::layout::{pad_inches_px, panel_rect_for_slot};
use crate::render::legend::{collect_entries, draw_legend};
use crate::render::model::{CompiledFigure, CompiledPanel, CompiledSeries};
use crate::render::mpl_style::{
    pt_to_px, tick_size_px, CHART_MARGIN_PX, LABEL_AREA_BOTTOM, LABEL_AREA_LEFT, MPL_FONT,
    MPL_GRID, MPL_SPINE,
};
use crate::render::primitives::{draw_colorbar, draw_series};

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
            let area = canvas.margin(
                rect.x,
                rect.y,
                inner_w.saturating_sub(rect.x + rect.width),
                inner_h.saturating_sub(rect.y + rect.height),
            );
            area.fill(&RGBColor(255, 255, 255))
                .map_err(|_| "failed to clear subplot")?;
            draw_panel_in_area(figure, panel, &area, rect.width)?;
        }
    } else {
        for panel in &figure.panels {
            if panel.hide_axes && panel.series.is_empty() {
                continue;
            }
            draw_panel_in_area(figure, panel, &canvas, inner_w)?;
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
    area_width_px: u32,
) -> Result<(), &'static str> {
    if panel.hide_axes {
        return Ok(());
    }

    let inset_frac = panel.layout.right_inset_frac;
    let bar_px = (area_width_px as f64 * inset_frac).round() as u32;
    let plot_area = if bar_px > 0 {
        area.margin(0, bar_px, 0, 0)
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

    let mut chart = ChartBuilder::on(&plot_area)
        .margin(CHART_MARGIN_PX)
        .set_label_area_size(LabelAreaPosition::Left, LABEL_AREA_LEFT)
        .set_label_area_size(LabelAreaPosition::Bottom, LABEL_AREA_BOTTOM)
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
    let x_formatter = move |value: &f64| {
        let key = (value * 1000.0).round() as i64;
        if let Some(label) = x_label_map.get(&key) {
            return label.clone();
        }
        format_axis_tick(x_scale, custom_x, *value)
    };
    let y_formatter = move |value: &f64| {
        let key = (value * 1000.0).round() as i64;
        if let Some(label) = y_label_map.get(&key) {
            return label.clone();
        }
        format_axis_tick(y_scale, custom_y, *value)
    };

    let mut mesh = chart.configure_mesh();
    mesh.x_desc(panel.xlabel.as_deref().unwrap_or(""))
        .y_desc(panel.ylabel.as_deref().unwrap_or(""))
        .x_label_style((MPL_FONT, label_px as i32))
        .y_label_style((MPL_FONT, label_px as i32))
        .label_style((MPL_FONT, tick_px as i32))
        .bold_line_style(spine)
        .axis_style(ShapeStyle {
            color: spine.to_rgba(),
            filled: false,
            stroke_width: crate::render::mpl_style::MPL_AXES_LINE_WIDTH.round() as u32,
        })
        .set_all_tick_mark_size(tick_mark)
        .x_labels(ticks.x_count)
        .y_labels(ticks.y_count)
        .x_label_formatter(&x_formatter)
        .y_label_formatter(&y_formatter);

    if panel.show_grid {
        mesh.light_line_style(RGBColor(MPL_GRID.0, MPL_GRID.1, MPL_GRID.2));
    } else {
        mesh.disable_mesh();
    }

    mesh.draw().map_err(|_| "failed to draw mesh")?;
    draw_extra_spines(&mut chart, xmin, xmax, ymin, ymax, spine)?;

    let mut colorbar: Option<(Normalize, Colormap)> = None;
    for series in &panel.series {
        draw_series(&mut chart, series, scales, tick_px)?;
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

    if let Some((normalize, colormap)) = colorbar {
        if bar_px > 0 {
            let bar_area = area.margin(0, 0, 0, area_width_px.saturating_sub(bar_px));
            draw_colorbar_in_area(
                &bar_area,
                normalize,
                colormap,
                tick_px,
                ymin,
                ymax,
            )?;
        } else {
            draw_colorbar(&mut chart, normalize, colormap, xmin, xmax, ymin, ymax)?;
        }
    }

    if panel.show_legend {
        let entries = collect_entries(&panel.series);
        draw_legend(
            &mut chart,
            &entries,
            tick_px as i32,
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
) -> Result<(), &'static str> {
    let mut chart = ChartBuilder::on(area)
        .margin(2)
        .build_cartesian_2d(0.0..1.0, ymin..ymax)
        .map_err(|_| "failed to build colorbar chart")?;
    draw_colorbar(&mut chart, normalize, colormap, 0.0, 1.0, ymin, ymax)?;
    let _ = tick_px;
    Ok(())
}

fn draw_extra_spines<DB: DrawingBackend>(
    chart: &mut Chart<'_, DB>,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    spine: RGBColor,
) -> Result<(), &'static str> {
    let style = spine.stroke_width(1);
    chart
        .draw_series([
            PathElement::new(vec![(xmin, ymax), (xmax, ymax)], style),
            PathElement::new(vec![(xmax, ymin), (xmax, ymax)], style),
        ])
        .map_err(|_| "failed to draw spines")?;
    Ok(())
}
