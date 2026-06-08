use crate::gridspec::SubplotSlot;
use crate::render::mpl_style::{
    MPL_SUBPLOT_BOTTOM, MPL_SUBPLOT_LEFT, MPL_SUBPLOT_RIGHT, MPL_SUBPLOT_TOP, MPL_TIGHT_BOTTOM,
    MPL_TIGHT_LEFT, MPL_TIGHT_RIGHT, MPL_TIGHT_TOP,
};

#[derive(Clone, Copy, Debug)]
pub struct PanelRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub fn panel_rect_for_slot(
    figure_width_px: u32,
    figure_height_px: u32,
    slot: &SubplotSlot,
    wspace: f64,
    hspace: f64,
    tight: bool,
    constrained: bool,
) -> PanelRect {
    let use_tight = tight || constrained;
    let (left, right, bottom, top) = if use_tight {
        (MPL_TIGHT_LEFT, MPL_TIGHT_RIGHT, MPL_TIGHT_BOTTOM, MPL_TIGHT_TOP)
    } else {
        (
            MPL_SUBPLOT_LEFT,
            MPL_SUBPLOT_RIGHT,
            MPL_SUBPLOT_BOTTOM,
            MPL_SUBPLOT_TOP,
        )
    };

    let fw = figure_width_px as f64;
    let fh = figure_height_px as f64;
    let cols = slot.grid_cols().max(1);
    let rows = slot.grid_rows().max(1);
    let usable_w = fw * (right - left);
    let usable_h = fh * (top - bottom);

    let cell_w = usable_w / (cols as f64 + (cols.saturating_sub(1) as f64) * wspace);
    let cell_h = usable_h / (rows as f64 + (rows.saturating_sub(1) as f64) * hspace);
    let gap_w = wspace * cell_w;
    let gap_h = hspace * cell_h;
    let cell_h_frac = cell_h / fh;
    let gap_h_frac = gap_h / fh;

    let x = (left * fw + slot.col() as f64 * (cell_w + gap_w)).round() as u32;
    let width = (slot.colspan() as f64 * cell_w
        + slot.colspan().saturating_sub(1) as f64 * gap_w)
        .max(1.0)
        .round() as u32;

    let mpl_bottom_frac = bottom
        + (rows - slot.row() - slot.rowspan()) as f64 * (cell_h_frac + gap_h_frac);
    let mpl_top_frac =
        mpl_bottom_frac + slot.rowspan() as f64 * cell_h_frac + (slot.rowspan().saturating_sub(1) as f64) * gap_h_frac;

    PanelRect {
        x,
        y: (fh * (1.0 - mpl_top_frac)).round() as u32,
        width,
        height: ((mpl_top_frac - mpl_bottom_frac) * fh).max(1.0).round() as u32,
    }
}

pub fn subplot_panels(
    figure_width_px: u32,
    figure_height_px: u32,
    rows: usize,
    cols: usize,
    wspace: f64,
    hspace: f64,
    tight: bool,
) -> Vec<PanelRect> {
    if rows == 0 || cols == 0 {
        return Vec::new();
    }

    let mut panels = Vec::with_capacity(rows * cols);
    for row in 0..rows {
        for col in 0..cols {
            let slot = SubplotSlot {
                grid_rows: rows,
                grid_cols: cols,
                row,
                col,
                rowspan: 1,
                colspan: 1,
            };
            panels.push(panel_rect_for_slot(
                figure_width_px,
                figure_height_px,
                &slot,
                wspace,
                hspace,
                tight,
                false,
            ));
        }
    }
    panels
}

pub fn pad_inches_px(pad_inches: Option<f64>, dpi: u32) -> u32 {
    pad_inches
        .map(|pad| (pad * dpi as f64).round() as u32)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subplot_grid_has_expected_count() {
        let panels = subplot_panels(1000, 800, 2, 2, 0.2, 0.2, false);
        assert_eq!(panels.len(), 4);
        assert!(panels[0].width > 0);
        assert!(panels[0].height > 0);
    }

    #[test]
    fn span_is_wider_than_single_cell() {
        let gs = crate::gridspec::GridSpec::new(2, 2);
        let single = panel_rect_for_slot(1000, 800, &gs.at(0, 0), 0.2, 0.2, false, false);
        let span = panel_rect_for_slot(1000, 800, &gs.span(0, 0, 1, 2), 0.2, 0.2, false, false);
        assert!(span.width > single.width);
    }
}
