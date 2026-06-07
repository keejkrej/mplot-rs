use crate::as_vector::{vector_to_f64, AsVector};
use num_traits::{Num, NumCast};

/// Subplot address in a grid (1-based index, matplotlib convention).
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
            labels: labels.iter().map(|label| label.as_ref().to_string()).collect(),
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

use crate::series::Scale;

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
    grid: bool,
    hide: bool,
    x_ticks: Option<TickLabels>,
    y_ticks: Option<TickLabels>,
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
        self.x_range = Some((min, max));
        self
    }

    pub fn y_range(mut self, min: f64, max: f64) -> Self {
        self.y_range = Some((min, max));
        self
    }

    pub fn grid(mut self, show: bool) -> Self {
        self.grid = show;
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
                labels: labels.iter().map(|label| label.as_ref().to_string()).collect(),
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
                labels: labels.iter().map(|label| label.as_ref().to_string()).collect(),
            });
        }
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

    pub(crate) fn grid_value(&self) -> bool {
        self.grid
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

    pub(crate) fn legend_value(&self) -> LegendStyle {
        self.legend
    }
}

/// One subplot's content before compilation.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PanelSpec {
    pub pos: GridPos,
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
    pub fn line<'b, T, U>(&mut self, x: &'b T, y: &'b T, style: crate::series::LineStyle) -> &mut Self
    where
        T: AsVector<'b, U>,
        U: 'b + Num + NumCast + Copy,
    {
        self.spec
            .series
            .push(crate::series::Series::line(x, y, style));
        self
    }

    pub fn boxplot(&mut self, groups: &[Vec<f64>], style: crate::series::BoxplotStyle) -> &mut Self {
        self.spec
            .series
            .push(crate::series::Series::boxplot(groups, style));
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
