//! Matplotlib-inspired tick locators and label formatters.

pub fn linear_ticks(min: f64, max: f64, max_ticks: usize) -> Vec<f64> {
    if !min.is_finite() || !max.is_finite() || min >= max {
        return vec![min];
    }
    let span = max - min;
    if span <= 0.0 {
        return vec![min];
    }

    let raw_step = span / max_ticks as f64;
    let magnitude = 10f64.powf(raw_step.log10().floor());
    let normalized = raw_step / magnitude;
    let step = if normalized <= 1.0 {
        magnitude
    } else if normalized <= 2.0 {
        2.0 * magnitude
    } else if normalized <= 5.0 {
        5.0 * magnitude
    } else {
        10.0 * magnitude
    };

    let start = (min / step).ceil() * step;
    let mut ticks = Vec::new();
    let mut value = start;
    while value <= max + step * 1e-9 {
        if value >= min - step * 1e-9 {
            ticks.push(trim_float(value));
        }
        value += step;
        if ticks.len() > max_ticks + 2 {
            break;
        }
    }
    if ticks.is_empty() {
        ticks.push(trim_float(min));
        if max > min {
            ticks.push(trim_float(max));
        }
    }
    ticks
}

pub fn log_ticks_data(min_pos: f64, max_pos: f64) -> Vec<f64> {
    if !min_pos.is_finite() || !max_pos.is_finite() || min_pos <= 0.0 || max_pos <= 0.0 {
        return Vec::new();
    }
    let mut lo = min_pos;
    let mut hi = max_pos;
    if lo > hi {
        std::mem::swap(&mut lo, &mut hi);
    }

    let mut ticks = Vec::new();
    let start_exp = lo.log10().floor() as i32;
    let end_exp = hi.log10().ceil() as i32;
    for exp in start_exp..=end_exp {
        let value = 10f64.powi(exp);
        if value >= lo * (1.0 - 1e-9) && value <= hi * (1.0 + 1e-9) {
            ticks.push(value);
        }
    }
    if ticks.is_empty() {
        ticks.push(lo);
    }
    ticks
}

pub fn format_linear(value: f64) -> String {
    if !value.is_finite() {
        return String::new();
    }
    if (value.fract()).abs() < 1e-6 {
        format!("{:.0}", value)
    } else if (value * 10.0).fract().abs() < 1e-6 {
        format!("{:.1}", value)
    } else if (value * 100.0).fract().abs() < 1e-6 {
        format!("{:.2}", value)
    } else {
        format!("{:.3}", value)
    }
}

pub fn format_log_data(value: f64) -> String {
    if !value.is_finite() || value <= 0.0 {
        return String::new();
    }
    if value >= 1000.0 || value <= 0.001 {
        format!("{value:.0e}")
    } else if (value.fract()).abs() < 1e-6 {
        format!("{:.0}", value)
    } else {
        format!("{:.2}", value)
    }
}

pub fn format_log_axis_coord(log10_value: f64) -> String {
    format_log_data(10f64.powf(log10_value))
}

fn trim_float(value: f64) -> f64 {
    (value * 1_000_000_000.0).round() / 1_000_000_000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_ticks_are_decades() {
        let ticks = log_ticks_data(8.0, 55.0);
        assert!(ticks.contains(&10.0));
        assert!(!ticks.contains(&100.0));
    }

    #[test]
    fn linear_ticks_cover_range() {
        let ticks = linear_ticks(0.0, 4.0, 6);
        assert!(!ticks.is_empty());
        assert!(*ticks.first().unwrap() >= 0.0);
        assert!(*ticks.last().unwrap() <= 4.0);
    }
}
