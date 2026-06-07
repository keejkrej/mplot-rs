use crate::as_vector::{vector_to_f64, AsVector};
use crate::color::Color;
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

/// Marker style for line series (reserved for future use).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Marker {
    #[default]
    None,
}

/// Styling for line series.
#[derive(Clone, Debug, PartialEq)]
pub struct LineStyle {
    color: Color,
    width: f64,
    dash: LineDash,
    label: Option<String>,
    alpha: f64,
}

impl Default for LineStyle {
    fn default() -> Self {
        LineStyle {
            color: Color::TABLEAU[0],
            width: DEFAULT_LINE_WIDTH,
            dash: LineDash::Solid,
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
}
