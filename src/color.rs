use plotters::style::RGBColor;

pub fn parse_color(name: &str) -> RGBColor {
    let trimmed = name.trim();
    if trimmed.starts_with('#') && trimmed.len() == 7 {
        let r = u8::from_str_radix(&trimmed[1..3], 16).unwrap_or(0);
        let g = u8::from_str_radix(&trimmed[3..5], 16).unwrap_or(0);
        let b = u8::from_str_radix(&trimmed[5..7], 16).unwrap_or(0);
        return RGBColor(r, g, b);
    }

    match trimmed.to_ascii_lowercase().as_str() {
        "green" => RGBColor(0, 128, 0),
        "red" => RGBColor(255, 0, 0),
        "blue" => RGBColor(0, 0, 255),
        "yellow" => RGBColor(255, 215, 0),
        "gray" | "grey" => RGBColor(128, 128, 128),
        "black" => RGBColor(0, 0, 0),
        "white" => RGBColor(255, 255, 255),
        _ => RGBColor(31, 119, 180),
    }
}

pub fn with_alpha(color: RGBColor, alpha: f64) -> RGBColor {
    let a = alpha.clamp(0.0, 1.0);
    RGBColor(
        ((f64::from(color.0) * a) + (255.0 * (1.0 - a))) as u8,
        ((f64::from(color.1) * a) + (255.0 * (1.0 - a))) as u8,
        ((f64::from(color.2) * a) + (255.0 * (1.0 - a))) as u8,
    )
}
