use std::collections::HashMap;
use std::path::Path;

use plotters::backend::BitMapBackend;
use plotters::chart::ChartBuilder;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::element::{PathElement, Rectangle};
use plotters::prelude::*;
use plotters::style::{Color, RGBColor};

use crate::boxplot::BoxplotData;
use crate::curve::CurveData;
use crate::graph::GraphEntity;
use crate::plot::Panel;
use crate::plot::Plot;
use crate::render::scene::{
    apply_panel_limits, box_positions, box_stats, panel_bounds, transform_axis,
};
use crate::StrError;

type Chart<'a> = ChartContext<'a, BitMapBackend<'a>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;

pub fn render_plot(plot: &Plot, path: &Path) -> Result<(), StrError> {
    if plot.panels().is_empty() {
        return Err("plot has no panels");
    }

    let width_px = (plot.figure_width_inches() * plot.dpi() as f64).round() as u32;
    let height_px = (plot.figure_height_inches() * plot.dpi() as f64).round() as u32;
    if width_px == 0 || height_px == 0 {
        return Err("figure size must be positive");
    }

    let root = BitMapBackend::new(path, (width_px, height_px)).into_drawing_area();
    root.fill(&RGBColor(255, 255, 255))
        .map_err(|_| "failed to initialize canvas")?;

    let (rows, cols) = grid_size(plot);

    if rows * cols == 1 {
        for panel in plot.panels() {
            if panel.hide_axes() && panel.entities().is_empty() {
                continue;
            }
            draw_panel(plot, panel, &root)?;
        }
        root.present().map_err(|_| "failed to write figure")?;
        return Ok(());
    }

    let areas = root.split_evenly((rows, cols));

    for area in &areas {
        area.fill(&RGBColor(255, 255, 255))
            .map_err(|_| "failed to clear subplot")?;
    }

    for panel in plot.panels() {
        if panel.hide_axes() && panel.entities().is_empty() {
            continue;
        }
        let index = panel.key().index();
        if index == 0 || index > rows * cols {
            return Err("invalid subplot index");
        }
        let row = (index - 1) / cols;
        let col = (index - 1) % cols;
        let area_idx = row * cols + col;
        draw_panel(plot, panel, &areas[area_idx])?;
    }

    areas[0].present().map_err(|_| "failed to write figure")?;
    Ok(())
}

fn grid_size(plot: &Plot) -> (usize, usize) {
    plot.panels()
        .iter()
        .fold((1usize, 1usize), |(rows, cols), panel| {
            (
                rows.max(panel.key().rows()),
                cols.max(panel.key().cols()),
            )
        })
}

fn draw_panel(
    plot: &Plot,
    panel: &Panel,
    area: &DrawingArea<BitMapBackend, Shift>,
) -> Result<(), StrError> {
    if panel.hide_axes() {
        return Ok(());
    }

    let auto = panel_bounds(panel);
    let (xmin, xmax, ymin, ymax) = apply_panel_limits(panel, auto);
    let label_px = pt_to_px(plot.label_fontsize(), plot.dpi());
    let tick_px = pt_to_px(plot.tick_fontsize(), plot.dpi());
    let title_px = pt_to_px(plot.title_fontsize(), plot.dpi());

    let mut chart = ChartBuilder::on(area)
        .margin(8)
        .set_all_label_area_size(8)
        .caption(
            panel.title().unwrap_or(""),
            ("sans-serif", title_px as i32),
        )
        .build_cartesian_2d(xmin..xmax, ymin..ymax)
        .map_err(|_| "failed to build chart")?;

    let mut mesh = chart.configure_mesh();
    mesh.x_desc(panel.xlabel().unwrap_or(""))
        .y_desc(panel.ylabel().unwrap_or(""))
        .x_label_style(("sans-serif", label_px as i32))
        .y_label_style(("sans-serif", label_px as i32))
        .label_style(("sans-serif", tick_px as i32));

    let x_label_map: HashMap<i64, String> = panel
        .ticks_x()
        .map(|ticks| {
            ticks
                .ticks()
                .iter()
                .zip(ticks.labels().iter())
                .map(|(tick, label)| ((tick * 1000.0).round() as i64, label.replace("\\n", "\n")))
                .collect()
        })
        .unwrap_or_default();
    let log_x = panel.log_x();
    let x_formatter = move |value: &f64| {
        let key = (value * 1000.0).round() as i64;
        x_label_map
            .get(&key)
            .cloned()
            .unwrap_or_else(|| format_tick(*value, log_x))
    };
    mesh.x_label_formatter(&x_formatter);

    let y_label_map: HashMap<i64, String> = panel
        .ticks_y()
        .map(|ticks| {
            ticks
                .ticks()
                .iter()
                .zip(ticks.labels().iter())
                .map(|(tick, label)| {
                    let value = transform_axis(*tick, panel.log_y());
                    ((value * 1000.0).round() as i64, label.replace("\\n", "\n"))
                })
                .collect()
        })
        .unwrap_or_default();
    let log_y = panel.log_y();
    let y_formatter = move |value: &f64| {
        let key = (value * 1000.0).round() as i64;
        y_label_map
            .get(&key)
            .cloned()
            .unwrap_or_else(|| format_log_tick(*value, log_y))
    };
    mesh.y_label_formatter(&y_formatter);

    if panel.show_grid() {
        mesh.light_line_style(RGBColor(200, 200, 200));
    } else {
        mesh.disable_mesh();
    }

    mesh.draw().map_err(|_| "failed to draw mesh")?;

    for entity in panel.entities() {
        match entity {
            GraphEntity::Curve(curve) => draw_curve(&mut chart, curve, panel.log_y())?,
            GraphEntity::Boxplot(boxes) => draw_boxplot(&mut chart, boxes, panel.log_y())?,
        }
    }

    Ok(())
}

fn draw_curve(chart: &mut Chart<'_>, curve: &CurveData, log_y: bool) -> Result<(), StrError> {
    if curve.x.len() != curve.y.len() || curve.x.is_empty() {
        return Ok(());
    }
    let points: Vec<(f64, f64)> = curve
        .x
        .iter()
        .zip(curve.y.iter())
        .filter(|(_, y)| y.is_finite())
        .map(|(x, y)| (*x, transform_axis(*y, log_y)))
        .collect();
    if points.len() < 2 {
        return Ok(());
    }
    let rgb = curve.rgb_color();
    let width = if curve.line_width > 0.0 {
        curve.line_width
    } else {
        1.0
    };
    chart
        .draw_series(LineSeries::new(points, rgb.stroke_width(width as u32)))
        .map_err(|_| "failed to draw curve")?;
    Ok(())
}

fn draw_boxplot(chart: &mut Chart<'_>, boxes: &BoxplotData, log_y: bool) -> Result<(), StrError> {
    let whisker = boxes.whisker.unwrap_or(1.5);
    let width = boxes.width.unwrap_or(0.5);
    let positions = box_positions(boxes);
    let fill = RGBColor(198, 219, 239);
    let edge = RGBColor(31, 119, 180);

    for (idx, group) in boxes.groups.iter().enumerate() {
        let pos = positions.get(idx).copied().unwrap_or((idx + 1) as f64);
        let Some((low, q1, median, q3, high)) = box_stats(group, whisker) else {
            continue;
        };
        let y_low = transform_axis(low, log_y);
        let y_q1 = transform_axis(q1, log_y);
        let y_med = transform_axis(median, log_y);
        let y_q3 = transform_axis(q3, log_y);
        let y_high = transform_axis(high, log_y);
        let x0 = pos - width / 2.0;
        let x1 = pos + width / 2.0;

        chart
            .draw_series(std::iter::once(Rectangle::new(
                [(x0, y_q1), (x1, y_q3)],
                fill.filled(),
            )))
            .map_err(|_| "failed to draw box")?;
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(x0, y_med), (x1, y_med)],
                edge.stroke_width(2),
            )))
            .map_err(|_| "failed to draw median")?;
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(pos, y_low), (pos, y_q1)],
                edge.stroke_width(1),
            )))
            .map_err(|_| "failed to draw lower whisker")?;
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(pos, y_q3), (pos, y_high)],
                edge.stroke_width(1),
            )))
            .map_err(|_| "failed to draw upper whisker")?;
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(x0, y_low), (x1, y_low)],
                edge.stroke_width(1),
            )))
            .map_err(|_| "failed to draw lower cap")?;
        chart
            .draw_series(std::iter::once(PathElement::new(
                vec![(x0, y_high), (x1, y_high)],
                edge.stroke_width(1),
            )))
            .map_err(|_| "failed to draw upper cap")?;
    }
    Ok(())
}

fn pt_to_px(points: f64, dpi: u32) -> f64 {
    points * dpi as f64 / 72.0
}

fn format_tick(value: f64, log: bool) -> String {
    if log {
        format_log_tick(value, true)
    } else if (value.fract()).abs() < 1e-6 {
        format!("{:.0}", value)
    } else {
        format!("{:.2}", value)
    }
}

fn format_log_tick(value: f64, log: bool) -> String {
    if log {
        let original = 10f64.powf(value);
        if original >= 1000.0 || original <= 0.001 {
            format!("{original:.0e}")
        } else if (original.fract()).abs() < 1e-6 {
            format!("{:.0}", original)
        } else {
            format!("{:.2}", original)
        }
    } else if (value.fract()).abs() < 1e-6 {
        format!("{:.0}", value)
    } else {
        format!("{:.2}", value)
    }
}
