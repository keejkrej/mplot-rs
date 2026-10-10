use crate::as_vector::{vector_to_f64, AsVector};
use crate::color::Color;
use crate::colormap::{Colormap, Normalize};
use crate::constants::DEFAULT_LINE_WIDTH;
use num_traits::{Num, NumCast};

/// Axis scale type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Scale {
    #[default]
    Linear,
    Log,
}

/// Line dash pattern.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LineDash {
    #[default]
    Solid,
    Dashed,
    DashDot,
    Dotted,
}

/// Marker style for line and scatter series.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Marker {
    #[default]
    None,
    Circle,
    Square,
    Cross,
    Plus,
    TriangleUp,
    TriangleDown,
    Diamond,
    Star,
}

/// Styling for line series.
#[derive(Clone, Debug, PartialEq)]
pub struct LineStyle {
    color: Color,
    width: f64,
    dash: LineDash,
    marker: Marker,
    label: Option<String>,
    alpha: f64,
}

impl Default for LineStyle {
    fn default() -> Self {
        LineStyle {
            color: Color::TABLEAU[0],
            width: DEFAULT_LINE_WIDTH,
            dash: LineDash::Solid,
            marker: Marker::None,
            label: None,
            alpha: 1.0,
        }
    }
}

impl LineStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn dash(mut self, dash: LineDash) -> Self {
        self.dash = dash;
        self
    }

    pub fn marker(mut self, marker: Marker) -> Self {
        self.marker = marker;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn width_value(&self) -> f64 {
        self.width
    }

    pub(crate) fn dash_value(&self) -> LineDash {
        self.dash
    }

    pub(crate) fn marker_value(&self) -> Marker {
        self.marker
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub(crate) fn alpha_value(&self) -> f64 {
        self.alpha
    }
}

/// Styling for boxplot series.
#[derive(Clone, Debug, PartialEq)]
pub struct BoxplotStyle {
    width: Option<f64>,
    whisker: f64,
    positions: Vec<f64>,
    no_fliers: bool,
    patch_artist: bool,
    horizontal: bool,
}

impl Default for BoxplotStyle {
    fn default() -> Self {
        BoxplotStyle {
            width: None,
            whisker: 1.5,
            positions: Vec::new(),
            no_fliers: false,
            patch_artist: false,
            horizontal: false,
        }
    }
}

impl BoxplotStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }

    pub fn whisker(mut self, whisker: f64) -> Self {
        self.whisker = whisker;
        self
    }

    pub fn positions(mut self, positions: &[f64]) -> Self {
        self.positions = positions.to_vec();
        self
    }

    pub fn no_fliers(mut self, flag: bool) -> Self {
        self.no_fliers = flag;
        self
    }

    pub fn patch_artist(mut self, flag: bool) -> Self {
        self.patch_artist = flag;
        self
    }

    pub fn horizontal(mut self, flag: bool) -> Self {
        self.horizontal = flag;
        self
    }

    pub(crate) fn width_value(&self) -> Option<f64> {
        self.width
    }

    pub(crate) fn whisker_value(&self) -> f64 {
        self.whisker
    }

    pub(crate) fn positions_value(&self) -> &[f64] {
        &self.positions
    }

    pub(crate) fn no_fliers_value(&self) -> bool {
        self.no_fliers
    }

    pub(crate) fn patch_artist_value(&self) -> bool {
        self.patch_artist
    }

    pub(crate) fn horizontal_value(&self) -> bool {
        self.horizontal
    }
}

/// Styling for bar charts.
#[derive(Clone, Debug, PartialEq)]
pub struct BarStyle {
    color: Color,
    width: f64,
    baseline: f64,
    label: Option<String>,
}

impl Default for BarStyle {
    fn default() -> Self {
        BarStyle {
            color: Color::TABLEAU[0],
            width: 0.8,
            baseline: 0.0,
            label: None,
        }
    }
}

impl BarStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn baseline(mut self, baseline: f64) -> Self {
        self.baseline = baseline;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn width_value(&self) -> f64 {
        self.width
    }

    pub(crate) fn baseline_value(&self) -> f64 {
        self.baseline
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Styling for histograms.
#[derive(Clone, Debug, PartialEq)]
pub struct HistStyle {
    bins: usize,
    color: Color,
    label: Option<String>,
}

impl Default for HistStyle {
    fn default() -> Self {
        HistStyle {
            bins: 10,
            color: Color::TABLEAU[0],
            label: None,
        }
    }
}

impl HistStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bins(mut self, bins: usize) -> Self {
        self.bins = bins.max(1);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub(crate) fn bins_value(&self) -> usize {
        self.bins
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Styling for filled regions between curves.
#[derive(Clone, Debug, PartialEq)]
pub struct FillBetweenStyle {
    color: Color,
    alpha: f64,
    label: Option<String>,
}

impl Default for FillBetweenStyle {
    fn default() -> Self {
        FillBetweenStyle {
            color: Color::TABLEAU[0],
            alpha: 0.3,
            label: None,
        }
    }
}

impl FillBetweenStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn alpha_value(&self) -> f64 {
        self.alpha
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Styling for raster image (`imshow`) series.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ImageStyle {
    extent: (f64, f64, f64, f64),
    colormap: Colormap,
    normalize: Option<Normalize>,
    show_colorbar: bool,
}

impl Default for ImageStyle {
    fn default() -> Self {
        ImageStyle {
            extent: (0.0, 1.0, 0.0, 1.0),
            colormap: Colormap::Viridis,
            normalize: None,
            show_colorbar: false,
        }
    }
}

impl ImageStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extent(mut self, x0: f64, x1: f64, y0: f64, y1: f64) -> Self {
        self.extent = (x0, x1, y0, y1);
        self
    }

    pub fn colormap(mut self, colormap: Colormap) -> Self {
        self.colormap = colormap;
        self
    }

    pub fn normalize(mut self, normalize: Normalize) -> Self {
        self.normalize = Some(normalize);
        self
    }

    pub fn colorbar(mut self, show: bool) -> Self {
        self.show_colorbar = show;
        self
    }

    pub(crate) fn extent_value(&self) -> (f64, f64, f64, f64) {
        self.extent
    }

    pub(crate) fn colormap_value(&self) -> Colormap {
        self.colormap
    }

    pub(crate) fn normalize_value(&self) -> Option<Normalize> {
        self.normalize
    }

    pub(crate) fn show_colorbar_value(&self) -> bool {
        self.show_colorbar
    }
}

/// Styling for contour plots.
#[derive(Clone, Debug, PartialEq)]
pub struct ContourStyle {
    extent: (f64, f64, f64, f64),
    levels: Vec<f64>,
    line_color: Color,
    colormap: Colormap,
    normalize: Option<Normalize>,
    show_colorbar: bool,
}

impl Default for ContourStyle {
    fn default() -> Self {
        ContourStyle {
            extent: (0.0, 1.0, 0.0, 1.0),
            levels: vec![0.25, 0.5, 0.75],
            line_color: Color::TABLEAU[0],
            colormap: Colormap::Viridis,
            normalize: None,
            show_colorbar: false,
        }
    }
}

impl ContourStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn extent(mut self, x0: f64, x1: f64, y0: f64, y1: f64) -> Self {
        self.extent = (x0, x1, y0, y1);
        self
    }

    pub fn levels(mut self, levels: &[f64]) -> Self {
        self.levels = levels.to_vec();
        self
    }

    pub fn line_color(mut self, color: Color) -> Self {
        self.line_color = color;
        self
    }

    pub fn colormap(mut self, colormap: Colormap) -> Self {
        self.colormap = colormap;
        self
    }

    pub fn normalize(mut self, normalize: Normalize) -> Self {
        self.normalize = Some(normalize);
        self
    }

    pub fn colorbar(mut self, show: bool) -> Self {
        self.show_colorbar = show;
        self
    }

    pub(crate) fn extent_value(&self) -> (f64, f64, f64, f64) {
        self.extent
    }

    pub(crate) fn levels_value(&self) -> &[f64] {
        &self.levels
    }

    pub(crate) fn line_color_value(&self) -> Color {
        self.line_color
    }

    pub(crate) fn colormap_value(&self) -> Colormap {
        self.colormap
    }

    pub(crate) fn normalize_value(&self) -> Option<Normalize> {
        self.normalize
    }

    pub(crate) fn show_colorbar_value(&self) -> bool {
        self.show_colorbar
    }
}

/// Styling for text annotations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextStyle {
    color: Color,
    fontsize: f64,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            color: Color::rgb(0, 0, 0),
            fontsize: 10.0,
        }
    }
}

impl TextStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn fontsize(mut self, size: f64) -> Self {
        self.fontsize = size;
        self
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn fontsize_value(&self) -> f64 {
        self.fontsize
    }
}

/// Styling for scatter series.
#[derive(Clone, Debug, PartialEq)]
pub struct ScatterStyle {
    color: Color,
    size: f64,
    marker: Marker,
    alpha: f64,
    edge_color: Option<Color>,
    edge_width: f64,
    label: Option<String>,
}

impl Default for ScatterStyle {
    fn default() -> Self {
        ScatterStyle {
            color: Color::TABLEAU[0],
            size: 36.0,
            marker: Marker::Circle,
            alpha: 1.0,
            edge_color: None,
            edge_width: 1.0,
            label: None,
        }
    }
}

impl ScatterStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: f64) -> Self {
        self.size = size.max(0.0);
        self
    }

    pub fn marker(mut self, marker: Marker) -> Self {
        self.marker = marker;
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub fn edge_color(mut self, color: Color) -> Self {
        self.edge_color = Some(color);
        self
    }

    pub fn edge_width(mut self, width: f64) -> Self {
        self.edge_width = width.max(0.0);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn size_value(&self) -> f64 {
        self.size
    }

    pub(crate) fn marker_value(&self) -> Marker {
        self.marker
    }

    pub(crate) fn alpha_value(&self) -> f64 {
        self.alpha
    }

    pub(crate) fn edge_color_value(&self) -> Option<Color> {
        self.edge_color
    }

    pub(crate) fn edge_width_value(&self) -> f64 {
        self.edge_width
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Styling for errorbar series.
#[derive(Clone, Debug, PartialEq)]
pub struct ErrorBarStyle {
    yerr: Option<Vec<f64>>,
    yerr_lower: Option<Vec<f64>>,
    xerr: Option<Vec<f64>>,
    xerr_lower: Option<Vec<f64>>,
    color: Color,
    line_width: f64,
    dash: LineDash,
    marker: Marker,
    capsize: f64,
    capthick: f64,
    elinewidth: f64,
    ecolor: Option<Color>,
    alpha: f64,
    label: Option<String>,
}

impl Default for ErrorBarStyle {
    fn default() -> Self {
        ErrorBarStyle {
            yerr: None,
            yerr_lower: None,
            xerr: None,
            xerr_lower: None,
            color: Color::TABLEAU[0],
            line_width: DEFAULT_LINE_WIDTH,
            dash: LineDash::Solid,
            marker: Marker::Circle,
            capsize: 3.0,
            capthick: 1.0,
            elinewidth: 1.5,
            ecolor: None,
            alpha: 1.0,
            label: None,
        }
    }
}

impl ErrorBarStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn yerr<'a, T, U>(mut self, yerr: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.yerr = Some(vector_to_f64(yerr));
        self
    }

    pub fn yerr_asymmetric<'a, T, U>(mut self, lower: &'a T, upper: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.yerr_lower = Some(vector_to_f64(lower));
        self.yerr = Some(vector_to_f64(upper));
        self
    }

    pub fn xerr<'a, T, U>(mut self, xerr: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.xerr = Some(vector_to_f64(xerr));
        self
    }

    pub fn xerr_asymmetric<'a, T, U>(mut self, lower: &'a T, upper: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.xerr_lower = Some(vector_to_f64(lower));
        self.xerr = Some(vector_to_f64(upper));
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn line_width(mut self, width: f64) -> Self {
        self.line_width = width;
        self
    }

    pub fn dash(mut self, dash: LineDash) -> Self {
        self.dash = dash;
        self
    }

    pub fn marker(mut self, marker: Marker) -> Self {
        self.marker = marker;
        self
    }

    pub fn capsize(mut self, capsize: f64) -> Self {
        self.capsize = capsize.max(0.0);
        self
    }

    pub fn capthick(mut self, capthick: f64) -> Self {
        self.capthick = capthick.max(0.0);
        self
    }

    pub fn elinewidth(mut self, elinewidth: f64) -> Self {
        self.elinewidth = elinewidth.max(0.0);
        self
    }

    pub fn ecolor(mut self, color: Color) -> Self {
        self.ecolor = Some(color);
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub(crate) fn yerr_value(&self) -> Option<&[f64]> {
        self.yerr.as_deref()
    }

    pub(crate) fn yerr_lower_value(&self) -> Option<&[f64]> {
        self.yerr_lower.as_deref()
    }

    pub(crate) fn xerr_value(&self) -> Option<&[f64]> {
        self.xerr.as_deref()
    }

    pub(crate) fn xerr_lower_value(&self) -> Option<&[f64]> {
        self.xerr_lower.as_deref()
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn line_width_value(&self) -> f64 {
        self.line_width
    }

    pub(crate) fn dash_value(&self) -> LineDash {
        self.dash
    }

    pub(crate) fn marker_value(&self) -> Marker {
        self.marker
    }

    pub(crate) fn capsize_value(&self) -> f64 {
        self.capsize
    }

    pub(crate) fn capthick_value(&self) -> f64 {
        self.capthick
    }

    pub(crate) fn elinewidth_value(&self) -> f64 {
        self.elinewidth
    }

    pub(crate) fn ecolor_value(&self) -> Option<Color> {
        self.ecolor
    }

    pub(crate) fn alpha_value(&self) -> f64 {
        self.alpha
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Where the step occurs in a step plot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepWhere {
    #[default]
    Pre,
    Post,
    Mid,
}

/// Styling for step plots.
#[derive(Clone, Debug, PartialEq)]
pub struct StepStyle {
    where_step: StepWhere,
    color: Color,
    width: f64,
    dash: LineDash,
    label: Option<String>,
    alpha: f64,
}

impl Default for StepStyle {
    fn default() -> Self {
        StepStyle {
            where_step: StepWhere::Pre,
            color: Color::TABLEAU[0],
            width: DEFAULT_LINE_WIDTH,
            dash: LineDash::Solid,
            label: None,
            alpha: 1.0,
        }
    }
}

impl StepStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn where_step(mut self, where_step: StepWhere) -> Self {
        self.where_step = where_step;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn dash(mut self, dash: LineDash) -> Self {
        self.dash = dash;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub(crate) fn where_step_value(&self) -> StepWhere {
        self.where_step
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn width_value(&self) -> f64 {
        self.width
    }

    pub(crate) fn dash_value(&self) -> LineDash {
        self.dash
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub(crate) fn alpha_value(&self) -> f64 {
        self.alpha
    }
}

/// Styling for horizontal bar charts.
#[derive(Clone, Debug, PartialEq)]
pub struct BarHStyle {
    color: Color,
    height: f64,
    baseline: f64,
    label: Option<String>,
}

impl Default for BarHStyle {
    fn default() -> Self {
        BarHStyle {
            color: Color::TABLEAU[0],
            height: 0.8,
            baseline: 0.0,
            label: None,
        }
    }
}

impl BarHStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn baseline(mut self, baseline: f64) -> Self {
        self.baseline = baseline;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn height_value(&self) -> f64 {
        self.height
    }

    pub(crate) fn baseline_value(&self) -> f64 {
        self.baseline
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// A drawable data series on a panel.
#[derive(Clone, Debug, PartialEq)]
pub enum Series {
    Line {
        x: Vec<f64>,
        y: Vec<f64>,
        style: LineStyle,
    },
    Scatter {
        x: Vec<f64>,
        y: Vec<f64>,
        style: ScatterStyle,
    },
    ErrorBar {
        x: Vec<f64>,
        y: Vec<f64>,
        style: ErrorBarStyle,
    },
    Step {
        x: Vec<f64>,
        y: Vec<f64>,
        style: StepStyle,
    },
    Boxplot {
        groups: Vec<Vec<f64>>,
        style: BoxplotStyle,
    },
    Bar {
        x: Vec<f64>,
        heights: Vec<f64>,
        style: BarStyle,
    },
    BarH {
        y: Vec<f64>,
        widths: Vec<f64>,
        style: BarHStyle,
    },
    Histogram {
        data: Vec<f64>,
        style: HistStyle,
    },
    FillBetween {
        x: Vec<f64>,
        y1: Vec<f64>,
        y2: Vec<f64>,
        style: FillBetweenStyle,
    },
    Image {
        data: Vec<f64>,
        width: usize,
        height: usize,
        style: ImageStyle,
    },
    Contour {
        data: Vec<f64>,
        width: usize,
        height: usize,
        style: ContourStyle,
    },
    Text {
        x: f64,
        y: f64,
        text: String,
        style: TextStyle,
    },
    AxHLine {
        y: f64,
        style: LineStyle,
    },
    AxVLine {
        x: f64,
        style: LineStyle,
    },
    AxHSpan {
        ymin: f64,
        ymax: f64,
        style: FillBetweenStyle,
    },
    AxVSpan {
        xmin: f64,
        xmax: f64,
        style: FillBetweenStyle,
    },
}

impl Series {
    pub fn line<'a, T, U>(x: &'a T, y: &'a T, style: LineStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::Line {
            x: vector_to_f64(x),
            y: vector_to_f64(y),
            style,
        }
    }

    pub fn scatter<'a, T, U>(x: &'a T, y: &'a T, style: ScatterStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::Scatter {
            x: vector_to_f64(x),
            y: vector_to_f64(y),
            style,
        }
    }

    pub fn errorbar<'a, T, U>(x: &'a T, y: &'a T, style: ErrorBarStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::ErrorBar {
            x: vector_to_f64(x),
            y: vector_to_f64(y),
            style,
        }
    }

    pub fn step<'a, T, U>(x: &'a T, y: &'a T, style: StepStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::Step {
            x: vector_to_f64(x),
            y: vector_to_f64(y),
            style,
        }
    }

    pub fn boxplot(groups: &[Vec<f64>], style: BoxplotStyle) -> Self {
        Series::Boxplot {
            groups: groups.to_vec(),
            style,
        }
    }

    pub fn bar(x: &[f64], heights: &[f64], style: BarStyle) -> Self {
        Series::Bar {
            x: x.to_vec(),
            heights: heights.to_vec(),
            style,
        }
    }

    pub fn barh<'a, T, U>(y: &'a T, widths: &'a T, style: BarHStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::BarH {
            y: vector_to_f64(y),
            widths: vector_to_f64(widths),
            style,
        }
    }

    pub fn histogram(data: &[f64], style: HistStyle) -> Self {
        Series::Histogram {
            data: data.to_vec(),
            style,
        }
    }

    pub fn fill_between<'a, T, U>(x: &'a T, y1: &'a T, y2: &'a T, style: FillBetweenStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::FillBetween {
            x: vector_to_f64(x),
            y1: vector_to_f64(y1),
            y2: vector_to_f64(y2),
            style,
        }
    }

    pub fn image(data: Vec<f64>, width: usize, height: usize, style: ImageStyle) -> Self {
        Series::Image {
            data,
            width,
            height,
            style,
        }
    }

    pub fn contour(data: Vec<f64>, width: usize, height: usize, style: ContourStyle) -> Self {
        Series::Contour {
            data,
            width,
            height,
            style,
        }
    }

    pub fn text(x: f64, y: f64, text: impl Into<String>, style: TextStyle) -> Self {
        Series::Text {
            x,
            y,
            text: text.into(),
            style,
        }
    }

    pub fn axhline(y: f64, style: LineStyle) -> Self {
        Series::AxHLine { y, style }
    }

    pub fn axvline(x: f64, style: LineStyle) -> Self {
        Series::AxVLine { x, style }
    }

    pub fn axhspan(ymin: f64, ymax: f64, style: FillBetweenStyle) -> Self {
        Series::AxHSpan { ymin, ymax, style }
    }

    pub fn axvspan(xmin: f64, xmax: f64, style: FillBetweenStyle) -> Self {
        Series::AxVSpan { xmin, xmax, style }
    }
}
