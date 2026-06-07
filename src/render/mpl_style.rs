//! Matplotlib rcParams defaults used by the native renderer.

pub const MPL_FONT: &str = "DejaVu Sans";

pub const MPL_LINE_WIDTH: f64 = 1.5;
pub const MPL_AXES_LINE_WIDTH: f64 = 0.8;

pub const MPL_FONT_SIZE: f64 = 10.0;
pub const MPL_TICK_FONT_SIZE: f64 = 10.0;

pub const MPL_SPINE: (u8, u8, u8) = (0, 0, 0);
pub const MPL_GRID: (u8, u8, u8) = (190, 190, 190);

pub const MPL_XMARGIN: f64 = 0.05;
pub const MPL_YMARGIN: f64 = 0.05;

pub const MPL_TITLE_PAD: f64 = 6.0;
pub const MPL_LABEL_PAD: f64 = 4.0;

pub const MPL_XTICK_MAJOR_SIZE: f64 = 3.5;
pub const MPL_XTICK_MAJOR_WIDTH: f64 = 0.8;
pub const MPL_XTICK_MAJOR_PAD: f64 = 3.5;

pub const MPL_SUBPLOT_LEFT: f64 = 0.125;
pub const MPL_SUBPLOT_RIGHT: f64 = 0.9;
pub const MPL_SUBPLOT_BOTTOM: f64 = 0.11;
pub const MPL_SUBPLOT_TOP: f64 = 0.88;

pub const MPL_TIGHT_LEFT: f64 = 0.05;
pub const MPL_TIGHT_RIGHT: f64 = 0.95;
pub const MPL_TIGHT_BOTTOM: f64 = 0.08;
pub const MPL_TIGHT_TOP: f64 = 0.92;

pub const MPL_BOX_EDGE: (u8, u8, u8) = (0, 0, 0);
pub const MPL_BOX_FACE: (u8, u8, u8) = (255, 255, 255);
pub const MPL_MEDIAN: (u8, u8, u8) = (255, 127, 14);
pub const MPL_BOX_LINE_WIDTH: f64 = 1.0;
pub const MPL_MEDIAN_LINE_WIDTH: f64 = 1.0;
pub const MPL_WHISKER_LINE_WIDTH: f64 = 1.0;
pub const MPL_FLIER_SIZE: f64 = 6.0;

pub const CHART_MARGIN_PX: i32 = 8;
pub const LABEL_AREA_LEFT: u32 = 48;
pub const LABEL_AREA_BOTTOM: u32 = 40;

pub fn pt_to_px(points: f64, dpi: u32) -> f64 {
    points * dpi as f64 / 72.0
}

pub fn tick_size_px(dpi: u32) -> i32 {
    pt_to_px(MPL_XTICK_MAJOR_SIZE, dpi).round() as i32
}
