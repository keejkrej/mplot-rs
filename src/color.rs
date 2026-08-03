use plotters::style::{RGBColor, RGBAColor};

/// RGBA color for series and axes styling.
///
/// Alpha is a real opacity channel (0 = transparent, 255 = opaque). Overlapping
/// semi-transparent series darken via backend blending — do **not** bake alpha
/// into RGB against white.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    /// Opacity 0–255 (matplotlib `alpha` × 255).
    pub a: u8,
}

impl Color {
    pub const TABLEAU: [Color; 10] = [
        Color::rgb(31, 119, 180),
        Color::rgb(255, 127, 14),
        Color::rgb(44, 160, 44),
        Color::rgb(214, 39, 40),
        Color::rgb(148, 103, 189),
        Color::rgb(140, 86, 75),
        Color::rgb(227, 119, 194),
        Color::rgb(127, 127, 127),
        Color::rgb(188, 189, 34),
        Color::rgb(23, 190, 207),
    ];

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn hex(value: &str) -> Self {
        let trimmed = value.trim();
        if trimmed.starts_with('#') && trimmed.len() == 7 {
            let r = u8::from_str_radix(&trimmed[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&trimmed[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&trimmed[5..7], 16).unwrap_or(0);
            return Color::rgb(r, g, b);
        }
        if trimmed.starts_with('#') && trimmed.len() == 9 {
            let r = u8::from_str_radix(&trimmed[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&trimmed[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&trimmed[5..7], 16).unwrap_or(0);
            let a = u8::from_str_radix(&trimmed[7..9], 16).unwrap_or(255);
            return Color::rgba(r, g, b, a);
        }
        match trimmed.to_ascii_lowercase().as_str() {
            "green" => Color::rgb(0, 128, 0),
            "red" => Color::rgb(255, 0, 0),
            "blue" => Color::rgb(0, 0, 255),
            "yellow" => Color::rgb(255, 215, 0),
            "gray" | "grey" => Color::rgb(128, 128, 128),
            "black" => Color::rgb(0, 0, 0),
            "white" => Color::rgb(255, 255, 255),
            _ => Color::TABLEAU[0],
        }
    }

    /// Set opacity without changing RGB (matplotlib line `alpha=`).
    pub fn with_alpha(self, alpha: f64) -> Self {
        Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: (alpha.clamp(0.0, 1.0) * 255.0).round() as u8,
        }
    }

    pub fn alpha_f64(self) -> f64 {
        f64::from(self.a) / 255.0
    }

    pub(crate) fn to_rgb(self) -> RGBColor {
        RGBColor(self.r, self.g, self.b)
    }

    /// Plotters color with true alpha for per-series blending.
    pub(crate) fn to_rgba(self) -> RGBAColor {
        RGBAColor(self.r, self.g, self.b, self.alpha_f64())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_alpha_preserves_rgb() {
        let c = Color::rgb(128, 64, 0).with_alpha(0.1);
        assert_eq!(c.r, 128);
        assert_eq!(c.g, 64);
        assert_eq!(c.b, 0);
        assert_eq!(c.a, 26); // round(0.1 * 255)
        let rgba = c.to_rgba();
        assert!((rgba.3 - 0.1).abs() < 0.01);
    }
}
