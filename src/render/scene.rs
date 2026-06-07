use crate::boxplot::BoxplotData;
use crate::graph::GraphEntity;
use crate::plot::Panel;

pub(crate) fn panel_bounds(panel: &Panel) -> (f64, f64, f64, f64) {
    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;

    for entity in panel.entities() {
        match entity {
            GraphEntity::Curve(curve) => {
                for (x, y) in curve.x.iter().zip(curve.y.iter()) {
                    if x.is_finite() {
                        xmin = xmin.min(*x);
                        xmax = xmax.max(*x);
                    }
                    if y.is_finite() && usable_for_axis(*y, panel.log_y()) {
                        ymin = ymin.min(transform_axis(*y, panel.log_y()));
                        ymax = ymax.max(transform_axis(*y, panel.log_y()));
                    }
                }
            }
            GraphEntity::Boxplot(boxes) => {
                let positions = box_positions(&boxes);
                for pos in &positions {
                    xmin = xmin.min(*pos - 0.5);
                    xmax = xmax.max(*pos + 0.5);
                }
                for group in &boxes.groups {
                    for value in group {
                        if value.is_finite() && usable_for_axis(*value, panel.log_y()) {
                            let v = transform_axis(*value, panel.log_y());
                            ymin = ymin.min(v);
                            ymax = ymax.max(v);
                        }
                    }
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

    (xmin, xmax, ymin, ymax)
}

pub(crate) fn box_positions(boxes: &BoxplotData) -> Vec<f64> {
    if !boxes.positions.is_empty() {
        return boxes.positions.clone();
    }
    (1..=boxes.groups.len()).map(|i| i as f64).collect()
}

pub(crate) fn box_stats(values: &[f64], whisker: f64) -> Option<(f64, f64, f64, f64, f64)> {
    let mut data: Vec<f64> = values.iter().copied().filter(|v| v.is_finite()).collect();
    if data.is_empty() {
        return None;
    }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let q1 = percentile(&data, 25.0);
    let median = percentile(&data, 50.0);
    let q3 = percentile(&data, 75.0);
    let iqr = q3 - q1;
    let lower_fence = q1 - whisker * iqr;
    let upper_fence = q3 + whisker * iqr;
    let low = data
        .iter()
        .copied()
        .filter(|v| *v >= lower_fence)
        .next()
        .unwrap_or(q1);
    let high = data
        .iter()
        .copied()
        .filter(|v| *v <= upper_fence)
        .last()
        .unwrap_or(q3);
    Some((low, q1, median, q3, high))
}

fn percentile(sorted: &[f64], pct: f64) -> f64 {
    if sorted.len() == 1 {
        return sorted[0];
    }
    let rank = (pct / 100.0) * (sorted.len() - 1) as f64;
    let lower = rank.floor() as usize;
    let upper = rank.ceil() as usize;
    if lower == upper {
        sorted[lower]
    } else {
        let weight = rank - lower as f64;
        sorted[lower] * (1.0 - weight) + sorted[upper] * weight
    }
}

pub(crate) fn transform_axis(value: f64, log: bool) -> f64 {
    if log {
        value.log10()
    } else {
        value
    }
}

pub(crate) fn usable_for_axis(value: f64, log: bool) -> bool {
    if log {
        value > 0.0
    } else {
        value.is_finite()
    }
}

pub(crate) fn apply_panel_limits(
    panel: &Panel,
    auto: (f64, f64, f64, f64),
) -> (f64, f64, f64, f64) {
    let (mut xmin, mut xmax, mut ymin, mut ymax) = auto;
    if let Some((lo, hi)) = panel.xrange() {
        xmin = lo;
        xmax = hi;
    }
    if let Some((lo, hi)) = panel.yrange() {
        if panel.log_y() {
            ymin = transform_axis(lo.max(1e-300_f64), true);
            ymax = transform_axis(hi.max(1e-300_f64), true);
        } else {
            ymin = lo;
            ymax = hi;
        }
    }
    (xmin, xmax, ymin, ymax)
}
