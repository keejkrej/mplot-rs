use plotters::style::RGBColor;

/// RGB color for series and axes styling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
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
        Color { r, g, b }
    }

    pub fn hex(value: &str) -> Self {
        let trimmed = value.trim();
        if trimmed.starts_with('#') && trimmed.len() == 7 {
            let r = u8::from_str_radix(&trimmed[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&trimmed[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&trimmed[5..7], 16).unwrap_or(0);
            return Color { r, g, b };
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

    pub fn with_alpha(self, alpha: f64) -> Self {
        let a = alpha.clamp(0.0, 1.0);
        Color {
            r: ((f64::from(self.r) * a) + (255.0 * (1.0 - a))) as u8,
            g: ((f64::from(self.g) * a) + (255.0 * (1.0 - a))) as u8,
            b: ((f64::from(self.b) * a) + (255.0 * (1.0 - a))) as u8,
        }
    }

    pub(crate) fn to_rgb(self) -> RGBColor {
        RGBColor(self.r, self.g, self.b)
    }
}
