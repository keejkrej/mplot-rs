use std::path::Path;

use svg2pdf::{ConversionOptions, PageOptions};

use crate::render::model::CompiledFigure;
use crate::render::svg;

pub fn render(figure: &CompiledFigure, path: &Path) -> Result<(), &'static str> {
    let svg = svg::render_to_string(figure)?;

    let mut options = svg2pdf::usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    let tree = svg2pdf::usvg::Tree::from_str(&svg, &options)
        .map_err(|_| "failed to parse svg for pdf")?;

    let page_options = PageOptions {
        dpi: figure.dpi as f32,
    };
    let pdf = svg2pdf::to_pdf(&tree, ConversionOptions::default(), page_options)
        .map_err(|_| "failed to convert svg to pdf")?;

    std::fs::write(path, pdf).map_err(|_| "failed to write figure")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::figure::{Figure, SaveOptions};
    use crate::panel::GridPos;
    use crate::prelude::{LineStyle, Result};

    #[test]
    fn pdf_export_produces_bytes() -> Result<()> {
        let figure = Figure::builder()
            .panel(GridPos::new(1, 1, 1), |p| {
                p.line(
                    &[0.0, 1.0, 2.0],
                    &[0.0, 1.0, 4.0],
                    LineStyle::new(),
                );
            })
            .build()?;
        let compiled = crate::compile::build(&figure, &SaveOptions::default())?;
        let path = std::env::temp_dir().join("mplot_pdf_export_test.pdf");
        render(&compiled, &path).map_err(|msg| crate::error::Error::RenderFailed(msg))?;
        let bytes = std::fs::read(&path).map_err(|err| crate::error::Error::Io(err.to_string()))?;
        assert!(bytes.starts_with(b"%PDF"));
        assert!(bytes.len() > 500);
        Ok(())
    }
}
