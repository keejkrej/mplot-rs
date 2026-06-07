use crate::render::model::{BoxplotSeries, CompiledPanel, CompiledSeries};
use crate::render::mpl_style::{MPL_XMARGIN, MPL_YMARGIN};

#[derive(Clone, Debug, PartialEq)]
pub struct BoxStats {
    pub whislo: f64,
    pub q1: f64,
    pub med: f64,
    pub q3: f64,
    pub whishi: f64,
    pub fliers: Vec<f64>,
}

pub(crate) fn panel_bounds(panel: &CompiledPanel) -> (f64, f64, f64, f64) {
    let mut xmin = f64::INFINITY;
    let mut xmax = f64::NEG_INFINITY;
    let mut ymin = f64::INFINITY;
    let mut ymax = f64::NEG_INFINITY;

    for series in &panel.series {
        match series {
            CompiledSeries::Line(curve) => {
                for (x, y) in curve.x.iter().zip(curve.y.iter()) {
                    if x.is_finite() {
                        xmin = xmin.min(*x);
                        xmax = xmax.max(*x);
                    }
                    if y.is_finite() && usable_for_axis(*y, panel.log_y) {
                        ymin = ymin.min(*y);
                        ymax = ymax.max(*y);
                    }
                }
            }
            CompiledSeries::Boxplot(boxes) => {
                let positions = box_positions(boxes);
                for pos in &positions {
                    xmin = xmin.min(*pos - 0.5);
                    xmax = xmax.max(*pos + 0.5);
                }
                for group in &boxes.groups {
                    for value in group {
                        if value.is_finite() && usable_for_axis(*value, panel.log_y) {
                            ymin = ymin.min(*value);
                            ymax = ymax.max(*value);
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

    if panel.xrange.is_none() {
        let span = xmax - xmin;
        xmin -= span * MPL_XMARGIN;
        xmax += span * MPL_XMARGIN;
    }
    if panel.yrange.is_none() {
        let span = ymax - ymin;
        if panel.log_y {
            if ymin > 0.0 {
                let log_span = ymax / ymin;
                ymin /= log_span.powf(MPL_YMARGIN);
                ymax *= log_span.powf(MPL_YMARGIN);
            }
        } else {
            ymin -= span * MPL_YMARGIN;
            ymax += span * MPL_YMARGIN;
        }
    }

    (xmin, xmax, ymin, ymax)
}

pub(crate) fn box_positions(boxes: &BoxplotSeries) -> Vec<f64> {
    if !boxes.positions.is_empty() {
        return boxes.positions.clone();
    }
    (1..=boxes.groups.len()).map(|i| i as f64).collect()
}

pub(crate) fn default_box_width(positions: &[f64]) -> f64 {
    if positions.len() < 2 {
        return 0.5;
    }
    let ptp = positions.last().unwrap() - positions.first().unwrap();
    (0.15 * ptp).clamp(0.15, 0.5)
}

pub(crate) fn box_stats(values: &[f64], whisker: f64) -> Option<BoxStats> {
    let mut data: Vec<f64> = values.iter().copied().filter(|v| v.is_finite()).collect();
    if data.is_empty() {
        return None;
    }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let q1 = numpy_percentile(&data, 25.0);
    let med = numpy_percentile(&data, 50.0);
    let q3 = numpy_percentile(&data, 75.0);
    let iqr = q3 - q1;

    let (loval, hival) = if whisker <= 0.0 {
        (*data.first().unwrap(), *data.last().unwrap())
    } else {
        (q1 - whisker * iqr, q3 + whisker * iqr)
    };

    let wiskhi_candidates: Vec<f64> = data.iter().copied().filter(|v| *v <= hival).collect();
    let whishi = if wiskhi_candidates.is_empty() {
        q3
    } else {
        let max_val = wiskhi_candidates
            .into_iter()
            .fold(f64::NEG_INFINITY, f64::max);
        if max_val < q3 { q3 } else { max_val }
    };

    let wisklo_candidates: Vec<f64> = data.iter().copied().filter(|v| *v >= loval).collect();
    let whislo = if wisklo_candidates.is_empty() {
        q1
    } else {
        let min_val = wisklo_candidates
            .into_iter()
            .fold(f64::INFINITY, f64::min);
        if min_val > q1 { q1 } else { min_val }
    };

    let fliers: Vec<f64> = data
        .iter()
        .copied()
        .filter(|v| *v < whislo || *v > whishi)
        .collect();

    Some(BoxStats {
        whislo,
        q1,
        med,
        q3,
        whishi,
        fliers,
    })
}

fn numpy_percentile(sorted: &[f64], pct: f64) -> f64 {
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
    panel: &CompiledPanel,
    auto: (f64, f64, f64, f64),
) -> (f64, f64, f64, f64) {
    let (mut xmin, mut xmax, mut ymin, mut ymax) = auto;
    if let Some((lo, hi)) = panel.xrange {
        xmin = lo;
        xmax = hi;
    }
    if let Some((lo, hi)) = panel.yrange {
        if panel.log_y {
            ymin = lo.max(1e-300_f64);
            ymax = hi.max(1e-300_f64);
        } else {
            ymin = lo;
            ymax = hi;
        }
    }

    if panel.log_y {
        ymin = transform_axis(ymin, true);
        ymax = transform_axis(ymax, true);
    }

    (xmin, xmax, ymin, ymax)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn box_stats_matches_matplotlib_reference() {
        let data = [1.2, 1.5, 1.8, 2.0, 2.1];
        let stats = box_stats(&data, 1.5).unwrap();
        assert_relative_eq!(stats.q1, 1.5, epsilon = 1e-9);
        assert_relative_eq!(stats.med, 1.8, epsilon = 1e-9);
        assert_relative_eq!(stats.q3, 2.0, epsilon = 1e-9);
        assert_relative_eq!(stats.whislo, 1.2, epsilon = 1e-9);
        assert_relative_eq!(stats.whishi, 2.1, epsilon = 1e-9);
    }
}
