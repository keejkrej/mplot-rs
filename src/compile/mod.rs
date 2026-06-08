use crate::figure::{Figure, SaveOptions};
use crate::error::Error;
use crate::panel::PanelSpec;
use crate::render::model::{CompiledFigure, CompiledPanel, CompiledSeries, LineSeries, BoxplotSeries};
use crate::series::{Scale, Series};

pub fn build(figure: &Figure, options: &SaveOptions) -> Result<CompiledFigure, Error> {
    let panels = figure
        .panels()
        .iter()
        .map(|panel| compile_panel(panel))
        .collect();

    Ok(CompiledFigure {
        width_in: figure.size().width_in(),
        height_in: figure.size().height_in(),
        dpi: options.dpi_value(),
        h_gap: figure.h_gap(),
        v_gap: figure.v_gap(),
        label_fontsize: figure.label_fontsize(),
        tick_fontsize: figure.tick_fontsize(),
        title_fontsize: figure.title_fontsize(),
        save_tight: options.tight_value(),
        save_pad_inches: options.pad_inches_value(),
        panels,
    })
}

fn compile_panel(panel: &PanelSpec) -> CompiledPanel {
    let axes = &panel.axes;
    CompiledPanel {
        rows: panel.pos.rows(),
        cols: panel.pos.cols(),
        index: panel.pos.index(),
        title: axes.title_value().map(str::to_string),
        xlabel: axes.x_label_value().map(str::to_string),
        ylabel: axes.y_label_value().map(str::to_string),
        xrange: axes.x_range_value(),
        yrange: axes.y_range_value(),
        log_x: axes.x_scale_value() == Scale::Log,
        log_y: axes.y_scale_value() == Scale::Log,
        hide_axes: axes.hide_value(),
        show_grid: axes.grid_value(),
        ticks_x: axes.x_ticks_value().cloned(),
        ticks_y: axes.y_ticks_value().cloned(),
        show_legend: axes.legend_value().show,
        series: panel.series.iter().map(compile_series).collect(),
    }
}

fn compile_series(series: &Series) -> CompiledSeries {
    match series {
        Series::Line { x, y, style } => {
            let color = if style.alpha_value() < 1.0 {
                style.color_value().with_alpha(style.alpha_value())
            } else {
                style.color_value()
            };
            CompiledSeries::Line(LineSeries {
                x: x.clone(),
                y: y.clone(),
                label: style.label_value().unwrap_or("").to_string(),
                color,
                dash: style.dash_value(),
                marker: style.marker_value(),
                width: style.width_value(),
            })
        }
        Series::Boxplot { groups, style } => CompiledSeries::Boxplot(BoxplotSeries {
            groups: groups.clone(),
            horizontal: style.horizontal_value(),
            whisker: style.whisker_value(),
            positions: style.positions_value().to_vec(),
            width: style.width_value(),
            no_fliers: style.no_fliers_value(),
            patch_artist: style.patch_artist_value(),
        }),
    }
}
