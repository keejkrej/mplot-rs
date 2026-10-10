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
    TriangleLeft,
    TriangleRight,
    Diamond,
    Star,
    Pentagon,
    Hexagon,
    Point,
    VLine,
    HLine,
}

/// Horizontal text alignment (matplotlib `ha`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum HorizontalAlignment {
    #[default]
    Left,
    Center,
    Right,
}

/// Vertical text alignment (matplotlib `va`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
    #[default]
    Baseline,
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

/// Styling for violin plots (matplotlib `ax.violinplot(...)`).
#[derive(Clone, Debug, PartialEq)]
pub struct ViolinStyle {
    width: f64,
    positions: Vec<f64>,
    face_color: Option<Color>,
    edge_color: Option<Color>,
    show_means: bool,
    show_extrema: bool,
    show_medians: bool,
    vert: bool,
    alpha: f64,
    points: usize,
    label: Option<String>,
}

impl Default for ViolinStyle {
    fn default() -> Self {
        ViolinStyle {
            width: 0.5,
            positions: Vec::new(),
            face_color: None,
            edge_color: None,
            show_means: false,
            show_extrema: true,
            show_medians: false,
            vert: true,
            alpha: 0.3,
            points: 100,
            label: None,
        }
    }
}

impl ViolinStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width.max(0.0);
        self
    }

    pub fn positions<'a, T, U>(mut self, positions: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.positions = vector_to_f64(positions);
        self
    }

    pub fn face_color(mut self, color: Color) -> Self {
        self.face_color = Some(color);
        self
    }

    pub fn edge_color(mut self, color: Color) -> Self {
        self.edge_color = Some(color);
        self
    }

    pub fn show_means(mut self, show: bool) -> Self {
        self.show_means = show;
        self
    }

    pub fn show_extrema(mut self, show: bool) -> Self {
        self.show_extrema = show;
        self
    }

    pub fn show_medians(mut self, show: bool) -> Self {
        self.show_medians = show;
        self
    }

    pub fn vert(mut self, vert: bool) -> Self {
        self.vert = vert;
        self
    }

    pub fn horizontal(mut self, flag: bool) -> Self {
        self.vert = !flag;
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub fn points(mut self, points: usize) -> Self {
        self.points = points.max(2);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub(crate) fn width_value(&self) -> f64 {
        self.width
    }

    pub(crate) fn positions_value(&self) -> &[f64] {
        &self.positions
    }

    pub(crate) fn face_color_value(&self) -> Option<Color> {
        self.face_color
    }

    pub(crate) fn edge_color_value(&self) -> Option<Color> {
        self.edge_color
    }

    pub(crate) fn show_means_value(&self) -> bool {
        self.show_means
    }

    pub(crate) fn show_extrema_value(&self) -> bool {
        self.show_extrema
    }

    pub(crate) fn show_medians_value(&self) -> bool {
        self.show_medians
    }

    pub(crate) fn vert_value(&self) -> bool {
        self.vert
    }

    pub(crate) fn alpha_value(&self) -> f64 {
        self.alpha
    }

    pub(crate) fn points_value(&self) -> usize {
        self.points
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Styling for bar charts.
#[derive(Clone, Debug, PartialEq)]
pub struct BarStyle {
    color: Color,
    width: f64,
    baseline: f64,
    bottom: Option<Vec<f64>>,
    yerr: Option<Vec<f64>>,
    yerr_lower: Option<Vec<f64>>,
    capsize: f64,
    edge_color: Option<Color>,
    edge_width: f64,
    alpha: f64,
    label: Option<String>,
}

impl Default for BarStyle {
    fn default() -> Self {
        BarStyle {
            color: Color::TABLEAU[0],
            width: 0.8,
            baseline: 0.0,
            bottom: None,
            yerr: None,
            yerr_lower: None,
            capsize: 3.0,
            edge_color: None,
            edge_width: 1.0,
            alpha: 1.0,
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

    pub fn bottom<'a, T, U>(mut self, bottom: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.bottom = Some(vector_to_f64(bottom));
        self
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

    pub fn capsize(mut self, capsize: f64) -> Self {
        self.capsize = capsize.max(0.0);
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

    pub(crate) fn width_value(&self) -> f64 {
        self.width
    }

    pub(crate) fn baseline_value(&self) -> f64 {
        self.baseline
    }

    pub(crate) fn bottom_value(&self) -> Option<&[f64]> {
        self.bottom.as_deref()
    }

    pub(crate) fn yerr_value(&self) -> Option<&[f64]> {
        self.yerr.as_deref()
    }

    pub(crate) fn yerr_lower_value(&self) -> Option<&[f64]> {
        self.yerr_lower.as_deref()
    }

    pub(crate) fn capsize_value(&self) -> f64 {
        self.capsize
    }

    pub(crate) fn edge_color_value(&self) -> Option<Color> {
        self.edge_color
    }

    pub(crate) fn edge_width_value(&self) -> f64 {
        self.edge_width
    }

    pub(crate) fn alpha_value(&self) -> f64 {
        self.alpha
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

/// Styling for bounding box around text (matplotlib `bbox=dict(...)`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BboxStyle {
    face_color: Option<Color>,
    edge_color: Option<Color>,
    edge_width: f64,
    pad: f64,
    alpha: f64,
}

impl Default for BboxStyle {
    fn default() -> Self {
        BboxStyle {
            face_color: Some(Color::rgb(255, 255, 255)),
            edge_color: Some(Color::rgb(0, 0, 0)),
            edge_width: 1.0,
            pad: 3.0,
            alpha: 1.0,
        }
    }
}

impl BboxStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn face_color(mut self, color: Color) -> Self {
        self.face_color = Some(color);
        self
    }

    pub fn edge_color(mut self, color: Color) -> Self {
        self.edge_color = Some(color);
        self
    }

    pub fn edge_width(mut self, width: f64) -> Self {
        self.edge_width = width;
        self
    }

    pub fn pad(mut self, pad: f64) -> Self {
        self.pad = pad;
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub fn face_color_value(&self) -> Option<Color> {
        self.face_color
    }

    pub fn edge_color_value(&self) -> Option<Color> {
        self.edge_color
    }

    pub fn edge_width_value(&self) -> f64 {
        self.edge_width
    }

    pub fn pad_value(&self) -> f64 {
        self.pad
    }

    pub fn alpha_value(&self) -> f64 {
        self.alpha
    }
}

/// Orientation for event plots (matplotlib `ax.eventplot`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventplotOrientation {
    Horizontal,
    Vertical,
}

/// Styling for event plots (matplotlib `ax.eventplot`).
#[derive(Clone, Debug, PartialEq)]
pub struct EventplotStyle {
    orientation: EventplotOrientation,
    lineoffset: f64,
    linelength: f64,
    color: Color,
    linewidth: f64,
    dash: LineDash,
    alpha: f64,
    label: String,
}

impl Default for EventplotStyle {
    fn default() -> Self {
        EventplotStyle {
            orientation: EventplotOrientation::Horizontal,
            lineoffset: 1.0,
            linelength: 1.0,
            color: Color::TABLEAU[0],
            linewidth: 1.5,
            dash: LineDash::Solid,
            alpha: 1.0,
            label: String::new(),
        }
    }
}

impl EventplotStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn orientation(mut self, orientation: EventplotOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn lineoffset(mut self, offset: f64) -> Self {
        self.lineoffset = offset;
        self
    }

    pub fn linelength(mut self, length: f64) -> Self {
        self.linelength = length;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn linewidth(mut self, width: f64) -> Self {
        self.linewidth = width;
        self
    }

    pub fn dash(mut self, dash: LineDash) -> Self {
        self.dash = dash;
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn orientation_value(&self) -> EventplotOrientation {
        self.orientation
    }

    pub fn lineoffset_value(&self) -> f64 {
        self.lineoffset
    }

    pub fn linelength_value(&self) -> f64 {
        self.linelength
    }

    pub fn color_value(&self) -> Color {
        self.color
    }

    pub fn linewidth_value(&self) -> f64 {
        self.linewidth
    }

    pub fn dash_value(&self) -> LineDash {
        self.dash
    }

    pub fn alpha_value(&self) -> f64 {
        self.alpha
    }

    pub fn label_value(&self) -> &str {
        &self.label
    }
}

/// Styling for text annotations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextStyle {
    color: Color,
    fontsize: f64,
    ha: HorizontalAlignment,
    va: VerticalAlignment,
    rotation: f64,
    bbox: Option<BboxStyle>,
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle {
            color: Color::rgb(0, 0, 0),
            fontsize: 10.0,
            ha: HorizontalAlignment::Left,
            va: VerticalAlignment::Baseline,
            rotation: 0.0,
            bbox: None,
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

    pub fn ha(mut self, ha: HorizontalAlignment) -> Self {
        self.ha = ha;
        self
    }

    pub fn va(mut self, va: VerticalAlignment) -> Self {
        self.va = va;
        self
    }

    pub fn rotation(mut self, degrees: f64) -> Self {
        self.rotation = degrees;
        self
    }

    pub fn bbox(mut self, bbox: BboxStyle) -> Self {
        self.bbox = Some(bbox);
        self
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn fontsize_value(&self) -> f64 {
        self.fontsize
    }

    pub(crate) fn ha_value(&self) -> HorizontalAlignment {
        self.ha
    }

    pub(crate) fn va_value(&self) -> VerticalAlignment {
        self.va
    }

    pub(crate) fn rotation_value(&self) -> f64 {
        self.rotation
    }

    pub(crate) fn bbox_value(&self) -> Option<BboxStyle> {
        self.bbox
    }
}

/// Styling for callout arrows in annotations.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ArrowStyle {
    color: Color,
    width: f64,
    head_width: f64,
    head_length: f64,
    shrink: f64,
}

impl Default for ArrowStyle {
    fn default() -> Self {
        ArrowStyle {
            color: Color::rgb(0, 0, 0),
            width: 1.0,
            head_width: 6.0,
            head_length: 7.0,
            shrink: 2.0,
        }
    }
}

impl ArrowStyle {
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

    pub fn head_width(mut self, head_width: f64) -> Self {
        self.head_width = head_width;
        self
    }

    pub fn head_length(mut self, head_length: f64) -> Self {
        self.head_length = head_length;
        self
    }

    pub fn shrink(mut self, shrink: f64) -> Self {
        self.shrink = shrink;
        self
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn width_value(&self) -> f64 {
        self.width
    }

    pub(crate) fn head_width_value(&self) -> f64 {
        self.head_width
    }

    pub(crate) fn head_length_value(&self) -> f64 {
        self.head_length
    }

    pub(crate) fn shrink_value(&self) -> f64 {
        self.shrink
    }
}

/// Styling for annotations with optional callout arrows (matplotlib `ax.annotate`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AnnotationStyle {
    text_style: TextStyle,
    arrow: Option<ArrowStyle>,
}

impl AnnotationStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text_style(mut self, style: TextStyle) -> Self {
        self.text_style = style;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.text_style = self.text_style.color(color);
        self
    }

    pub fn fontsize(mut self, size: f64) -> Self {
        self.text_style = self.text_style.fontsize(size);
        self
    }

    pub fn ha(mut self, ha: HorizontalAlignment) -> Self {
        self.text_style = self.text_style.ha(ha);
        self
    }

    pub fn va(mut self, va: VerticalAlignment) -> Self {
        self.text_style = self.text_style.va(va);
        self
    }

    pub fn arrow(mut self, arrow: ArrowStyle) -> Self {
        self.arrow = Some(arrow);
        self
    }

    pub fn bbox(mut self, bbox: BboxStyle) -> Self {
        self.text_style = self.text_style.bbox(bbox);
        self
    }

    pub(crate) fn text_style_value(&self) -> TextStyle {
        self.text_style
    }

    pub(crate) fn arrow_value(&self) -> Option<ArrowStyle> {
        self.arrow
    }
}

/// Styling for scatter series.
#[derive(Clone, Debug, PartialEq)]
pub struct ScatterStyle {
    color: Color,
    size: f64,
    sizes: Option<Vec<f64>>,
    values: Option<Vec<f64>>,
    colormap: Option<Colormap>,
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
            sizes: None,
            values: None,
            colormap: None,
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

    pub fn sizes<'a, T, U>(mut self, sizes: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.sizes = Some(vector_to_f64(sizes));
        self
    }

    pub fn values<'a, T, U>(mut self, values: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.values = Some(vector_to_f64(values));
        self
    }

    pub fn c<'a, T, U>(self, values: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.values(values)
    }

    pub fn colormap(mut self, colormap: Colormap) -> Self {
        self.colormap = Some(colormap);
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

    pub(crate) fn sizes_value(&self) -> Option<&[f64]> {
        self.sizes.as_deref()
    }

    pub(crate) fn values_value(&self) -> Option<&[f64]> {
        self.values.as_deref()
    }

    pub(crate) fn colormap_value(&self) -> Option<Colormap> {
        self.colormap
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

    pub fn cap_size(self, cap_size: f64) -> Self {
        self.capsize(cap_size)
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

    pub fn step_where(self, step_where: StepWhere) -> Self {
        self.where_step(step_where)
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
    left: Option<Vec<f64>>,
    xerr: Option<Vec<f64>>,
    xerr_lower: Option<Vec<f64>>,
    capsize: f64,
    edge_color: Option<Color>,
    edge_width: f64,
    alpha: f64,
    label: Option<String>,
}

impl Default for BarHStyle {
    fn default() -> Self {
        BarHStyle {
            color: Color::TABLEAU[0],
            height: 0.8,
            baseline: 0.0,
            left: None,
            xerr: None,
            xerr_lower: None,
            capsize: 3.0,
            edge_color: None,
            edge_width: 1.0,
            alpha: 1.0,
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

    pub fn left<'a, T, U>(mut self, left: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.left = Some(vector_to_f64(left));
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

    pub fn capsize(mut self, capsize: f64) -> Self {
        self.capsize = capsize.max(0.0);
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

    pub(crate) fn height_value(&self) -> f64 {
        self.height
    }

    pub(crate) fn baseline_value(&self) -> f64 {
        self.baseline
    }

    pub(crate) fn left_value(&self) -> Option<&[f64]> {
        self.left.as_deref()
    }

    pub(crate) fn xerr_value(&self) -> Option<&[f64]> {
        self.xerr.as_deref()
    }

    pub(crate) fn xerr_lower_value(&self) -> Option<&[f64]> {
        self.xerr_lower.as_deref()
    }

    pub(crate) fn capsize_value(&self) -> f64 {
        self.capsize
    }

    pub(crate) fn edge_color_value(&self) -> Option<Color> {
        self.edge_color
    }

    pub(crate) fn edge_width_value(&self) -> f64 {
        self.edge_width
    }

    pub(crate) fn alpha_value(&self) -> f64 {
        self.alpha
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Styling for stem plots (matplotlib `ax.stem(...)`).
#[derive(Clone, Debug, PartialEq)]
pub struct StemStyle {
    bottom: f64,
    color: Color,
    line_width: f64,
    line_dash: LineDash,
    marker: Marker,
    marker_size: f64,
    marker_color: Option<Color>,
    base_color: Option<Color>,
    base_width: f64,
    horizontal: bool,
    label: Option<String>,
}

impl Default for StemStyle {
    fn default() -> Self {
        StemStyle {
            bottom: 0.0,
            color: Color::TABLEAU[0],
            line_width: DEFAULT_LINE_WIDTH,
            line_dash: LineDash::Solid,
            marker: Marker::Circle,
            marker_size: 4.0,
            marker_color: None,
            base_color: None,
            base_width: 1.0,
            horizontal: false,
            label: None,
        }
    }
}

impl StemStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bottom(mut self, bottom: f64) -> Self {
        self.bottom = bottom;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn line_width(mut self, width: f64) -> Self {
        self.line_width = width.max(0.0);
        self
    }

    pub fn line_dash(mut self, dash: LineDash) -> Self {
        self.line_dash = dash;
        self
    }

    pub fn marker(mut self, marker: Marker) -> Self {
        self.marker = marker;
        self
    }

    pub fn marker_size(mut self, size: f64) -> Self {
        self.marker_size = size.max(0.0);
        self
    }

    pub fn marker_color(mut self, color: Color) -> Self {
        self.marker_color = Some(color);
        self
    }

    pub fn base_color(mut self, color: Color) -> Self {
        self.base_color = Some(color);
        self
    }

    pub fn base_width(mut self, width: f64) -> Self {
        self.base_width = width.max(0.0);
        self
    }

    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub(crate) fn bottom_value(&self) -> f64 {
        self.bottom
    }

    pub(crate) fn color_value(&self) -> Color {
        self.color
    }

    pub(crate) fn line_width_value(&self) -> f64 {
        self.line_width
    }

    pub(crate) fn line_dash_value(&self) -> LineDash {
        self.line_dash
    }

    pub(crate) fn marker_value(&self) -> Marker {
        self.marker
    }

    pub(crate) fn marker_size_value(&self) -> f64 {
        self.marker_size
    }

    pub(crate) fn marker_color_value(&self) -> Option<Color> {
        self.marker_color
    }

    pub(crate) fn base_color_value(&self) -> Option<Color> {
        self.base_color
    }

    pub(crate) fn base_width_value(&self) -> f64 {
        self.base_width
    }

    pub(crate) fn horizontal_value(&self) -> bool {
        self.horizontal
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Styling for stacked area plots (matplotlib `ax.stackplot(...)`).
#[derive(Clone, Debug, PartialEq)]
pub struct StackplotStyle {
    colors: Vec<Color>,
    labels: Vec<String>,
    alpha: f64,
    baseline: f64,
}

impl Default for StackplotStyle {
    fn default() -> Self {
        StackplotStyle {
            colors: Vec::new(),
            labels: Vec::new(),
            alpha: 1.0,
            baseline: 0.0,
        }
    }
}

impl StackplotStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn colors(mut self, colors: Vec<Color>) -> Self {
        self.colors = colors;
        self
    }

    pub fn labels<S: Into<String>>(mut self, labels: impl IntoIterator<Item = S>) -> Self {
        self.labels = labels.into_iter().map(Into::into).collect();
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub fn baseline(mut self, baseline: f64) -> Self {
        self.baseline = baseline;
        self
    }

    pub(crate) fn colors_value(&self) -> &[Color] {
        &self.colors
    }

    pub(crate) fn labels_value(&self) -> &[String] {
        &self.labels
    }

    pub(crate) fn alpha_value(&self) -> f64 {
        self.alpha
    }

    pub(crate) fn baseline_value(&self) -> f64 {
        self.baseline
    }
}

/// Styling for stairs / step histogram plots (matplotlib `ax.stairs(...)`).
#[derive(Clone, Debug, PartialEq)]
pub struct StairsStyle {
    color: Color,
    width: f64,
    dash: LineDash,
    fill: bool,
    fill_color: Option<Color>,
    fill_alpha: f64,
    baseline: f64,
    horizontal: bool,
    label: Option<String>,
}

impl Default for StairsStyle {
    fn default() -> Self {
        StairsStyle {
            color: Color::TABLEAU[0],
            width: DEFAULT_LINE_WIDTH,
            dash: LineDash::Solid,
            fill: false,
            fill_color: None,
            fill_alpha: 0.3,
            baseline: 0.0,
            horizontal: false,
            label: None,
        }
    }
}

impl StairsStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width.max(0.0);
        self
    }

    pub fn dash(mut self, dash: LineDash) -> Self {
        self.dash = dash;
        self
    }

    pub fn fill(mut self, fill: bool) -> Self {
        self.fill = fill;
        self
    }

    pub fn fill_color(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    pub fn fill_alpha(mut self, alpha: f64) -> Self {
        self.fill_alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub fn baseline(mut self, baseline: f64) -> Self {
        self.baseline = baseline;
        self
    }

    pub fn horizontal(mut self, horizontal: bool) -> Self {
        self.horizontal = horizontal;
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

    pub(crate) fn dash_value(&self) -> LineDash {
        self.dash
    }

    pub(crate) fn fill_value(&self) -> bool {
        self.fill
    }

    pub(crate) fn fill_color_value(&self) -> Option<Color> {
        self.fill_color
    }

    pub(crate) fn fill_alpha_value(&self) -> f64 {
        self.fill_alpha
    }

    pub(crate) fn baseline_value(&self) -> f64 {
        self.baseline
    }

    pub(crate) fn horizontal_value(&self) -> bool {
        self.horizontal
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Styling for quiver / 2D vector field plots (matplotlib `ax.quiver(...)`).
#[derive(Clone, Debug, PartialEq)]
pub struct QuiverStyle {
    color: Color,
    width: f64,
    scale: f64,
    arrow_size: f64,
    alpha: f64,
    label: Option<String>,
}

impl Default for QuiverStyle {
    fn default() -> Self {
        QuiverStyle {
            color: Color::TABLEAU[0],
            width: DEFAULT_LINE_WIDTH,
            scale: 1.0,
            arrow_size: 4.0,
            alpha: 1.0,
            label: None,
        }
    }
}

impl QuiverStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width.max(0.0);
        self
    }

    pub fn scale(mut self, scale: f64) -> Self {
        self.scale = scale;
        self
    }

    pub fn arrow_size(mut self, size: f64) -> Self {
        self.arrow_size = size.max(0.0);
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

    pub(crate) fn width_value(&self) -> f64 {
        self.width
    }

    pub(crate) fn scale_value(&self) -> f64 {
        self.scale
    }

    pub(crate) fn arrow_size_value(&self) -> f64 {
        self.arrow_size
    }

    pub(crate) fn alpha_value(&self) -> f64 {
        self.alpha
    }

    pub(crate) fn label_value(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

/// Styling for pie charts (matplotlib `ax.pie(...)`).
#[derive(Clone, Debug, PartialEq)]
pub struct PieStyle {
    labels: Vec<String>,
    colors: Vec<Color>,
    explode: Vec<f64>,
    autopct: Option<String>,
    start_angle: f64,
    edge_color: Option<Color>,
    edge_width: f64,
    radius: f64,
}

impl Default for PieStyle {
    fn default() -> Self {
        PieStyle {
            labels: Vec::new(),
            colors: Vec::new(),
            explode: Vec::new(),
            autopct: None,
            start_angle: 0.0,
            edge_color: Some(Color::rgb(255, 255, 255)),
            edge_width: 1.0,
            radius: 1.0,
        }
    }
}

impl PieStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn labels<S: AsRef<str>>(mut self, labels: &[S]) -> Self {
        self.labels = labels.iter().map(|s| s.as_ref().to_string()).collect();
        self
    }

    pub fn colors(mut self, colors: Vec<Color>) -> Self {
        self.colors = colors;
        self
    }

    pub fn explode<'a, T, U>(mut self, explode: &'a T) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        self.explode = vector_to_f64(explode);
        self
    }

    pub fn autopct(mut self, autopct: impl Into<String>) -> Self {
        self.autopct = Some(autopct.into());
        self
    }

    pub fn start_angle(mut self, angle: f64) -> Self {
        self.start_angle = angle;
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

    pub fn radius(mut self, radius: f64) -> Self {
        self.radius = radius.max(0.0);
        self
    }

    pub(crate) fn labels_value(&self) -> &[String] {
        &self.labels
    }

    pub(crate) fn colors_value(&self) -> &[Color] {
        &self.colors
    }

    pub(crate) fn explode_value(&self) -> &[f64] {
        &self.explode
    }

    pub(crate) fn autopct_value(&self) -> Option<&str> {
        self.autopct.as_deref()
    }

    pub(crate) fn start_angle_value(&self) -> f64 {
        self.start_angle
    }

    pub(crate) fn edge_color_value(&self) -> Option<Color> {
        self.edge_color
    }

    pub(crate) fn edge_width_value(&self) -> f64 {
        self.edge_width
    }

    pub(crate) fn radius_value(&self) -> f64 {
        self.radius
    }
}

/// Styling for 2D histograms (matplotlib `ax.hist2d(...)`).
#[derive(Clone, Debug, PartialEq)]
pub struct Hist2DStyle {
    bins: (usize, usize),
    range_x: Option<(f64, f64)>,
    range_y: Option<(f64, f64)>,
    colormap: Colormap,
    show_colorbar: bool,
    density: bool,
}

impl Default for Hist2DStyle {
    fn default() -> Self {
        Hist2DStyle {
            bins: (10, 10),
            range_x: None,
            range_y: None,
            colormap: Colormap::Viridis,
            show_colorbar: false,
            density: false,
        }
    }
}

impl Hist2DStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bins(mut self, nx: usize, ny: usize) -> Self {
        self.bins = (nx.max(1), ny.max(1));
        self
    }

    pub fn bin_count(mut self, count: usize) -> Self {
        self.bins = (count.max(1), count.max(1));
        self
    }

    pub fn range_x(mut self, min: f64, max: f64) -> Self {
        self.range_x = Some((min, max));
        self
    }

    pub fn range_y(mut self, min: f64, max: f64) -> Self {
        self.range_y = Some((min, max));
        self
    }

    pub fn colormap(mut self, colormap: Colormap) -> Self {
        self.colormap = colormap;
        self
    }

    pub fn show_colorbar(mut self, show: bool) -> Self {
        self.show_colorbar = show;
        self
    }

    pub fn density(mut self, density: bool) -> Self {
        self.density = density;
        self
    }

    pub(crate) fn bins_value(&self) -> (usize, usize) {
        self.bins
    }

    pub(crate) fn range_x_value(&self) -> Option<(f64, f64)> {
        self.range_x
    }

    pub(crate) fn range_y_value(&self) -> Option<(f64, f64)> {
        self.range_y
    }

    pub(crate) fn colormap_value(&self) -> Colormap {
        self.colormap
    }

    pub(crate) fn show_colorbar_value(&self) -> bool {
        self.show_colorbar
    }

    pub(crate) fn density_value(&self) -> bool {
        self.density
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
    FillBetweenX {
        y: Vec<f64>,
        x1: Vec<f64>,
        x2: Vec<f64>,
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
    Annotation {
        xy: (f64, f64),
        xytext: (f64, f64),
        text: String,
        style: AnnotationStyle,
    },
    Violin {
        groups: Vec<Vec<f64>>,
        style: ViolinStyle,
    },
    Stem {
        x: Vec<f64>,
        y: Vec<f64>,
        style: StemStyle,
    },
    Stackplot {
        x: Vec<f64>,
        ys: Vec<Vec<f64>>,
        style: StackplotStyle,
    },
    Stairs {
        values: Vec<f64>,
        edges: Option<Vec<f64>>,
        style: StairsStyle,
    },
    Quiver {
        x: Vec<f64>,
        y: Vec<f64>,
        u: Vec<f64>,
        v: Vec<f64>,
        style: QuiverStyle,
    },
    Pie {
        values: Vec<f64>,
        style: PieStyle,
    },
    Hist2D {
        x: Vec<f64>,
        y: Vec<f64>,
        style: Hist2DStyle,
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
    AxLine {
        p1: (f64, f64),
        p2: Option<(f64, f64)>,
        slope: Option<f64>,
        style: LineStyle,
    },
    HLines {
        y: Vec<f64>,
        xmin: Vec<f64>,
        xmax: Vec<f64>,
        style: LineStyle,
    },
    VLines {
        x: Vec<f64>,
        ymin: Vec<f64>,
        ymax: Vec<f64>,
        style: LineStyle,
    },
    EventPlot {
        positions: Vec<f64>,
        style: EventplotStyle,
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

    pub fn annotate(
        text: impl Into<String>,
        xy: (f64, f64),
        xytext: (f64, f64),
        style: AnnotationStyle,
    ) -> Self {
        Series::Annotation {
            xy,
            xytext,
            text: text.into(),
            style,
        }
    }

    pub fn violin(groups: Vec<Vec<f64>>, style: ViolinStyle) -> Self {
        Series::Violin { groups, style }
    }

    pub fn axhline(y: f64, style: LineStyle) -> Self {
        Series::AxHLine { y, style }
    }

    pub fn axvline(x: f64, style: LineStyle) -> Self {
        Series::AxVLine { x, style }
    }

    pub fn fill_betweenx<'a, T, U>(y: &'a T, x1: &'a T, x2: &'a T, style: FillBetweenStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::FillBetweenX {
            y: vector_to_f64(y),
            x1: vector_to_f64(x1),
            x2: vector_to_f64(x2),
            style,
        }
    }

    pub fn stem<'a, Tx, Ty, Ux, Uy>(x: &'a Tx, y: &'a Ty, style: StemStyle) -> Self
    where
        Tx: AsVector<'a, Ux>,
        Ty: AsVector<'a, Uy>,
        Ux: 'a + Num + NumCast + Copy,
        Uy: 'a + Num + NumCast + Copy,
    {
        Series::Stem {
            x: vector_to_f64(x),
            y: vector_to_f64(y),
            style,
        }
    }

    pub fn stackplot<'a, T, U>(x: &'a T, ys: &[&'a T], style: StackplotStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::Stackplot {
            x: vector_to_f64(x),
            ys: ys.iter().map(|y| vector_to_f64(*y)).collect(),
            style,
        }
    }

    pub fn stairs<'a, Tv, Te, Uv, Ue>(
        values: &'a Tv,
        edges: Option<&'a Te>,
        style: StairsStyle,
    ) -> Self
    where
        Tv: AsVector<'a, Uv>,
        Te: AsVector<'a, Ue>,
        Uv: 'a + Num + NumCast + Copy,
        Ue: 'a + Num + NumCast + Copy,
    {
        Series::Stairs {
            values: vector_to_f64(values),
            edges: edges.map(vector_to_f64),
            style,
        }
    }

    pub fn quiver<'a, T, U>(x: &'a T, y: &'a T, u: &'a T, v: &'a T, style: QuiverStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::Quiver {
            x: vector_to_f64(x),
            y: vector_to_f64(y),
            u: vector_to_f64(u),
            v: vector_to_f64(v),
            style,
        }
    }

    pub fn pie<'a, T, U>(values: &'a T, style: PieStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::Pie {
            values: vector_to_f64(values),
            style,
        }
    }

    pub fn hist2d<'a, Tx, Ty, Ux, Uy>(x: &'a Tx, y: &'a Ty, style: Hist2DStyle) -> Self
    where
        Tx: AsVector<'a, Ux>,
        Ty: AsVector<'a, Uy>,
        Ux: 'a + Num + NumCast + Copy,
        Uy: 'a + Num + NumCast + Copy,
    {
        Series::Hist2D {
            x: vector_to_f64(x),
            y: vector_to_f64(y),
            style,
        }
    }

    pub fn axhspan(ymin: f64, ymax: f64, style: FillBetweenStyle) -> Self {
        Series::AxHSpan { ymin, ymax, style }
    }

    pub fn axvspan(xmin: f64, xmax: f64, style: FillBetweenStyle) -> Self {
        Series::AxVSpan { xmin, xmax, style }
    }

    pub fn axline(p1: (f64, f64), p2: (f64, f64), style: LineStyle) -> Self {
        Series::AxLine {
            p1,
            p2: Some(p2),
            slope: None,
            style,
        }
    }

    pub fn axline_slope(p1: (f64, f64), slope: f64, style: LineStyle) -> Self {
        Series::AxLine {
            p1,
            p2: None,
            slope: Some(slope),
            style,
        }
    }

    pub fn hlines<'a, Ty, Tmin, Tmax, Uy, Umin, Umax>(
        y: &'a Ty,
        xmin: &'a Tmin,
        xmax: &'a Tmax,
        style: LineStyle,
    ) -> Self
    where
        Ty: AsVector<'a, Uy>,
        Tmin: AsVector<'a, Umin>,
        Tmax: AsVector<'a, Umax>,
        Uy: 'a + Num + NumCast + Copy,
        Umin: 'a + Num + NumCast + Copy,
        Umax: 'a + Num + NumCast + Copy,
    {
        let ys = vector_to_f64(y);
        let mut xmins = vector_to_f64(xmin);
        let mut xmaxs = vector_to_f64(xmax);
        if xmins.len() == 1 && ys.len() > 1 {
            xmins = vec![xmins[0]; ys.len()];
        }
        if xmaxs.len() == 1 && ys.len() > 1 {
            xmaxs = vec![xmaxs[0]; ys.len()];
        }
        Series::HLines {
            y: ys,
            xmin: xmins,
            xmax: xmaxs,
            style,
        }
    }

    pub fn vlines<'a, Tx, Tmin, Tmax, Ux, Umin, Umax>(
        x: &'a Tx,
        ymin: &'a Tmin,
        ymax: &'a Tmax,
        style: LineStyle,
    ) -> Self
    where
        Tx: AsVector<'a, Ux>,
        Tmin: AsVector<'a, Umin>,
        Tmax: AsVector<'a, Umax>,
        Ux: 'a + Num + NumCast + Copy,
        Umin: 'a + Num + NumCast + Copy,
        Umax: 'a + Num + NumCast + Copy,
    {
        let xs = vector_to_f64(x);
        let mut ymins = vector_to_f64(ymin);
        let mut ymaxs = vector_to_f64(ymax);
        if ymins.len() == 1 && xs.len() > 1 {
            ymins = vec![ymins[0]; xs.len()];
        }
        if ymaxs.len() == 1 && xs.len() > 1 {
            ymaxs = vec![ymaxs[0]; xs.len()];
        }
        Series::VLines {
            x: xs,
            ymin: ymins,
            ymax: ymaxs,
            style,
        }
    }

    pub fn eventplot<'a, T, U>(positions: &'a T, style: EventplotStyle) -> Self
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        Series::EventPlot {
            positions: vector_to_f64(positions),
            style,
        }
    }
}
