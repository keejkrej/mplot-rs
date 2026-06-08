use crate::colormap::{Colormap, Normalize};
use crate::panel::TickLabels;
use crate::color::Color;
use crate::series::{LineDash, Marker};

#[derive(Clone, Debug, PartialEq)]
pub struct LineSeries {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub label: String,
    pub color: Color,
    pub dash: LineDash,
    pub marker: Marker,
    pub width: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BoxplotSeries {
    pub groups: Vec<Vec<f64>>,
    pub horizontal: bool,
    pub whisker: f64,
    pub positions: Vec<f64>,
    pub width: Option<f64>,
    pub no_fliers: bool,
    pub patch_artist: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BarSeries {
    pub x: Vec<f64>,
    pub heights: Vec<f64>,
    pub color: Color,
    pub width: f64,
    pub baseline: f64,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct HistSeries {
    pub data: Vec<f64>,
    pub bins: usize,
    pub color: Color,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FillBetweenSeries {
    pub x: Vec<f64>,
    pub y1: Vec<f64>,
    pub y2: Vec<f64>,
    pub color: Color,
    pub alpha: f64,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImageSeries {
    pub data: Vec<f64>,
    pub width: usize,
    pub height: usize,
    pub extent: (f64, f64, f64, f64),
    pub colormap: Colormap,
    pub normalize: Normalize,
    pub show_colorbar: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContourSeries {
    pub data: Vec<f64>,
    pub width: usize,
    pub height: usize,
    pub extent: (f64, f64, f64, f64),
    pub levels: Vec<f64>,
    pub line_color: Color,
    pub colormap: Colormap,
    pub normalize: Normalize,
    pub show_colorbar: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextSeries {
    pub x: f64,
    pub y: f64,
    pub text: String,
    pub color: Color,
    pub fontsize: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompiledSeries {
    Line(LineSeries),
    Boxplot(BoxplotSeries),
    Bar(BarSeries),
    Histogram(HistSeries),
    FillBetween(FillBetweenSeries),
    Image(ImageSeries),
    Contour(ContourSeries),
    Text(TextSeries),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledPanel {
    pub rows: usize,
    pub cols: usize,
    pub index: usize,
    pub title: Option<String>,
    pub xlabel: Option<String>,
    pub ylabel: Option<String>,
    pub xrange: Option<(f64, f64)>,
    pub yrange: Option<(f64, f64)>,
    pub log_x: bool,
    pub log_y: bool,
    pub hide_axes: bool,
    pub show_grid: bool,
    pub ticks_x: Option<TickLabels>,
    pub ticks_y: Option<TickLabels>,
    pub show_legend: bool,
    pub series: Vec<CompiledSeries>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledFigure {
    pub width_in: f64,
    pub height_in: f64,
    pub dpi: u32,
    pub h_gap: f64,
    pub v_gap: f64,
    pub label_fontsize: f64,
    pub tick_fontsize: f64,
    pub title_fontsize: f64,
    pub save_tight: bool,
    pub save_pad_inches: Option<f64>,
    pub panels: Vec<CompiledPanel>,
}
