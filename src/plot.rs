use std::ffi::OsStr;
use std::path::Path;

use crate::constants::{
    DEFAULT_DPI, DEFAULT_FIGURE_HEIGHT_IN, DEFAULT_FIGURE_WIDTH_IN, DEFAULT_FONT_SIZE,
    DEFAULT_HORIZONTAL_GAP, DEFAULT_TICK_FONT_SIZE, DEFAULT_VERTICAL_GAP,
};
use crate::graph::{GraphEntity, GraphMaker};
use crate::render::png;
use crate::StrError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SubplotKey {
    rows: usize,
    cols: usize,
    index: usize,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct TickLabels {
    ticks: Vec<f64>,
    labels: Vec<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct Panel {
    key: SubplotKey,
    entities: Vec<GraphEntity>,
    title: Option<String>,
    xlabel: Option<String>,
    ylabel: Option<String>,
    xrange: Option<(f64, f64)>,
    yrange: Option<(f64, f64)>,
    log_x: bool,
    log_y: bool,
    hide_axes: bool,
    show_grid: bool,
    ticks_x: Option<TickLabels>,
    ticks_y: Option<TickLabels>,
}

impl Panel {
    fn new(key: SubplotKey) -> Self {
        Panel {
            key,
            entities: Vec::new(),
            title: None,
            xlabel: None,
            ylabel: None,
            xrange: None,
            yrange: None,
            log_x: false,
            log_y: false,
            hide_axes: false,
            show_grid: false,
            ticks_x: None,
            ticks_y: None,
        }
    }
}

/// Driver structure that renders figures natively in Rust.
pub struct Plot {
    panels: Vec<Panel>,
    current: Option<SubplotKey>,
    figure_width_inches: f64,
    figure_height_inches: f64,
    dpi: u32,
    h_gap: f64,
    v_gap: f64,
    save_tight: bool,
    save_pad_inches: Option<f64>,
    label_fontsize: f64,
    tick_fontsize: f64,
    title_fontsize: f64,
    show_errors: bool,
}

impl Default for Plot {
    fn default() -> Self {
        Self::new()
    }
}

impl Plot {
    pub fn new() -> Self {
        Plot {
            panels: Vec::new(),
            current: None,
            figure_width_inches: DEFAULT_FIGURE_WIDTH_IN,
            figure_height_inches: DEFAULT_FIGURE_HEIGHT_IN,
            dpi: DEFAULT_DPI,
            h_gap: DEFAULT_HORIZONTAL_GAP,
            v_gap: DEFAULT_VERTICAL_GAP,
            save_tight: true,
            save_pad_inches: None,
            label_fontsize: DEFAULT_FONT_SIZE,
            tick_fontsize: DEFAULT_TICK_FONT_SIZE,
            title_fontsize: DEFAULT_FONT_SIZE,
            show_errors: false,
        }
    }

    fn current_panel(&mut self) -> Result<&mut Panel, StrError> {
        let key = self.current.clone().ok_or("call set_subplot before configuring axes")?;
        if !self.panels.iter().any(|panel| panel.key == key) {
            self.panels.push(Panel::new(key.clone()));
        }
        self.panels
            .iter_mut()
            .find(|panel| panel.key == key)
            .ok_or("internal subplot error")
    }

    fn ensure_panel(&mut self, key: SubplotKey) {
        if !self.panels.iter().any(|panel| panel.key == key) {
            self.panels.push(Panel::new(key));
        }
    }

    pub fn add(&mut self, graph: &dyn GraphMaker) -> &mut Self {
        if self.current.is_none() {
            self.set_subplot(1, 1, 1);
        }
        if let Some(entity) = graph.graph_entity() {
            if let Ok(panel) = self.current_panel() {
                panel.entities.push(entity);
            }
        }
        self
    }

    pub fn set_save_tight(&mut self, tight: bool) -> &mut Self {
        self.save_tight = tight;
        self
    }

    pub fn set_save_pad_inches(&mut self, pad_inches: f64) -> &mut Self {
        self.save_pad_inches = Some(pad_inches);
        self
    }

    pub fn save<S>(&self, figure_path: &S) -> Result<(), StrError>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let path = Path::new(figure_path);
        png::render_plot(self, path)
    }

    pub fn set_show_errors(&mut self, option: bool) -> &mut Self {
        self.show_errors = option;
        self
    }

    pub fn set_subplot(&mut self, row: usize, col: usize, index: usize) -> &mut Self {
        let key = SubplotKey { rows: row, cols: col, index };
        self.ensure_panel(key.clone());
        self.current = Some(key);
        self
    }

    pub fn grid_and_labels(&mut self, xlabel: &str, ylabel: &str) -> &mut Self {
        if let Ok(panel) = self.current_panel() {
            panel.show_grid = true;
            panel.xlabel = Some(xlabel.to_string());
            panel.ylabel = Some(ylabel.to_string());
        }
        self
    }

    pub fn grid_labels_legend(&mut self, xlabel: &str, ylabel: &str) -> &mut Self {
        self.grid_and_labels(xlabel, ylabel)
    }

    pub fn set_title(&mut self, title: &str) -> &mut Self {
        if let Ok(panel) = self.current_panel() {
            panel.title = Some(title.to_string());
        }
        self
    }

    pub fn set_horizontal_gap(&mut self, value: f64) -> &mut Self {
        self.h_gap = value;
        self
    }

    pub fn set_vertical_gap(&mut self, value: f64) -> &mut Self {
        self.v_gap = value;
        self
    }

    pub fn set_gaps(&mut self, horizontal: f64, vertical: f64) -> &mut Self {
        self.h_gap = horizontal;
        self.v_gap = vertical;
        self
    }

    pub fn set_figure_size_inches(&mut self, width: f64, height: f64) -> &mut Self {
        self.figure_width_inches = width;
        self.figure_height_inches = height;
        self
    }

    pub fn set_figure_size_points(&mut self, width: f64, height: f64) -> &mut Self {
        self.figure_width_inches = width / 72.0;
        self.figure_height_inches = height / 72.0;
        self
    }

    pub fn set_hide_axes(&mut self, hide: bool) -> &mut Self {
        if let Ok(panel) = self.current_panel() {
            panel.hide_axes = hide;
        }
        self
    }

    pub fn set_range(&mut self, xmin: f64, xmax: f64, ymin: f64, ymax: f64) -> &mut Self {
        if let Ok(panel) = self.current_panel() {
            panel.xrange = Some((xmin, xmax));
            panel.yrange = Some((ymin, ymax));
        }
        self
    }

    pub fn set_xrange(&mut self, xmin: f64, xmax: f64) -> &mut Self {
        if let Ok(panel) = self.current_panel() {
            panel.xrange = Some((xmin, xmax));
        }
        self
    }

    pub fn set_yrange(&mut self, ymin: f64, ymax: f64) -> &mut Self {
        if let Ok(panel) = self.current_panel() {
            panel.yrange = Some((ymin, ymax));
        }
        self
    }

    pub fn set_log_x(&mut self, log: bool) -> &mut Self {
        if let Ok(panel) = self.current_panel() {
            panel.log_x = log;
        }
        self
    }

    pub fn set_log_y(&mut self, log: bool) -> &mut Self {
        if let Ok(panel) = self.current_panel() {
            panel.log_y = log;
        }
        self
    }

    pub fn set_label_x(&mut self, label: &str) -> &mut Self {
        if let Ok(panel) = self.current_panel() {
            panel.xlabel = Some(label.to_string());
        }
        self
    }

    pub fn set_label_y(&mut self, label: &str) -> &mut Self {
        if let Ok(panel) = self.current_panel() {
            panel.ylabel = Some(label.to_string());
        }
        self
    }

    pub fn set_labels(&mut self, xlabel: &str, ylabel: &str) -> &mut Self {
        self.set_label_x(xlabel).set_label_y(ylabel)
    }

    pub fn set_label_x_fontsize(&mut self, fontsize: f64) -> &mut Self {
        self.label_fontsize = fontsize;
        self
    }

    pub fn set_label_y_fontsize(&mut self, fontsize: f64) -> &mut Self {
        self.label_fontsize = fontsize;
        self
    }

    pub fn set_ticks_x_fontsize(&mut self, fontsize: f64) -> &mut Self {
        self.tick_fontsize = fontsize;
        self
    }

    pub fn set_ticks_y_fontsize(&mut self, fontsize: f64) -> &mut Self {
        self.tick_fontsize = fontsize;
        self
    }

    pub fn set_ticks_x_labels<'a, S, T, U>(&mut self, ticks: &'a T, labels: &[S]) -> &mut Self
    where
        S: AsRef<str>,
        T: crate::as_vector::AsVector<'a, U>,
        U: 'a + num_traits::Num + num_traits::NumCast + Copy,
    {
        if labels.len() != ticks.vec_size() {
            return self;
        }
        let tick_values = crate::as_vector::vector_to_f64(ticks);
        let tick_labels = labels.iter().map(|label| label.as_ref().to_string()).collect();
        if let Ok(panel) = self.current_panel() {
            panel.ticks_x = Some(TickLabels {
                ticks: tick_values,
                labels: tick_labels,
            });
        }
        self
    }

    pub fn set_ticks_y_labels<'a, S, T, U>(&mut self, ticks: &'a T, labels: &[S]) -> &mut Self
    where
        S: AsRef<str>,
        T: crate::as_vector::AsVector<'a, U>,
        U: 'a + num_traits::Num + num_traits::NumCast + Copy,
    {
        if labels.len() != ticks.vec_size() {
            return self;
        }
        let tick_values = crate::as_vector::vector_to_f64(ticks);
        let tick_labels = labels.iter().map(|label| label.as_ref().to_string()).collect();
        if let Ok(panel) = self.current_panel() {
            panel.ticks_y = Some(TickLabels {
                ticks: tick_values,
                labels: tick_labels,
            });
        }
        self
    }

    pub(crate) fn panels(&self) -> &[Panel] {
        &self.panels
    }

    pub(crate) fn figure_width_inches(&self) -> f64 {
        self.figure_width_inches
    }

    pub(crate) fn figure_height_inches(&self) -> f64 {
        self.figure_height_inches
    }

    pub(crate) fn dpi(&self) -> u32 {
        self.dpi
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

    pub(crate) fn save_tight(&self) -> bool {
        self.save_tight
    }

    pub(crate) fn save_pad_inches(&self) -> Option<f64> {
        self.save_pad_inches
    }
}

impl Panel {
    pub(crate) fn key(&self) -> &SubplotKey {
        &self.key
    }

    pub(crate) fn entities(&self) -> &[GraphEntity] {
        &self.entities
    }

    pub(crate) fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub(crate) fn xlabel(&self) -> Option<&str> {
        self.xlabel.as_deref()
    }

    pub(crate) fn ylabel(&self) -> Option<&str> {
        self.ylabel.as_deref()
    }

    pub(crate) fn xrange(&self) -> Option<(f64, f64)> {
        self.xrange
    }

    pub(crate) fn yrange(&self) -> Option<(f64, f64)> {
        self.yrange
    }

    pub(crate) fn log_x(&self) -> bool {
        self.log_x
    }

    pub(crate) fn log_y(&self) -> bool {
        self.log_y
    }

    pub(crate) fn hide_axes(&self) -> bool {
        self.hide_axes
    }

    pub(crate) fn show_grid(&self) -> bool {
        self.show_grid
    }

    pub(crate) fn ticks_x(&self) -> Option<&TickLabels> {
        self.ticks_x.as_ref()
    }

    pub(crate) fn ticks_y(&self) -> Option<&TickLabels> {
        self.ticks_y.as_ref()
    }
}

impl SubplotKey {
    pub(crate) fn rows(&self) -> usize {
        self.rows
    }

    pub(crate) fn cols(&self) -> usize {
        self.cols
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}

impl TickLabels {
    pub(crate) fn ticks(&self) -> &[f64] {
        &self.ticks
    }

    pub(crate) fn labels(&self) -> &[String] {
        &self.labels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Curve;

    #[test]
    fn subplot_adds_curve() {
        let mut curve = Curve::new();
        let x = &[1.0, 2.0];
        let y = &[3.0, 4.0];
        curve.draw(x, y);

        let mut plot = Plot::new();
        plot.set_subplot(1, 1, 1).add(&curve);

        assert_eq!(plot.panels().len(), 1);
        assert_eq!(plot.panels()[0].entities().len(), 1);
    }
}
