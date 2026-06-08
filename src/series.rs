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

/// Marker style for line series.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Marker {
    #[default]
    None,
    Circle,
    Square,
    Cross,
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
        self.alpha = if alpha < 1e-14 { 1.0 } else { alpha };
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

/// A drawable data series on a panel.
#[derive(Clone, Debug, PartialEq)]
pub enum Series {
    Line {
        x: Vec<f64>,
        y: Vec<f64>,
        style: LineStyle,
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
}
