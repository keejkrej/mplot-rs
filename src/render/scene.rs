use crate::render::model::BoxplotSeries;

#[derive(Clone, Debug, PartialEq)]
pub struct BoxStats {
    pub whislo: f64,
    pub q1: f64,
    pub med: f64,
    pub q3: f64,
    pub whishi: f64,
    pub fliers: Vec<f64>,
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
        if max_val < q3 {
            q3
        } else {
            max_val
        }
    };

    let wisklo_candidates: Vec<f64> = data.iter().copied().filter(|v| *v >= loval).collect();
    let whislo = if wisklo_candidates.is_empty() {
        q1
    } else {
        let min_val = wisklo_candidates.into_iter().fold(f64::INFINITY, f64::min);
        if min_val > q1 {
            q1
        } else {
            min_val
        }
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

#[derive(Clone, Debug, PartialEq)]
pub struct ViolinStats {
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub median: f64,
    pub coords: Vec<f64>,
    pub vals: Vec<f64>,
}

pub(crate) fn violin_stats(values: &[f64], points: usize) -> Option<ViolinStats> {
    let mut clean: Vec<f64> = values.iter().copied().filter(|v| v.is_finite()).collect();
    if clean.len() < 2 {
        return None;
    }
    clean.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = clean.len() as f64;
    let min = *clean.first().unwrap();
    let max = *clean.last().unwrap();
    if (max - min).abs() < 1e-12 {
        return None;
    }
    let mean = clean.iter().sum::<f64>() / n;
    let median = numpy_percentile(&clean, 50.0);

    let variance = clean.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / (n - 1.0);
    let std_dev = variance.sqrt().max(1e-12);

    let factor = n.powf(-0.2);
    let bw = factor * std_dev;

    let pts = points.max(2);
    let step = (max - min) / (pts - 1) as f64;
    let coords: Vec<f64> = (0..pts).map(|i| min + i as f64 * step).collect();

    let inv_cov = 1.0 / (bw * bw);
    let norm_factor = (2.0 * std::f64::consts::PI * bw * bw).sqrt() * n;

    let mut vals: Vec<f64> = coords
        .iter()
        .map(|&coord| {
            let sum: f64 = clean
                .iter()
                .map(|&x| {
                    let diff = x - coord;
                    (-0.5 * diff * diff * inv_cov).exp()
                })
                .sum();
            sum / norm_factor
        })
        .collect();

    let max_density = vals.iter().copied().fold(0.0, f64::max);
    if max_density > 0.0 {
        for v in &mut vals {
            *v /= max_density;
        }
    }

    Some(ViolinStats {
        min,
        max,
        mean,
        median,
        coords,
        vals,
    })
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

    #[test]
    fn violin_stats_matches_matplotlib_reference() {
        let data = [1.0, 2.0, 2.5, 3.0, 3.5, 4.0, 5.0];
        let stats = violin_stats(&data, 5).unwrap();
        assert_relative_eq!(stats.min, 1.0, epsilon = 1e-9);
        assert_relative_eq!(stats.max, 5.0, epsilon = 1e-9);
        assert_relative_eq!(stats.mean, 3.0, epsilon = 1e-9);
        assert_relative_eq!(stats.median, 3.0, epsilon = 1e-9);
        assert_eq!(stats.coords, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_relative_eq!(stats.vals[2], 1.0, epsilon = 1e-9);
        assert_relative_eq!(stats.vals[0], 0.47845075, epsilon = 1e-6);
        assert_relative_eq!(stats.vals[1], 0.82571619, epsilon = 1e-6);
    }
}
