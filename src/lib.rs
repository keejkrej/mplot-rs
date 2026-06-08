//! Native Rust 2D plotting with matplotlib-inspired rendering fidelity.
//!
//! Build figures with [`FigureBuilder`], add series per panel, then export with
//! [`Figure::save`].
//!
//! # Example
//!
//! ```
//! use mplot::prelude::*;
//!
//! # fn demo() -> mplot::Result<()> {
//! let x = [0.0, 1.0, 2.0, 3.0, 4.0];
//! let y = [0.0, 1.0, 4.0, 9.0, 16.0];
//!
//! let figure = Figure::builder()
//!     .panel(GridPos::new(1, 1, 1), |p| {
//!         p.line(
//!             &x,
//!             &y,
//!             LineStyle::new()
//!                 .color(Color::hex("#1f77b4"))
//!                 .label("y = x²"),
//!         )
//!         .axes(
//!             AxesStyle::new()
//!                 .title("Simple line plot")
//!                 .x_label("x")
//!                 .y_label("y"),
//!         );
//!     })
//!     .build()?;
//!
//! figure.save("/tmp/mplot/example.png", SaveOptions::default())?;
//! # Ok(())
//! # }
//! ```

mod as_vector;
mod auxiliary;
mod axes;
mod color;
mod colormap;
mod compile;
mod constants;
mod error;
mod figure;
mod panel;
mod render;
mod scale;
mod series;
mod ticker;

pub mod prelude {
    pub use crate::color::Color;
    pub use crate::colormap::{Colormap, Normalize};
    pub use crate::error::{Error, Result};
    pub use crate::figure::{Figure, FigureBuilder, SaveOptions, Size};
    pub use crate::panel::{AxesStyle, GridPos, LegendStyle, TickLabels};
    pub use crate::series::{
        BarStyle, BoxplotStyle, ContourStyle, FillBetweenStyle, HistStyle, ImageStyle, LineDash,
        LineStyle, Marker, Scale, Series, TextStyle,
    };
}

pub use as_vector::*;
pub use auxiliary::*;
pub use color::*;
pub use error::*;
pub use figure::*;
pub use panel::*;
pub use series::*;
