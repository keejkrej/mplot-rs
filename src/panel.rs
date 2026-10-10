use crate::as_vector::{vector_to_f64, AsVector};
use crate::gridspec::SubplotSlot;
use num_traits::{Num, NumCast};

/// Subplot address in a grid (1-based index, matplotlib convention).
///
/// Prefer [`GridSpec`](crate::gridspec::GridSpec) for irregular layouts with rowspan/colspan.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GridPos {
    rows: usize,
    cols: usize,
    index: usize,
}

impl GridPos {
    pub fn new(rows: usize, cols: usize, index: usize) -> Self {
        GridPos { rows, cols, index }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn to_slot(&self) -> SubplotSlot {
        SubplotSlot::from_index(self.rows, self.cols, self.index)
    }
}

impl From<GridPos> for SubplotSlot {
    fn from(pos: GridPos) -> Self {
        pos.to_slot()
    }
}

/// Custom tick positions and labels.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TickLabels {
    ticks: Vec<f64>,
    labels: Vec<String>,
}

impl TickLabels {
    pub fn new(ticks: &[f64], labels: &[impl AsRef<str>]) -> Self {
        TickLabels {
            ticks: ticks.to_vec(),
            labels: labels
                .iter()
                .map(|label| label.as_ref().to_string())
                .collect(),
        }
    }

    pub fn ticks(&self) -> &[f64] {
        &self.ticks
    }

    pub fn labels(&self) -> &[String] {
        &self.labels
    }
}

/// Legend display options.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LegendStyle {
    pub show: bool,
}

impl LegendStyle {
    pub fn show() -> Self {
        LegendStyle { show: true }
    }
}

use crate::color::Color;
use crate::series::Scale;

/// Grid line styling (matplotlib `ax.grid(...)`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridStyle {
    pub show: bool,
    pub color: Option<Color>,
    pub width: f64,
    pub alpha: f64,
}

impl Default for GridStyle {
    fn default() -> Self {
        GridStyle {
            show: true,
            color: None,
            width: 0.8,
            alpha: 1.0,
        }
    }
}

impl GridStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(mut self, show: bool) -> Self {
        self.show = show;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn width(mut self, width: f64) -> Self {
        self.width = width;
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }
}

/// How automatic tick labels are formatted (custom tick labels override this).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TickFormat {
    /// Compact plain numbers (`format_linear`).
    #[default]
    Auto,
    /// Scientific notation (`1.5e6`).
    Scientific,
}

/// Rotation for category / tick labels.
///
/// Plotters' built-in [`FontTransform`] only covers 90° steps. Cardinal
/// variants (`Ccw90` / `Cw90`) use that path; [`TickLabelRotation::Degrees`]
/// is drawn by mplot with an arbitrary rotation matrix (matplotlib-style
/// counter-clockwise degrees from horizontal).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum TickLabelRotation {
    #[default]
    None,
    /// Counter-clockwise 90° (bottom labels read upward — good for long category names).
    Ccw90,
    /// Clockwise 90°.
    Cw90,
    /// Counter-clockwise degrees from horizontal (matplotlib `rotation=`).
    ///
    /// Typical category axes use a mild tilt such as `Degrees(-30)` (matches
    /// Observable Plot / D3 `tickRotate: -30`) so labels hang below the axis.
    Degrees(i16),
}

impl TickLabelRotation {
    /// Signed CCW degrees from horizontal, or `None` when labels stay upright.
    pub fn degrees(self) -> Option<i16> {
        match self {
            TickLabelRotation::None => None,
            TickLabelRotation::Ccw90 => Some(90),
            TickLabelRotation::Cw90 => Some(-90),
            TickLabelRotation::Degrees(deg) => Some(deg),
        }
    }

    /// True when labels must be drawn outside plotters' 90° FontTransform path.
    pub(crate) fn needs_custom_draw(self) -> bool {
        match self {
            TickLabelRotation::None | TickLabelRotation::Ccw90 | TickLabelRotation::Cw90 => false,
            TickLabelRotation::Degrees(deg) => {
                let d = deg.rem_euclid(360);
                d != 0 && d != 90 && d != 180 && d != 270
            }
        }
    }
}

/// Axes styling and limits for one panel.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AxesStyle {
    title: Option<String>,
    x_label: Option<String>,
    y_label: Option<String>,
    x_scale: Scale,
    y_scale: Scale,
    x_range: Option<(f64, f64)>,
    y_range: Option<(f64, f64)>,
    invert_x: bool,
    invert_y: bool,
    grid: Option<GridStyle>,
    hide: bool,
    x_ticks: Option<TickLabels>,
    y_ticks: Option<TickLabels>,
    x_tick_format: TickFormat,
    y_tick_format: TickFormat,
    x_tick_label_rotation: TickLabelRotation,
    legend: LegendStyle,
}

impl AxesStyle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn x_label(mut self, label: impl Into<String>) -> Self {
        self.x_label = Some(label.into());
        self
    }

    pub fn y_label(mut self, label: impl Into<String>) -> Self {
        self.y_label = Some(label.into());
        self
    }

    pub fn x_scale(mut self, scale: Scale) -> Self {
        self.x_scale = scale;
        self
    }

    pub fn y_scale(mut self, scale: Scale) -> Self {
        self.y_scale = scale;
        self
    }

    pub fn x_range(mut self, min: f64, max: f64) -> Self {
        if min > max {
            self.invert_x = true;
            self.x_range = Some((max, min));
        } else {
            self.x_range = Some((min, max));
        }
        self
    }

    pub fn y_range(mut self, min: f64, max: f64) -> Self {
        if min > max {
            self.invert_y = true;
            self.y_range = Some((max, min));
        } else {
            self.y_range = Some((min, max));
        }
        self
    }

    pub fn invert_x(mut self, invert: bool) -> Self {
        self.invert_x = invert;
        self
    }

    pub fn invert_y(mut self, invert: bool) -> Self {
        self.invert_y = invert;
        self
    }

    pub fn grid(mut self, show: bool) -> Self {
        let mut style = self.grid.unwrap_or_default();
        style.show = show;
        self.grid = Some(style);
        self
    }

    pub fn grid_style(mut self, style: GridStyle) -> Self {
        self.grid = Some(style);
        self
    }

    pub fn hide(mut self, hide: bool) -> Self {
        self.hide = hide;
        self
    }

    pub fn x_tick_labels<'a, S, T, U>(mut self, ticks: &'a T, labels: &[S]) -> Self
    where
        S: AsRef<str>,
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        if labels.len() == ticks.vec_size() {
            self.x_ticks = Some(TickLabels {
                ticks: vector_to_f64(ticks),
                labels: labels
                    .iter()
                    .map(|label| label.as_ref().to_string())
                    .collect(),
            });
        }
        self
    }

    pub fn y_tick_labels<'a, S, T, U>(mut self, ticks: &'a T, labels: &[S]) -> Self
    where
        S: AsRef<str>,
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        if labels.len() == ticks.vec_size() {
            self.y_ticks = Some(TickLabels {
                ticks: vector_to_f64(ticks),
                labels: labels
                    .iter()
                    .map(|label| label.as_ref().to_string())
                    .collect(),
            });
        }
        self
    }

    pub fn x_tick_format(mut self, format: TickFormat) -> Self {
        self.x_tick_format = format;
        self
    }

    pub fn y_tick_format(mut self, format: TickFormat) -> Self {
        self.y_tick_format = format;
        self
    }

    /// Rotate x-axis tick labels (category names on boxplots, etc.).
    pub fn x_tick_label_rotation(mut self, rotation: TickLabelRotation) -> Self {
        self.x_tick_label_rotation = rotation;
        self
    }

    pub fn legend(mut self, legend: LegendStyle) -> Self {
        self.legend = legend;
        self
    }

    pub(crate) fn title_value(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub(crate) fn x_label_value(&self) -> Option<&str> {
        self.x_label.as_deref()
    }

    pub(crate) fn y_label_value(&self) -> Option<&str> {
        self.y_label.as_deref()
    }

    pub(crate) fn x_scale_value(&self) -> Scale {
        self.x_scale
    }

    pub(crate) fn y_scale_value(&self) -> Scale {
        self.y_scale
    }

    pub(crate) fn x_range_value(&self) -> Option<(f64, f64)> {
        self.x_range
    }

    pub(crate) fn y_range_value(&self) -> Option<(f64, f64)> {
        self.y_range
    }

    pub(crate) fn invert_x_value(&self) -> bool {
        self.invert_x
    }

    pub(crate) fn invert_y_value(&self) -> bool {
        self.invert_y
    }

    pub(crate) fn grid_style_value(&self) -> Option<GridStyle> {
        self.grid
    }

    pub(crate) fn grid_value(&self) -> bool {
        self.grid.map(|g| g.show).unwrap_or(false)
    }

    pub(crate) fn hide_value(&self) -> bool {
        self.hide
    }

    pub(crate) fn x_ticks_value(&self) -> Option<&TickLabels> {
        self.x_ticks.as_ref()
    }

    pub(crate) fn y_ticks_value(&self) -> Option<&TickLabels> {
        self.y_ticks.as_ref()
    }

    pub(crate) fn x_tick_format_value(&self) -> TickFormat {
        self.x_tick_format
    }

    pub(crate) fn y_tick_format_value(&self) -> TickFormat {
        self.y_tick_format
    }

    pub(crate) fn x_tick_label_rotation_value(&self) -> TickLabelRotation {
        self.x_tick_label_rotation
    }

    pub(crate) fn legend_value(&self) -> LegendStyle {
        self.legend
    }
}

/// One subplot's content before compilation.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PanelSpec {
    pub slot: SubplotSlot,
    pub axes: AxesStyle,
    pub series: Vec<crate::series::Series>,
}

pub(crate) fn configure_panel(spec: &mut PanelSpec, f: impl FnOnce(&mut PanelBuilder<'_>)) {
    f(&mut PanelBuilder { spec });
}

/// Builder for a single panel inside [`FigureBuilder`](crate::figure::FigureBuilder).
pub struct PanelBuilder<'a> {
    spec: &'a mut PanelSpec,
}

impl<'a> PanelBuilder<'a> {
    pub fn line<'b, T, U>(
        &mut self,
        x: &'b T,
        y: &'b T,
        style: crate::series::LineStyle,
    ) -> &mut Self
    where
        T: AsVector<'b, U>,
        U: 'b + Num + NumCast + Copy,
    {
        self.spec
            .series
            .push(crate::series::Series::line(x, y, style));
        self
    }

    pub fn scatter<'b, T, U>(
        &mut self,
        x: &'b T,
        y: &'b T,
        style: crate::series::ScatterStyle,
    ) -> &mut Self
    where
        T: AsVector<'b, U>,
        U: 'b + Num + NumCast + Copy,
    {
        self.spec
            .series
            .push(crate::series::Series::scatter(x, y, style));
        self
    }

    pub fn errorbar<'b, T, U>(
        &mut self,
        x: &'b T,
        y: &'b T,
        style: crate::series::ErrorBarStyle,
    ) -> &mut Self
    where
        T: AsVector<'b, U>,
        U: 'b + Num + NumCast + Copy,
    {
        self.spec
            .series
            .push(crate::series::Series::errorbar(x, y, style));
        self
    }

    pub fn step<'b, T, U>(
        &mut self,
        x: &'b T,
        y: &'b T,
        style: crate::series::StepStyle,
    ) -> &mut Self
    where
        T: AsVector<'b, U>,
        U: 'b + Num + NumCast + Copy,
    {
        self.spec
            .series
            .push(crate::series::Series::step(x, y, style));
        self
    }

    pub fn boxplot(
        &mut self,
        groups: &[Vec<f64>],
        style: crate::series::BoxplotStyle,
    ) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::boxplot(groups, style));
        self
    }

    pub fn violinplot(
        &mut self,
        groups: &[Vec<f64>],
        style: crate::series::ViolinStyle,
    ) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::violin(groups.to_vec(), style));
        self
    }

    pub fn bar(&mut self, x: &[f64], heights: &[f64], style: crate::series::BarStyle) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::bar(x, heights, style));
        self
    }

    pub fn barh<'b, T, U>(
        &mut self,
        y: &'b T,
        widths: &'b T,
        style: crate::series::BarHStyle,
    ) -> &mut Self
    where
        T: AsVector<'b, U>,
        U: 'b + Num + NumCast + Copy,
    {
        self.spec
            .series
            .push(crate::series::Series::barh(y, widths, style));
        self
    }

    pub fn histogram(&mut self, data: &[f64], style: crate::series::HistStyle) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::histogram(data, style));
        self
    }

    pub fn fill_between<'b, T, U>(
        &mut self,
        x: &'b T,
        y1: &'b T,
        y2: &'b T,
        style: crate::series::FillBetweenStyle,
    ) -> &mut Self
    where
        T: AsVector<'b, U>,
        U: 'b + Num + NumCast + Copy,
    {
        self.spec
            .series
            .push(crate::series::Series::fill_between(x, y1, y2, style));
        self
    }

    pub fn image(
        &mut self,
        data: Vec<f64>,
        width: usize,
        height: usize,
        style: crate::series::ImageStyle,
    ) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::image(data, width, height, style));
        self
    }

    pub fn contour(
        &mut self,
        data: Vec<f64>,
        width: usize,
        height: usize,
        style: crate::series::ContourStyle,
    ) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::contour(data, width, height, style));
        self
    }

    pub fn text(
        &mut self,
        x: f64,
        y: f64,
        text: impl Into<String>,
        style: crate::series::TextStyle,
    ) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::text(x, y, text, style));
        self
    }

    pub fn annotate(
        &mut self,
        text: impl Into<String>,
        xy: (f64, f64),
        xytext: (f64, f64),
        style: crate::series::AnnotationStyle,
    ) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::annotate(text, xy, xytext, style));
        self
    }

    pub fn axhline(&mut self, y: f64, style: crate::series::LineStyle) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::axhline(y, style));
        self
    }

    pub fn axvline(&mut self, x: f64, style: crate::series::LineStyle) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::axvline(x, style));
        self
    }

    pub fn axhspan(
        &mut self,
        ymin: f64,
        ymax: f64,
        style: crate::series::FillBetweenStyle,
    ) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::axhspan(ymin, ymax, style));
        self
    }

    pub fn axvspan(
        &mut self,
        xmin: f64,
        xmax: f64,
        style: crate::series::FillBetweenStyle,
    ) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::axvspan(xmin, xmax, style));
        self
    }

    pub fn series(&mut self, series: crate::series::Series) -> &mut Self {
        self.spec.series.push(series);
        self
    }

    pub fn axes(&mut self, axes: AxesStyle) -> &mut Self {
        self.spec.axes = axes;
        self
    }
}
