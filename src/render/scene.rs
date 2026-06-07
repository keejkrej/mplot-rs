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
