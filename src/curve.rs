use num_traits::{Num, NumCast};

use crate::as_vector::{vector_to_f64, AsVector};
use crate::color;
use crate::graph::{GraphEntity, GraphMaker};
use plotters::style::RGBColor;

#[derive(Clone, Debug, Default)]
pub struct CurveData {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub label: String,
    pub line_alpha: f64,
    pub line_color: String,
    pub line_style: String,
    pub line_width: f64,
}

impl CurveData {
    pub fn rgb_color(&self) -> RGBColor {
        let base = color::parse_color(&self.line_color);
        if self.line_alpha > 0.0 && self.line_alpha < 1.0 {
            color::with_alpha(base, self.line_alpha)
        } else {
            base
        }
    }
}

pub struct Curve {
    label: String,
    line_alpha: f64,
    line_color: String,
    line_style: String,
    line_width: f64,
    drawn: Option<CurveData>,
    buffer: String,
}

impl Default for Curve {
    fn default() -> Self {
        Self::new()
    }
}

impl Curve {
    pub fn new() -> Self {
        Curve {
            label: String::new(),
            line_alpha: 1.0,
            line_color: String::new(),
            line_style: String::new(),
            line_width: 1.0,
            drawn: None,
            buffer: String::new(),
        }
    }

    pub fn draw<'a, T, U>(&mut self, x: &'a T, y: &'a T)
    where
        T: AsVector<'a, U>,
        U: 'a + Num + NumCast + Copy,
    {
        let xs = vector_to_f64(x);
        let ys = vector_to_f64(y);
        self.drawn = Some(CurveData {
            x: xs,
            y: ys,
            label: self.label.clone(),
            line_alpha: self.line_alpha,
            line_color: self.line_color.clone(),
            line_style: self.line_style.clone(),
            line_width: self.line_width,
        });
    }

    pub fn set_label(&mut self, label: &str) -> &mut Self {
        self.label = label.to_string();
        if let Some(drawn) = &mut self.drawn {
            drawn.label = self.label.clone();
        }
        self
    }

    pub fn set_line_alpha(&mut self, alpha: f64) -> &mut Self {
        self.line_alpha = if alpha < 1e-14 { 1.0 } else { alpha };
        if let Some(drawn) = &mut self.drawn {
            drawn.line_alpha = self.line_alpha;
        }
        self
    }

    pub fn set_line_color(&mut self, color: &str) -> &mut Self {
        self.line_color = color.to_string();
        if let Some(drawn) = &mut self.drawn {
            drawn.line_color = self.line_color.clone();
        }
        self
    }

    pub fn set_line_style(&mut self, style: &str) -> &mut Self {
        self.line_style = style.to_string();
        if let Some(drawn) = &mut self.drawn {
            drawn.line_style = self.line_style.clone();
        }
        self
    }

    pub fn set_line_width(&mut self, width: f64) -> &mut Self {
        self.line_width = width;
        if let Some(drawn) = &mut self.drawn {
            drawn.line_width = self.line_width;
        }
        self
    }
}

impl GraphMaker for Curve {
    fn get_buffer(&self) -> &String {
        &self.buffer
    }

    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    fn graph_entity(&self) -> Option<GraphEntity> {
        self.drawn.clone().map(GraphEntity::Curve)
    }
}
