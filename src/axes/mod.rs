use std::collections::HashMap;

use crate::panel::TickLabels;
use crate::render::model::{CompiledPanel, CompiledSeries};
use crate::render::mpl_style::{MPL_XMARGIN, MPL_YMARGIN};
use crate::render::scene::box_positions;
use crate::scale::AxisScale;
use crate::ticker::{
    format_linear, format_log_axis_coord, format_log_data, linear_ticks, log_ticks_data,
};

/// Data-space bounds before scale transform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DataBounds {
    pub x: (f64, f64),
    pub y: (f64, f64),
}

/// Axis-coordinate view limits passed to the renderer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewLimits {
    pub x: (f64, f64),
    pub y: (f64, f64),
}

/// Per-axis scale configuration for a panel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PanelScales {
    pub x: AxisScale,
    pub y: AxisScale,
}

impl PanelScales {
    pub fn from_panel(panel: &CompiledPanel) -> Self {
        PanelScales {
            x: AxisScale::from_log_flag(panel.log_x),
            y: AxisScale::from_log_flag(panel.log_y),
        }
    }

    pub fn map_xy(self, x: f64, y: f64) -> (f64, f64) {
        (
            self.x.data_to_axis(x),
            self.y.data_to_axis(y),
        )
    }
}

/// Tick mesh configuration for plotters.
#[derive(Clone, Debug, PartialEq)]
pub struct TickPlan {
    pub x_count: usize,
    pub y_count: usize,
    pub x_label_map: HashMap<i64, String>,
    pub y_label_map: HashMap<i64, String>,
    pub custom_x: bool,
    pub custom_y: bool,
}

pub fn data_bounds(panel: &CompiledPanel) -> DataBounds {
    let x_scale = AxisScale::from_log_flag(panel.log_x);
    let y_scale = AxisScale::from_log_flag(panel.log_y);

    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;

    for series in &panel.series {
        match series {
            CompiledSeries::Line(curve) => {
                for (x, y) in curve.x.iter().zip(curve.y.iter()) {
                    if x_scale.usable(*x) {
                        xmin = xmin.min(*x);
                        xmax = xmax.max(*x);
                    }
                    if y_scale.usable(*y) {
                        ymin = ymin.min(*y);
                        ymax = ymax.max(*y);
                    }
                }
            }
            CompiledSeries::Boxplot(boxes) => {
                let positions = box_positions(boxes);
                if boxes.horizontal {
                    for pos in &positions {
                        ymin = ymin.min(*pos - 0.5);
                        ymax = ymax.max(*pos + 0.5);
                    }
                    for group in &boxes.groups {
                        for value in group {
                            if x_scale.usable(*value) {
                                xmin = xmin.min(*value);
                                xmax = xmax.max(*value);
                            }
                        }
                    }
                } else {
                    for pos in &positions {
                        xmin = xmin.min(*pos - 0.5);
                        xmax = xmax.max(*pos + 0.5);
                    }
                    for group in &boxes.groups {
                        for value in group {
                            if y_scale.usable(*value) {
                                ymin = ymin.min(*value);
                                ymax = ymax.max(*value);
                            }
                        }
                    }
                }
            }
            CompiledSeries::Bar(bar) => {
                let half = bar.width / 2.0;
                for (x, height) in bar.x.iter().zip(bar.heights.iter()) {
                    xmin = xmin.min(*x - half);
                    xmax = xmax.max(*x + half);
                    if y_scale.usable(bar.baseline) {
                        ymin = ymin.min(bar.baseline);
                        ymax = ymax.max(bar.baseline);
                    }
                    if y_scale.usable(*height) {
                        ymin = ymin.min(*height);
                        ymax = ymax.max(*height);
                    }
                }
            }
            CompiledSeries::Histogram(hist) => {
                if let Some(((edge_min, edge_max), max_count)) =
                    histogram_bounds(&hist.data, hist.bins)
                {
                    xmin = xmin.min(edge_min);
                    xmax = xmax.max(edge_max);
                    ymax = ymax.max(max_count);
                }
            }
            CompiledSeries::FillBetween(fill) => {
                for (x, y) in fill.x.iter().zip(fill.y1.iter()) {
                    if x_scale.usable(*x) {
                        xmin = xmin.min(*x);
                        xmax = xmax.max(*x);
                    }
                    if y_scale.usable(*y) {
                        ymin = ymin.min(*y);
                        ymax = ymax.max(*y);
                    }
                }
                for y in &fill.y2 {
                    if y_scale.usable(*y) {
                        ymin = ymin.min(*y);
                        ymax = ymax.max(*y);
                    }
                }
            }
            CompiledSeries::Image(image) => {
                let (x0, x1, y0, y1) = image.extent;
                if x_scale.usable(x0) && x_scale.usable(x1) {
                    xmin = xmin.min(x0).min(x1);
                    xmax = xmax.max(x0).max(x1);
                }
                if y_scale.usable(y0) && y_scale.usable(y1) {
                    ymin = ymin.min(y0).min(y1);
                    ymax = ymax.max(y0).max(y1);
                }
            }
            CompiledSeries::Contour(contour) => {
                let (x0, x1, y0, y1) = contour.extent;
                if x_scale.usable(x0) && x_scale.usable(x1) {
                    xmin = xmin.min(x0).min(x1);
                    xmax = xmax.max(x0).max(x1);
                }
                if y_scale.usable(y0) && y_scale.usable(y1) {
                    ymin = ymin.min(y0).min(y1);
                    ymax = ymax.max(y0).max(y1);
                }
            }
            CompiledSeries::Text(text) => {
                if x_scale.usable(text.x) {
                    xmin = xmin.min(text.x);
                    xmax = xmax.max(text.x);
                }
                if y_scale.usable(text.y) {
                    ymin = ymin.min(text.y);
                    ymax = ymax.max(text.y);
                }
            }
        }
    }

    if !xmin.is_finite() {
        xmin = 0.0;
        xmax = 1.0;
    }
    if !ymin.is_finite() {
        ymin = 0.0;
        ymax = 1.0;
    }
    if (xmax - xmin).abs() < 1e-12 {
        xmax = xmin + 1.0;
    }
    if (ymax - ymin).abs() < 1e-12 {
        ymax = ymin + 1.0;
    }

    if panel.xrange.is_none() {
        let (lo, hi) = x_scale.expand_data_limits(xmin, xmax, MPL_XMARGIN);
        xmin = lo;
        xmax = hi;
    }
    if panel.yrange.is_none() {
        let (lo, hi) = y_scale.expand_data_limits(ymin, ymax, MPL_YMARGIN);
        ymin = lo;
        ymax = hi;
    }

    DataBounds {
        x: (xmin, xmax),
        y: (ymin, ymax),
    }
}

pub fn view_limits(panel: &CompiledPanel, data: DataBounds) -> ViewLimits {
    let x_scale = AxisScale::from_log_flag(panel.log_x);
    let y_scale = AxisScale::from_log_flag(panel.log_y);

    let (mut xmin, mut xmax) = data.x;
    let (mut ymin, mut ymax) = data.y;

    if let Some((lo, hi)) = panel.xrange {
        let (lo, hi) = x_scale.clamp_user_range(lo, hi);
        xmin = lo;
        xmax = hi;
    }
    if let Some((lo, hi)) = panel.yrange {
        let (lo, hi) = y_scale.clamp_user_range(lo, hi);
        ymin = lo;
        ymax = hi;
    }

    ViewLimits {
        x: (
            x_scale.data_to_axis(xmin),
            x_scale.data_to_axis(xmax),
        ),
        y: (
            y_scale.data_to_axis(ymin),
            y_scale.data_to_axis(ymax),
        ),
    }
}

pub fn tick_plan(panel: &CompiledPanel, view: ViewLimits) -> TickPlan {
    let x_scale = AxisScale::from_log_flag(panel.log_x);
    let y_scale = AxisScale::from_log_flag(panel.log_y);
    let (xmin, xmax) = view.x;
    let (ymin, ymax) = view.y;

    let (x_count, x_label_map, custom_x) = axis_ticks(
        panel.ticks_x.as_ref(),
        x_scale,
        xmin,
        xmax,
    );
    let (y_count, y_label_map, custom_y) = axis_ticks(
        panel.ticks_y.as_ref(),
        y_scale,
        ymin,
        ymax,
    );

    TickPlan {
        x_count,
        y_count,
        x_label_map,
        y_label_map,
        custom_x,
        custom_y,
    }
}

fn axis_ticks(
    custom: Option<&TickLabels>,
    scale: AxisScale,
    axis_min: f64,
    axis_max: f64,
) -> (usize, HashMap<i64, String>, bool) {
    if let Some(custom) = custom {
        let map = custom
            .ticks()
            .iter()
            .zip(custom.labels().iter())
            .map(|(tick, label)| {
                let axis = scale.data_to_axis(*tick);
                ((axis * 1000.0).round() as i64, label.replace("\\n", "\n"))
            })
            .collect();
        return (custom.ticks().len().max(1), map, true);
    }

    if scale.is_log() {
        let data_min = scale.axis_to_data(axis_min);
        let data_max = scale.axis_to_data(axis_max);
        let data_ticks = log_ticks_data(data_min, data_max);
        let map = data_ticks
            .iter()
            .map(|tick| {
                let axis = scale.data_to_axis(*tick);
                ((axis * 1000.0).round() as i64, format_log_data(*tick))
            })
            .collect();
        return (data_ticks.len().max(1), map, false);
    }

    let ticks = linear_ticks(axis_min, axis_max, 6);
    let map = ticks
        .iter()
        .map(|tick| ((*tick * 1000.0).round() as i64, format_linear(*tick)))
        .collect();
    (ticks.len().max(1), map, false)
}

pub fn format_axis_tick(scale: AxisScale, custom: bool, axis_value: f64) -> String {
    if custom {
        return String::new();
    }
    if scale.is_log() {
        format_log_axis_coord(axis_value)
    } else {
        format_linear(axis_value)
    }
}

fn histogram_bounds(data: &[f64], bins: usize) -> Option<((f64, f64), f64)> {
    let mut values: Vec<f64> = data.iter().copied().filter(|v| v.is_finite()).collect();
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let min = *values.first()?;
    let max = *values.last()?;
    let span = if (max - min).abs() < 1e-12 { 1.0 } else { max - min };
    let step = span / bins as f64;
    let mut counts = vec![0.0; bins];
    for value in values {
        let mut idx = ((value - min) / step).floor() as usize;
        if idx >= bins {
            idx = bins - 1;
        }
        counts[idx] += 1.0;
    }
    let max_count = counts.iter().copied().fold(0.0, f64::max);
    Some(((min, max + step * 1e-9), max_count))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::model::{CompiledSeries, LineSeries};
    use crate::color::Color;
    use crate::series::LineDash;

    fn sample_panel(log_x: bool, log_y: bool) -> CompiledPanel {
        CompiledPanel {
            rows: 1,
            cols: 1,
            index: 1,
            title: None,
            xlabel: None,
            ylabel: None,
            xrange: None,
            yrange: None,
            log_x,
            log_y,
            hide_axes: false,
            show_grid: false,
            ticks_x: None,
            ticks_y: None,
            show_legend: false,
            series: vec![CompiledSeries::Line(LineSeries {
                x: vec![1.0, 10.0, 100.0],
                y: vec![10.0, 100.0, 1000.0],
                label: String::new(),
                color: Color::TABLEAU[0],
                dash: LineDash::Solid,
                marker: crate::series::Marker::None,
                width: 1.5,
            })],
        }
    }

    #[test]
    fn log_axes_map_to_log_coordinates() {
        let panel = sample_panel(true, true);
        let data = data_bounds(&panel);
        let view = view_limits(&panel, data);
        assert!(view.x.0.is_finite() && view.x.1.is_finite());
        assert!(view.y.0.is_finite() && view.y.1.is_finite());
        assert!(view.x.0 < view.x.1);
        assert!(view.y.0 < view.y.1);
    }

    #[test]
    fn tick_plan_includes_decades_on_log_y() {
        let panel = sample_panel(false, true);
        let data = data_bounds(&panel);
        let view = view_limits(&panel, data);
        let plan = tick_plan(&panel, view);
        assert!(plan.y_count >= 2);
        assert!(!plan.custom_y);
    }
}
