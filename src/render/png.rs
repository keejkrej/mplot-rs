use std::path::Path;

use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::element::PathElement;
use plotters::prelude::*;
use plotters::style::{RGBColor, ShapeStyle};

use crate::axes::{data_bounds, format_axis_tick, tick_plan, view_limits, PanelScales};
use crate::colormap::{Colormap, Normalize};
use crate::render::layout::{pad_inches_px, subplot_panels};
use crate::render::legend::{collect_entries, draw_legend};
use crate::render::model::{CompiledFigure, CompiledPanel, CompiledSeries};
use crate::render::mpl_style::{
    pt_to_px, tick_size_px, CHART_MARGIN_PX, LABEL_AREA_BOTTOM, LABEL_AREA_LEFT, MPL_FONT,
    MPL_GRID, MPL_SPINE,
};
use crate::render::primitives::{draw_colorbar, draw_series};

type Chart<'a> = ChartContext<'a, BitMapBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

pub fn render(figure: &CompiledFigure, path: &Path) -> Result<(), &'static str> {
    if figure.panels.is_empty() {
        return Err("figure has no panels");
    }

    let pad_px = pad_inches_px(figure.save_pad_inches, figure.dpi);
    let width_px = (figure.width_in * figure.dpi as f64).round() as u32 + 2 * pad_px;
    let height_px = (figure.height_in * figure.dpi as f64).round() as u32 + 2 * pad_px;
    if width_px <= 2 * pad_px || height_px <= 2 * pad_px {
        return Err("figure size must be positive");
    }

    let root = BitMapBackend::new(path, (width_px, height_px)).into_drawing_area();
    root.fill(&RGBColor(255, 255, 255))
        .map_err(|_| "failed to initialize canvas")?;

    let inner_w = width_px - 2 * pad_px;
    let inner_h = height_px - 2 * pad_px;
    let canvas = root.margin(pad_px, pad_px, pad_px, pad_px);

    let (rows, cols) = grid_size(figure);
    let panel_rects = subplot_panels(
        inner_w,
        inner_h,
        rows,
        cols,
        figure.h_gap,
        figure.v_gap,
        figure.save_tight,
    );

    if rows * cols == 1 {
        for panel in &figure.panels {
            if panel.hide_axes && panel.series.is_empty() {
                continue;
            }
            draw_panel_in_area(figure, panel, &canvas)?;
        }
    } else {
        for panel in &figure.panels {
            if panel.hide_axes && panel.series.is_empty() {
                continue;
            }
            let index = panel.index;
            if index == 0 || index > rows * cols {
                return Err("invalid subplot index");
            }
            let rect = panel_rects
                .get(index - 1)
                .copied()
                .ok_or("invalid subplot index")?;
            let area = canvas.margin(
                rect.x,
                rect.y,
                inner_w.saturating_sub(rect.x + rect.width),
                inner_h.saturating_sub(rect.y + rect.height),
            );
            area.fill(&RGBColor(255, 255, 255))
                .map_err(|_| "failed to clear subplot")?;
            draw_panel_in_area(figure, panel, &area)?;
        }
    }

    root.present().map_err(|_| "failed to write figure")?;
    Ok(())
}

fn grid_size(figure: &CompiledFigure) -> (usize, usize) {
    figure.panels.iter().fold((1usize, 1usize), |(rows, cols), panel| {
        (rows.max(panel.rows), cols.max(panel.cols))
    })
}

fn draw_panel_in_area<'a>(
    figure: &CompiledFigure,
    panel: &CompiledPanel,
    area: &DrawingArea<BitMapBackend<'a>, Shift>,
) -> Result<(), &'static str> {
    if panel.hide_axes {
        return Ok(());
    }

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

    let mut chart = ChartBuilder::on(area)
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
        draw_colorbar(&mut chart, normalize, colormap, xmin, xmax, ymin, ymax)?;
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

fn draw_extra_spines(
    chart: &mut Chart<'_>,
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
