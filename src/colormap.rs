use crate::color::Color;

/// Named colormap for image and contour series.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Colormap {
    #[default]
    Viridis,
    Grayscale,
    Plasma,
    Inferno,
    Magma,
    Cividis,
    Coolwarm,
    Turbo,
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
            Colormap::Plasma => interpolate_stops(t, &PLASMA_STOPS),
            Colormap::Inferno => interpolate_stops(t, &INFERNO_STOPS),
            Colormap::Magma => interpolate_stops(t, &MAGMA_STOPS),
            Colormap::Cividis => interpolate_stops(t, &CIVIDIS_STOPS),
            Colormap::Coolwarm => interpolate_stops(t, &COOLWARM_STOPS),
            Colormap::Turbo => interpolate_stops(t, &TURBO_STOPS),
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
    interpolate_stops(t, &STOPS)
}

fn interpolate_stops(t: f64, stops: &[(f64, u8, u8, u8)]) -> Color {
    for window in stops.windows(2) {
        let (t0, r0, g0, b0) = window[0];
        let (t1, r1, g1, b1) = window[1];
        if t <= t1 {
            let w = if (t1 - t0).abs() < 1e-12 {
                0.0
            } else {
                (t - t0) / (t1 - t0)
            };
            return Color::rgb(lerp_u8(r0, r1, w), lerp_u8(g0, g1, w), lerp_u8(b0, b1, w));
        }
    }
    let (_, r, g, b) = stops[stops.len() - 1];
    Color::rgb(r, g, b)
}

const PLASMA_STOPS: [(f64, u8, u8, u8); 9] = [
    (0.000, 13, 8, 135),
    (0.125, 76, 2, 161),
    (0.250, 126, 3, 168),
    (0.375, 170, 35, 149),
    (0.500, 204, 71, 120),
    (0.625, 230, 108, 92),
    (0.750, 248, 149, 64),
    (0.875, 253, 197, 39),
    (1.000, 240, 249, 33),
];

const INFERNO_STOPS: [(f64, u8, u8, u8); 9] = [
    (0.000, 0, 0, 4),
    (0.125, 33, 12, 74),
    (0.250, 87, 16, 110),
    (0.375, 138, 34, 106),
    (0.500, 188, 55, 84),
    (0.625, 228, 90, 49),
    (0.750, 249, 142, 9),
    (0.875, 249, 203, 53),
    (1.000, 252, 255, 164),
];

const MAGMA_STOPS: [(f64, u8, u8, u8); 9] = [
    (0.000, 0, 0, 4),
    (0.125, 29, 17, 71),
    (0.250, 81, 18, 124),
    (0.375, 131, 38, 129),
    (0.500, 183, 55, 121),
    (0.625, 231, 82, 99),
    (0.750, 252, 137, 97),
    (0.875, 254, 196, 136),
    (1.000, 252, 253, 191),
];

const CIVIDIS_STOPS: [(f64, u8, u8, u8); 9] = [
    (0.000, 0, 34, 78),
    (0.125, 26, 56, 111),
    (0.250, 67, 78, 108),
    (0.375, 97, 101, 111),
    (0.500, 125, 124, 120),
    (0.625, 155, 148, 118),
    (0.750, 188, 174, 108),
    (0.875, 222, 201, 88),
    (1.000, 254, 232, 56),
];

const COOLWARM_STOPS: [(f64, u8, u8, u8); 9] = [
    (0.000, 59, 76, 192),
    (0.125, 98, 130, 234),
    (0.250, 141, 176, 254),
    (0.375, 185, 208, 249),
    (0.500, 221, 220, 220),
    (0.625, 245, 196, 172),
    (0.750, 244, 152, 122),
    (0.875, 221, 95, 75),
    (1.000, 180, 4, 38),
];

const TURBO_STOPS: [(f64, u8, u8, u8); 9] = [
    (0.000, 48, 18, 59),
    (0.125, 70, 107, 227),
    (0.250, 40, 188, 235),
    (0.375, 50, 242, 152),
    (0.500, 164, 252, 60),
    (0.625, 238, 207, 58),
    (0.750, 251, 126, 33),
    (0.875, 208, 47, 5),
    (1.000, 122, 4, 3),
];

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

    #[test]
    fn new_colormaps_produce_valid_colors() {
        for cmap in [
            Colormap::Plasma,
            Colormap::Inferno,
            Colormap::Magma,
            Colormap::Cividis,
            Colormap::Coolwarm,
            Colormap::Turbo,
        ] {
            let c0 = cmap.map(0.0);
            let c1 = cmap.map(0.5);
            let c2 = cmap.map(1.0);
            assert!(c0.a > 0 && c1.a > 0 && c2.a > 0);
        }
    }
}
