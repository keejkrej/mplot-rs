use std::path::Path;

use crate::compile;
use crate::constants::{
    DEFAULT_DPI, DEFAULT_FIGURE_HEIGHT_IN, DEFAULT_FIGURE_WIDTH_IN, DEFAULT_FONT_SIZE,
    DEFAULT_HORIZONTAL_GAP, DEFAULT_TICK_FONT_SIZE, DEFAULT_VERTICAL_GAP,
};
use crate::error::{Error, Result};
use crate::panel::{configure_panel, AxesStyle, GridPos, PanelBuilder, PanelSpec};
use crate::render::png;

/// Figure dimensions.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    width_in: f64,
    height_in: f64,
}

impl Size {
    pub fn inches(width: f64, height: f64) -> Self {
        Size {
            width_in: width,
            height_in: height,
        }
    }

    pub fn width_in(&self) -> f64 {
        self.width_in
    }

    pub fn height_in(&self) -> f64 {
        self.height_in
    }
}

impl Default for Size {
    fn default() -> Self {
        Size::inches(DEFAULT_FIGURE_WIDTH_IN, DEFAULT_FIGURE_HEIGHT_IN)
    }
}

/// Options passed to [`Figure::save`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SaveOptions {
    dpi: u32,
    tight: bool,
    pad_inches: Option<f64>,
}

impl Default for SaveOptions {
    fn default() -> Self {
        SaveOptions {
            dpi: DEFAULT_DPI,
            tight: true,
            pad_inches: None,
        }
    }
}

impl SaveOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn dpi(mut self, dpi: u32) -> Self {
        self.dpi = dpi;
        self
    }

    pub fn tight(mut self, tight: bool) -> Self {
        self.tight = tight;
        self
    }

    pub fn pad_inches(mut self, pad: f64) -> Self {
        self.pad_inches = Some(pad);
        self
    }

    pub(crate) fn dpi_value(&self) -> u32 {
        self.dpi
    }

    pub(crate) fn tight_value(&self) -> bool {
        self.tight
    }

    pub(crate) fn pad_inches_value(&self) -> Option<f64> {
        self.pad_inches
    }
}

/// A compiled figure ready for export.
#[derive(Clone, Debug, PartialEq)]
pub struct Figure {
    size: Size,
    h_gap: f64,
    v_gap: f64,
    label_fontsize: f64,
    tick_fontsize: f64,
    title_fontsize: f64,
    panels: Vec<PanelSpec>,
}

impl Figure {
    pub fn builder() -> FigureBuilder {
        FigureBuilder::new()
    }

    pub fn save<P: AsRef<Path>>(&self, path: P, options: SaveOptions) -> Result<()> {
        if self.panels.is_empty() {
            return Err(Error::EmptyFigure);
        }
        let compiled = compile::build(self, &options)?;
        png::render(&compiled, path.as_ref()).map_err(Error::RenderFailed)
    }

    pub(crate) fn size(&self) -> Size {
        self.size
    }

    pub(crate) fn h_gap(&self) -> f64 {
        self.h_gap
    }

    pub(crate) fn v_gap(&self) -> f64 {
        self.v_gap
    }

    pub(crate) fn label_fontsize(&self) -> f64 {
        self.label_fontsize
    }

    pub(crate) fn tick_fontsize(&self) -> f64 {
        self.tick_fontsize
    }

    pub(crate) fn title_fontsize(&self) -> f64 {
        self.title_fontsize
    }

    pub(crate) fn panels(&self) -> &[PanelSpec] {
        &self.panels
    }
}

/// Builds a [`Figure`] panel by panel.
#[derive(Clone, Debug, Default)]
pub struct FigureBuilder {
    size: Size,
    h_gap: f64,
    v_gap: f64,
    label_fontsize: f64,
    tick_fontsize: f64,
    title_fontsize: f64,
    panels: Vec<PanelSpec>,
}

impl FigureBuilder {
    pub fn new() -> Self {
        FigureBuilder {
            size: Size::default(),
            h_gap: DEFAULT_HORIZONTAL_GAP,
            v_gap: DEFAULT_VERTICAL_GAP,
            label_fontsize: DEFAULT_FONT_SIZE,
            tick_fontsize: DEFAULT_TICK_FONT_SIZE,
            title_fontsize: DEFAULT_FONT_SIZE,
            panels: Vec::new(),
        }
    }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn gaps(mut self, horizontal: f64, vertical: f64) -> Self {
        self.h_gap = horizontal;
        self.v_gap = vertical;
        self
    }

    pub fn label_fontsize(mut self, size: f64) -> Self {
        self.label_fontsize = size;
        self.tick_fontsize = size;
        self
    }

    pub fn tick_fontsize(mut self, size: f64) -> Self {
        self.tick_fontsize = size;
        self
    }

    pub fn title_fontsize(mut self, size: f64) -> Self {
        self.title_fontsize = size;
        self
    }

    pub fn panel<F>(mut self, pos: GridPos, f: F) -> Self
    where
        F: FnOnce(&mut PanelBuilder<'_>),
    {
        let mut spec = PanelSpec {
            pos,
            axes: AxesStyle::default(),
            series: Vec::new(),
        };
        configure_panel(&mut spec, f);
        self.panels.push(spec);
        self
    }

    pub fn build(self) -> Result<Figure> {
        if self.panels.is_empty() {
            return Err(Error::EmptyFigure);
        }
        Ok(Figure {
            size: self.size,
            h_gap: self.h_gap,
            v_gap: self.v_gap,
            label_fontsize: self.label_fontsize,
            tick_fontsize: self.tick_fontsize,
            title_fontsize: self.title_fontsize,
            panels: self.panels,
        })
    }
}