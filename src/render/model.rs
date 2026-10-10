use crate::color::Color;
use crate::colormap::{Colormap, Normalize};
use crate::panel::{GridStyle, TickFormat, TickLabelRotation, TickLabels};
use crate::series::{
    ArrowStyle, HorizontalAlignment, LineDash, Marker, StepWhere, TextStyle, VerticalAlignment,
    ViolinStyle,
};

/// Layout insets applied during rendering (constrained layout).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PanelLayout {
    pub right_inset_frac: f64,
}

impl PanelLayout {
    pub fn with_colorbar_inset() -> Self {
        PanelLayout {
            right_inset_frac: 0.12,
        }
    }
}

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
pub struct ScatterSeries {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub label: String,
    pub color: Color,
    pub size: f64,
    pub marker: Marker,
    pub alpha: f64,
    pub edge_color: Option<Color>,
    pub edge_width: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ErrorBarSeries {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub yerr: Option<Vec<f64>>,
    pub yerr_lower: Option<Vec<f64>>,
    pub xerr: Option<Vec<f64>>,
    pub xerr_lower: Option<Vec<f64>>,
    pub label: String,
    pub color: Color,
    pub line_width: f64,
    pub dash: LineDash,
    pub marker: Marker,
    pub capsize: f64,
    pub capthick: f64,
    pub elinewidth: f64,
    pub ecolor: Color,
    pub alpha: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StepSeries {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub where_step: StepWhere,
    pub label: String,
    pub color: Color,
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
pub struct BarSeries {
    pub x: Vec<f64>,
    pub heights: Vec<f64>,
    pub color: Color,
    pub width: f64,
    pub baseline: f64,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BarHSeries {
    pub y: Vec<f64>,
    pub widths: Vec<f64>,
    pub color: Color,
    pub height: f64,
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
    pub ha: HorizontalAlignment,
    pub va: VerticalAlignment,
    pub rotation: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AnnotationSeries {
    pub xy: (f64, f64),
    pub xytext: (f64, f64),
    pub text: String,
    pub text_style: TextStyle,
    pub arrow: Option<ArrowStyle>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AxHLineSeries {
    pub y: f64,
    pub color: Color,
    pub width: f64,
    pub dash: LineDash,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AxVLineSeries {
    pub x: f64,
    pub color: Color,
    pub width: f64,
    pub dash: LineDash,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AxHSpanSeries {
    pub ymin: f64,
    pub ymax: f64,
    pub color: Color,
    pub alpha: f64,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AxVSpanSeries {
    pub xmin: f64,
    pub xmax: f64,
    pub color: Color,
    pub alpha: f64,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ViolinSeries {
    pub groups: Vec<Vec<f64>>,
    pub style: ViolinStyle,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompiledSeries {
    Line(LineSeries),
    Scatter(ScatterSeries),
    ErrorBar(ErrorBarSeries),
    Step(StepSeries),
    Boxplot(BoxplotSeries),
    Violin(ViolinSeries),
    Bar(BarSeries),
    BarH(BarHSeries),
    Histogram(HistSeries),
    FillBetween(FillBetweenSeries),
    Image(ImageSeries),
    Contour(ContourSeries),
    Text(TextSeries),
    Annotation(AnnotationSeries),
    AxHLine(AxHLineSeries),
    AxVLine(AxVLineSeries),
    AxHSpan(AxHSpanSeries),
    AxVSpan(AxVSpanSeries),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompiledPanel {
    pub grid_rows: usize,
    pub grid_cols: usize,
    pub row: usize,
    pub col: usize,
    pub rowspan: usize,
    pub colspan: usize,
    pub title: Option<String>,
    pub xlabel: Option<String>,
    pub ylabel: Option<String>,
    pub xrange: Option<(f64, f64)>,
    pub yrange: Option<(f64, f64)>,
    pub invert_x: bool,
    pub invert_y: bool,
    pub log_x: bool,
    pub log_y: bool,
    pub hide_axes: bool,
    pub show_grid: bool,
    pub grid_style: Option<GridStyle>,
    pub ticks_x: Option<TickLabels>,
    pub ticks_y: Option<TickLabels>,
    pub x_tick_format: TickFormat,
    pub y_tick_format: TickFormat,
    pub x_tick_label_rotation: TickLabelRotation,
    pub show_legend: bool,
    pub layout: PanelLayout,
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
    pub constrained_layout: bool,
    pub panels: Vec<CompiledPanel>,
}
