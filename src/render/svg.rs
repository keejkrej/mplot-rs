use std::path::Path;

use plotters::prelude::*;

use crate::render::canvas::{figure_dimensions, render_figure};
use crate::render::model::CompiledFigure;

pub fn render_to_string(figure: &CompiledFigure) -> Result<String, &'static str> {
    let (width_px, height_px, _) = figure_dimensions(figure)?;
    let mut svg = String::new();
    {
        let root = SVGBackend::with_string(&mut svg, (width_px, height_px)).into_drawing_area();
        root.fill(&RGBColor(255, 255, 255))
            .map_err(|_| "failed to initialize canvas")?;
        render_figure(figure, &root)?;
        root.present().map_err(|_| "failed to finalize figure")?;
    }
    Ok(svg)
}

pub fn render(figure: &CompiledFigure, path: &Path) -> Result<(), &'static str> {
    let svg = render_to_string(figure)?;
    std::fs::write(path, svg).map_err(|_| "failed to write figure")?;
    Ok(())
}
