/// Grid definition for subplot layout (matplotlib `GridSpec`-style).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridSpec {
    rows: usize,
    cols: usize,
}

/// Address of one panel inside a grid, optionally spanning multiple cells.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubplotSlot {
    pub(crate) grid_rows: usize,
    pub(crate) grid_cols: usize,
    pub(crate) row: usize,
    pub(crate) col: usize,
    pub(crate) rowspan: usize,
    pub(crate) colspan: usize,
}

impl GridSpec {
    pub fn new(rows: usize, cols: usize) -> Self {
        GridSpec {
            rows: rows.max(1),
            cols: cols.max(1),
        }
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn at(self, row: usize, col: usize) -> SubplotSlot {
        self.span(row, col, 1, 1)
    }

    pub fn span(self, row: usize, col: usize, rowspan: usize, colspan: usize) -> SubplotSlot {
        SubplotSlot {
            grid_rows: self.rows,
            grid_cols: self.cols,
            row,
            col,
            rowspan: rowspan.max(1),
            colspan: colspan.max(1),
        }
    }
}

impl SubplotSlot {
    pub fn grid_rows(&self) -> usize {
        self.grid_rows
    }

    pub fn grid_cols(&self) -> usize {
        self.grid_cols
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn rowspan(&self) -> usize {
        self.rowspan
    }

    pub fn colspan(&self) -> usize {
        self.colspan
    }

    pub(crate) fn from_index(rows: usize, cols: usize, index: usize) -> Self {
        let rows = rows.max(1);
        let cols = cols.max(1);
        let zero = index.saturating_sub(1);
        SubplotSlot {
            grid_rows: rows,
            grid_cols: cols,
            row: zero / cols,
            col: zero % cols,
            rowspan: 1,
            colspan: 1,
        }
    }

    pub(crate) fn from_parts(
        grid_rows: usize,
        grid_cols: usize,
        row: usize,
        col: usize,
        rowspan: usize,
        colspan: usize,
    ) -> Self {
        SubplotSlot {
            grid_rows: grid_rows.max(1),
            grid_cols: grid_cols.max(1),
            row,
            col,
            rowspan: rowspan.max(1),
            colspan: colspan.max(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_maps_to_row_col() {
        let slot = SubplotSlot::from_index(2, 2, 3);
        assert_eq!(slot.row(), 1);
        assert_eq!(slot.col(), 0);
    }

    #[test]
    fn span_covers_multiple_cells() {
        let gs = GridSpec::new(2, 2);
        let slot = gs.span(0, 0, 1, 2);
        assert_eq!(slot.rowspan(), 1);
        assert_eq!(slot.colspan(), 2);
    }
}
