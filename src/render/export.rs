use std::path::Path;

use crate::error::Error;
use crate::figure::ExportFormat;
use crate::render::model::CompiledFigure;

pub fn render(
    figure: &CompiledFigure,
    path: &Path,
    format: ExportFormat,
) -> Result<(), Error> {
    match format {
        ExportFormat::Png => crate::render::png::render(figure, path).map_err(Error::RenderFailed),
        ExportFormat::Svg => crate::render::svg::render(figure, path).map_err(Error::RenderFailed),
        ExportFormat::Pdf => Err(Error::UnsupportedFormat),
    }
}
