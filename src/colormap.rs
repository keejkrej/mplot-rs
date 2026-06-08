use crate::color::Color;

/// Named colormap for image and contour series.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Colormap {
    #[default]
    Viridis,
    Grayscale,
}

/// Maps data values to `[0, 1]` before colormap lookup.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Normalize {
    pub vmin: f64,
    pub vmax: f64,
}

impl Normalize {
    pub fn new(vmin: f64, vmax: f64) -> Self {
        Normalize { vmin, vmax }
    }

    pub fn of_slice(data: &[f64]) -> Self {
        let mut vmin = f64::INFINITY;
        let mut vmax = f64::NEG_INFINITY;
        for value in data {
            if value.is_finite() {
                vmin = vmin.min(*value);
                vmax = vmax.max(*value);
            }
        }
        if !vmin.is_finite() {
            vmin = 0.0;
            vmax = 1.0;
        }
        if (vmax - vmin).abs() < 1e-12 {
            vmax = vmin + 1.0;
        }
        Normalize { vmin, vmax }
    }

    pub fn apply(self, value: f64) -> f64 {
        if !value.is_finite() {
            return 0.0;
        }
        let span = self.vmax - self.vmin;
        if span <= 0.0 {
            return 0.0;
        }
        ((value - self.vmin) / span).clamp(0.0, 1.0)
    }
}

impl Colormap {
    pub fn map(self, t: f64) -> Color {
        let t = t.clamp(0.0, 1.0);
        match self {
            Colormap::Grayscale => {
                let v = (t * 255.0).round() as u8;
                Color::rgb(v, v, v)
            }
            Colormap::Viridis => viridis(t),
        }
    }
}

fn viridis(t: f64) -> Color {
    // Matplotlib viridis control points (approximate).
    const STOPS: [(f64, u8, u8, u8); 5] = [
        (0.0, 68, 1, 84),
        (0.25, 59, 82, 139),
        (0.5, 33, 145, 140),
        (0.75, 94, 201, 98),
        (1.0, 253, 231, 37),
    ];
    for window in STOPS.windows(2) {
        let (t0, r0, g0, b0) = window[0];
        let (t1, r1, g1, b1) = window[1];
        if t <= t1 {
            let w = if (t1 - t0).abs() < 1e-12 {
                0.0
            } else {
                (t - t0) / (t1 - t0)
            };
            return Color::rgb(
                lerp_u8(r0, r1, w),
                lerp_u8(g0, g1, w),
                lerp_u8(b0, b1, w),
            );
        }
    }
    let (_, r, g, b) = STOPS[STOPS.len() - 1];
    Color::rgb(r, g, b)
}

fn lerp_u8(a: u8, b: u8, t: f64) -> u8 {
    ((f64::from(a) * (1.0 - t)) + (f64::from(b) * t)).round() as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_maps_into_unit_interval() {
        let norm = Normalize::new(0.0, 10.0);
        assert!((norm.apply(5.0) - 0.5).abs() < 1e-9);
    }

    #[test]
    fn viridis_endpoints() {
        let lo = Colormap::Viridis.map(0.0);
        let hi = Colormap::Viridis.map(1.0);
        assert!(lo.b > lo.r);
        assert!(hi.r > hi.b);
    }
}
