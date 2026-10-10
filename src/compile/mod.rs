use crate::figure::{Figure, SaveOptions};
use crate::error::Error;
use crate::panel::PanelSpec;
use crate::colormap::Normalize;
use crate::render::model::{
    AxHLineSeries, AxHSpanSeries, AxVLineSeries, AxVSpanSeries, BarHSeries, BarSeries,
    BoxplotSeries, CompiledFigure, CompiledPanel, CompiledSeries, ContourSeries, ErrorBarSeries,
    FillBetweenSeries, HistSeries, ImageSeries, LineSeries, PanelLayout, ScatterSeries, StepSeries,
    TextSeries,
};
use crate::series::{Scale, Series};

pub fn build(figure: &Figure, options: &SaveOptions) -> Result<CompiledFigure, Error> {
    let constrained = figure.constrained_layout();
    let panels = figure
        .panels()
        .iter()
        .map(|panel| compile_panel(panel, constrained))
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
        constrained_layout: constrained,
        panels,
    })
}

fn compile_panel(panel: &PanelSpec, constrained_layout: bool) -> CompiledPanel {
    let axes = &panel.axes;
    let slot = panel.slot;
    let has_colorbar = panel.series.iter().any(|series| match series {
        Series::Image { style, .. } => style.show_colorbar_value(),
        Series::Contour { style, .. } => style.show_colorbar_value(),
        _ => false,
    });
    let layout = if constrained_layout && has_colorbar {
        PanelLayout::with_colorbar_inset()
    } else {
        PanelLayout::default()
    };

    CompiledPanel {
        grid_rows: slot.grid_rows(),
        grid_cols: slot.grid_cols(),
        row: slot.row(),
        col: slot.col(),
        rowspan: slot.rowspan(),
        colspan: slot.colspan(),
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
        x_tick_format: axes.x_tick_format_value(),
        y_tick_format: axes.y_tick_format_value(),
        x_tick_label_rotation: axes.x_tick_label_rotation_value(),
        show_legend: axes.legend_value().show,
        layout,
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
        Series::Scatter { x, y, style } => {
            let color = if style.alpha_value() < 1.0 {
                style.color_value().with_alpha(style.alpha_value())
            } else {
                style.color_value()
            };
            CompiledSeries::Scatter(ScatterSeries {
                x: x.clone(),
                y: y.clone(),
                label: style.label_value().unwrap_or("").to_string(),
                color,
                size: style.size_value(),
                marker: style.marker_value(),
                alpha: style.alpha_value(),
                edge_color: style.edge_color_value(),
                edge_width: style.edge_width_value(),
            })
        }
        Series::ErrorBar { x, y, style } => {
            let color = if style.alpha_value() < 1.0 {
                style.color_value().with_alpha(style.alpha_value())
            } else {
                style.color_value()
            };
            let ecolor = style
                .ecolor_value()
                .map(|c| {
                    if style.alpha_value() < 1.0 {
                        c.with_alpha(style.alpha_value())
                    } else {
                        c
                    }
                })
                .unwrap_or(color);
            CompiledSeries::ErrorBar(ErrorBarSeries {
                x: x.clone(),
                y: y.clone(),
                yerr: style.yerr_value().map(|s| s.to_vec()),
                yerr_lower: style.yerr_lower_value().map(|s| s.to_vec()),
                xerr: style.xerr_value().map(|s| s.to_vec()),
                xerr_lower: style.xerr_lower_value().map(|s| s.to_vec()),
                label: style.label_value().unwrap_or("").to_string(),
                color,
                line_width: style.line_width_value(),
                dash: style.dash_value(),
                marker: style.marker_value(),
                capsize: style.capsize_value(),
                capthick: style.capthick_value(),
                elinewidth: style.elinewidth_value(),
                ecolor,
                alpha: style.alpha_value(),
            })
        }
        Series::Step { x, y, style } => {
            let color = if style.alpha_value() < 1.0 {
                style.color_value().with_alpha(style.alpha_value())
            } else {
                style.color_value()
            };
            CompiledSeries::Step(StepSeries {
                x: x.clone(),
                y: y.clone(),
                where_step: style.where_step_value(),
                label: style.label_value().unwrap_or("").to_string(),
                color,
                dash: style.dash_value(),
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
        Series::Bar { x, heights, style } => CompiledSeries::Bar(BarSeries {
            x: x.clone(),
            heights: heights.clone(),
            color: style.color_value(),
            width: style.width_value(),
            baseline: style.baseline_value(),
            label: style.label_value().unwrap_or("").to_string(),
        }),
        Series::BarH { y, widths, style } => CompiledSeries::BarH(BarHSeries {
            y: y.clone(),
            widths: widths.clone(),
            color: style.color_value(),
            height: style.height_value(),
            baseline: style.baseline_value(),
            label: style.label_value().unwrap_or("").to_string(),
        }),
        Series::Histogram { data, style } => CompiledSeries::Histogram(HistSeries {
            data: data.clone(),
            bins: style.bins_value(),
            color: style.color_value(),
            label: style.label_value().unwrap_or("").to_string(),
        }),
        Series::FillBetween { x, y1, y2, style } => CompiledSeries::FillBetween(FillBetweenSeries {
            x: x.clone(),
            y1: y1.clone(),
            y2: y2.clone(),
            color: style.color_value().with_alpha(style.alpha_value()),
            alpha: style.alpha_value(),
            label: style.label_value().unwrap_or("").to_string(),
        }),
        Series::Image {
            data,
            width,
            height,
            style,
        } => {
            let normalize = style
                .normalize_value()
                .unwrap_or_else(|| Normalize::of_slice(data));
            CompiledSeries::Image(ImageSeries {
                data: data.clone(),
                width: *width,
                height: *height,
                extent: style.extent_value(),
                colormap: style.colormap_value(),
                normalize,
                show_colorbar: style.show_colorbar_value(),
            })
        }
        Series::Contour {
            data,
            width,
            height,
            style,
        } => {
            let normalize = style
                .normalize_value()
                .unwrap_or_else(|| Normalize::of_slice(data));
            CompiledSeries::Contour(ContourSeries {
                data: data.clone(),
                width: *width,
                height: *height,
                extent: style.extent_value(),
                levels: style.levels_value().to_vec(),
                line_color: style.line_color_value(),
                colormap: style.colormap_value(),
                normalize,
                show_colorbar: style.show_colorbar_value(),
            })
        }
        Series::Text { x, y, text, style } => CompiledSeries::Text(TextSeries {
            x: *x,
            y: *y,
            text: text.clone(),
            color: style.color_value(),
            fontsize: style.fontsize_value(),
        }),
        Series::AxHLine { y, style } => CompiledSeries::AxHLine(AxHLineSeries {
            y: *y,
            color: style.color_value(),
            width: style.width_value(),
            dash: style.dash_value(),
            label: style.label_value().unwrap_or("").to_string(),
        }),
        Series::AxVLine { x, style } => CompiledSeries::AxVLine(AxVLineSeries {
            x: *x,
            color: style.color_value(),
            width: style.width_value(),
            dash: style.dash_value(),
            label: style.label_value().unwrap_or("").to_string(),
        }),
        Series::AxHSpan { ymin, ymax, style } => CompiledSeries::AxHSpan(AxHSpanSeries {
            ymin: *ymin,
            ymax: *ymax,
            color: style.color_value().with_alpha(style.alpha_value()),
            alpha: style.alpha_value(),
            label: style.label_value().unwrap_or("").to_string(),
        }),
        Series::AxVSpan { xmin, xmax, style } => CompiledSeries::AxVSpan(AxVSpanSeries {
            xmin: *xmin,
            xmax: *xmax,
            color: style.color_value().with_alpha(style.alpha_value()),
            alpha: style.alpha_value(),
            label: style.label_value().unwrap_or("").to_string(),
        }),
    }
}
