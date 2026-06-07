use std::path::Path;

use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::element::{Circle, PathElement, Rectangle};
use plotters::prelude::*;
use plotters::series::LineSeries as PlotLineSeries;
use plotters::style::{Color, RGBColor, ShapeStyle};

use crate::axes::{data_bounds, format_axis_tick, tick_plan, view_limits, PanelScales};
use crate::render::layout::{pad_inches_px, subplot_panels};
use crate::render::model::{BoxplotSeries, CompiledFigure, CompiledPanel, CompiledSeries};
use crate::render::mpl_style::{
    pt_to_px, tick_size_px, CHART_MARGIN_PX, LABEL_AREA_BOTTOM, LABEL_AREA_LEFT, MPL_BOX_EDGE,
    MPL_BOX_FACE, MPL_BOX_LINE_WIDTH, MPL_FLIER_SIZE, MPL_FONT, MPL_GRID, MPL_MEDIAN,
    MPL_MEDIAN_LINE_WIDTH, MPL_SPINE, MPL_WHISKER_LINE_WIDTH,
};
use crate::render::scene::{box_positions, box_stats, default_box_width};
use crate::series::LineDash;

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

    for series in &panel.series {
        match series {
            CompiledSeries::Line(curve) => draw_curve(&mut chart, curve, scales)?,
            CompiledSeries::Boxplot(boxes) => draw_boxplot(&mut chart, boxes, scales)?,
        }
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

fn draw_curve(
    chart: &mut Chart<'_>,
    curve: &crate::render::model::LineSeries,
    scales: PanelScales,
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
    if points.len() < 2 {
        return Ok(());
    }

    let rgb = curve.color.to_rgb();
    let width = if curve.width > 0.0 {
        curve.width
    } else {
        crate::render::mpl_style::MPL_LINE_WIDTH
    };
    let stroke_width = width.round().max(1.0) as u32;
    let style = ShapeStyle {
        color: rgb.to_rgba(),
        filled: false,
        stroke_width,
    };

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
            .draw_series(PlotLineSeries::new(points, style))
            .map_err(|_| "failed to draw curve")?,
    };
    Ok(())
}

fn draw_boxplot(
    chart: &mut Chart<'_>,
    boxes: &BoxplotSeries,
    scales: PanelScales,
) -> Result<(), &'static str> {
    let whisker = boxes.whisker;
    let positions = box_positions(boxes);
    let width = boxes.width.unwrap_or_else(|| default_box_width(&positions));
    let cap_width = 0.5 * width;
    let edge = RGBColor(MPL_BOX_EDGE.0, MPL_BOX_EDGE.1, MPL_BOX_EDGE.2);
    let fill = RGBColor(MPL_BOX_FACE.0, MPL_BOX_FACE.1, MPL_BOX_FACE.2);
    let median_color = RGBColor(MPL_MEDIAN.0, MPL_MEDIAN.1, MPL_MEDIAN.2);
    let flier_size = (MPL_FLIER_SIZE / 2.0).round().max(2.0) as i32;

    for (idx, group) in boxes.groups.iter().enumerate() {
        let pos = positions.get(idx).copied().unwrap_or((idx + 1) as f64);
        let Some(stats) = box_stats(group, whisker) else {
            continue;
        };

        let y_low = scales.y.data_to_axis(stats.whislo);
        let y_q1 = scales.y.data_to_axis(stats.q1);
        let y_med = scales.y.data_to_axis(stats.med);
        let y_q3 = scales.y.data_to_axis(stats.q3);
        let y_high = scales.y.data_to_axis(stats.whishi);
        let x0 = pos - width / 2.0;
        let x1 = pos + width / 2.0;
        let cap_x0 = pos - cap_width / 2.0;
        let cap_x1 = pos + cap_width / 2.0;

        let edge_style = ShapeStyle {
            color: edge.to_rgba(),
            filled: false,
            stroke_width: MPL_BOX_LINE_WIDTH.round() as u32,
        };

        if boxes.patch_artist {
            chart
                .draw_series(std::iter::once(Rectangle::new(
                    [(x0, y_q1), (x1, y_q3)],
                    ShapeStyle {
                        color: fill.to_rgba(),
                        filled: true,
                        stroke_width: 1,
                    },
                )))
                .map_err(|_| "failed to draw box")?;
            chart
                .draw_series(std::iter::once(PathElement::new(
                    vec![(x0, y_q1), (x1, y_q1), (x1, y_q3), (x0, y_q3), (x0, y_q1)],
                    edge_style,
                )))
                .map_err(|_| "failed to draw box edge")?;
        } else {
            chart
                .draw_series(std::iter::once(PathElement::new(
                    vec![(x0, y_q1), (x1, y_q1), (x1, y_q3), (x0, y_q3), (x0, y_q1)],
                    edge_style,
                )))
                .map_err(|_| "failed to draw box")?;
        }

        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(x0, y_med), (x1, y_med)],
                median_color.stroke_width(MPL_MEDIAN_LINE_WIDTH.round() as u32),
            )))
            .map_err(|_| "failed to draw median")?;

        let whisker_style = edge.stroke_width(MPL_WHISKER_LINE_WIDTH.round() as u32);
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
            for flier in stats.fliers {
                let y = scales.y.data_to_axis(flier);
                chart
                    .draw_series(std::iter::once(Circle::new(
                        (pos, y),
                        flier_size,
                        ShapeStyle {
                            color: edge.to_rgba(),
                            filled: false,
                            stroke_width: 1,
                        },
                    )))
                    .map_err(|_| "failed to draw flier")?;
            }
        }
    }
    Ok(())
}
