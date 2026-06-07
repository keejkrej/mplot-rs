use crate::panel::TickLabels;
use crate::series::LineDash;

#[derive(Clone, Debug, PartialEq)]
pub struct LineSeries {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub label: String,
    pub color: crate::color::Color,
    pub dash: LineDash,
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
pub enum CompiledSeries {
    Line(LineSeries),
    Boxplot(BoxplotSeries),
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
