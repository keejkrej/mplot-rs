//! Matplotlib rcParams defaults used by the native renderer.
//!
//! Python reference script (`scripts/mpl_reference.py`) mirrors these via `apply_mplot_rcparams()`:
//!
//! | matplotlib rcParam   | Rust constant              |
//! |----------------------|----------------------------|
//! | `figure.dpi`         | `FIDELITY_DPI` (goldens); `DEFAULT_DPI` for saves |
//! | `font.size`          | `MPL_FONT_SIZE`            |
//! | `axes.titlesize`     | `MPL_FONT_SIZE`            |
//! | `axes.labelsize`     | `MPL_FONT_SIZE`            |
//! | `xtick.labelsize`    | `MPL_TICK_FONT_SIZE`       |
//! | `ytick.labelsize`    | `MPL_TICK_FONT_SIZE`       |
//! | `axes.linewidth`     | `MPL_AXES_LINE_WIDTH`      |
//! | `lines.linewidth`    | `MPL_LINE_WIDTH`           |
//! | `font.family`        | `MPL_FONT`                 |
//! | `axes.titlepad`      | `MPL_TITLE_PAD`            |
//! | `axes.labelpad`      | `MPL_LABEL_PAD`            |
//! | `xtick.major.size`   | `MPL_XTICK_MAJOR_SIZE`     |
//! | `xtick.major.width`  | `MPL_XTICK_MAJOR_WIDTH`    |
//! | `xtick.major.pad`    | `MPL_XTICK_MAJOR_PAD`      |

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
pub const MPL_MARKER_SIZE: f64 = 6.0;

/// Chart margin at the fidelity reference DPI (100).
pub const CHART_MARGIN_PX: i32 = 8;

pub fn pt_to_px(points: f64, dpi: u32) -> f64 {
    points * dpi as f64 / 72.0
}

pub fn tick_size_px(dpi: u32) -> i32 {
    pt_to_px(MPL_XTICK_MAJOR_SIZE, dpi).round() as i32
}

pub fn chart_margin_px(dpi: u32) -> i32 {
    (CHART_MARGIN_PX as f64 * dpi as f64 / crate::constants::FIDELITY_DPI as f64).round() as i32
}

pub fn stroke_width_px(width_pt: f64, dpi: u32) -> u32 {
    pt_to_px(width_pt, dpi).round().max(1.0) as u32
}

pub fn marker_radius_px(size_pt: f64, dpi: u32) -> i32 {
    (pt_to_px(size_pt, dpi) / 2.0).round().max(2.0) as i32
}

/// Left label area sized from tick font, tick padding, and y-axis label pad.
pub fn label_area_left_px(tick_fontsize: f64, dpi: u32) -> u32 {
    let tick_px = pt_to_px(tick_fontsize.max(MPL_TICK_FONT_SIZE * 0.5), dpi);
    let pad_px = pt_to_px(MPL_XTICK_MAJOR_PAD + MPL_LABEL_PAD, dpi);
    (tick_px * 2.2 + pad_px + chart_margin_px(dpi) as f64).round() as u32
}

/// Bottom label area sized from axis/tick fonts, tick marks, and label pad.
pub fn label_area_bottom_px(label_fontsize: f64, tick_fontsize: f64, dpi: u32) -> u32 {
    let label_px = pt_to_px(label_fontsize.max(MPL_FONT_SIZE * 0.5), dpi);
    let tick_px = pt_to_px(tick_fontsize.max(MPL_TICK_FONT_SIZE * 0.5), dpi);
    let tick_mark = tick_size_px(dpi) as f64;
    let pad_px = pt_to_px(MPL_XTICK_MAJOR_PAD + MPL_LABEL_PAD, dpi);
    (label_px * 0.35 + tick_px + tick_mark + pad_px + chart_margin_px(dpi) as f64).round() as u32
}

/// Extra top margin for panel titles (`axes.titlepad`).
pub fn title_pad_px(title_fontsize: f64, dpi: u32) -> i32 {
    let title_px = pt_to_px(title_fontsize.max(MPL_FONT_SIZE * 0.5), dpi);
    let pad_px = pt_to_px(MPL_TITLE_PAD, dpi);
    (pad_px * 0.5 + title_px * 0.1).round() as i32
}
