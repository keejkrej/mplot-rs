/// Axis scale semantics (matplotlib-inspired).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AxisScale {
    #[default]
    Linear,
    Log,
}

impl AxisScale {
    pub fn from_log_flag(log: bool) -> Self {
        if log {
            AxisScale::Log
        } else {
            AxisScale::Linear
        }
    }

    pub fn is_log(self) -> bool {
        matches!(self, AxisScale::Log)
    }

    pub fn data_to_axis(self, value: f64) -> f64 {
        match self {
            AxisScale::Linear => value,
            AxisScale::Log => value.log10(),
        }
    }

    pub fn axis_to_data(self, axis: f64) -> f64 {
        match self {
            AxisScale::Linear => axis,
            AxisScale::Log => 10f64.powf(axis),
        }
    }

    pub fn usable(self, value: f64) -> bool {
        match self {
            AxisScale::Linear => value.is_finite(),
            AxisScale::Log => value.is_finite() && value > 0.0,
        }
    }

    /// Expand `(min, max)` in data space before mapping to axis coordinates.
    pub fn expand_data_limits(self, min: f64, max: f64, margin: f64) -> (f64, f64) {
        if !min.is_finite() || !max.is_finite() {
            return (min, max);
        }
        if (max - min).abs() < 1e-12 {
            return (min, max + 1.0);
        }

        match self {
            AxisScale::Linear => {
                let span = max - min;
                (min - span * margin, max + span * margin)
            }
            AxisScale::Log => {
                if min <= 0.0 || max <= 0.0 {
                    return (min, max);
                }
                let ratio = max / min;
                (min / ratio.powf(margin), max * ratio.powf(margin))
            }
        }
    }

    pub fn clamp_user_range(self, min: f64, max: f64) -> (f64, f64) {
        match self {
            AxisScale::Linear => (min, max),
            AxisScale::Log => (min.max(1e-300_f64), max.max(1e-300_f64)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_scale_maps_decades() {
        let scale = AxisScale::Log;
        assert!((scale.data_to_axis(10.0) - 1.0).abs() < 1e-9);
        assert!((scale.axis_to_data(2.0) - 100.0).abs() < 1e-9);
    }

    #[test]
    fn log_margin_expands_multiplicatively() {
        let scale = AxisScale::Log;
        let (lo, hi) = scale.expand_data_limits(10.0, 100.0, 0.05);
        assert!(lo < 10.0);
        assert!(hi > 100.0);
    }
}
