//! Native Rust plotting with a plotpy-compatible API.
//!
//! [plotpy](https://github.com/cpmech/plotpy) generates Python/Matplotlib scripts;
//! **mplot** renders figures directly in Rust while exposing the same builder-style
//! workflow (`Curve`, `Boxplot`, `Plot`, …).
//!
//! # Example
//!
//! ```
//! use mplot::{Curve, Plot, StrError};
//!
//! fn main() -> Result<(), StrError> {
//!     let x = [0.0, 1.0, 2.0, 3.0];
//!     let y = [0.0, 1.0, 4.0, 9.0];
//!
//!     let mut curve = Curve::new();
//!     curve.set_line_color("#1f77b4");
//!     curve.draw(&x, &y);
//!
//!     let mut plot = Plot::new();
//!     plot.add(&curve).set_labels("x", "y");
//!     plot.save("/tmp/mplot/example.png")?;
//!     Ok(())
//! }
//! ```

pub type StrError = &'static str;

mod as_vector;
mod auxiliary;
mod boxplot;
mod color;
mod constants;
mod curve;
mod graph;
mod plot;
mod render;

pub use as_vector::*;
pub use auxiliary::*;
pub use boxplot::*;
pub use constants::*;
pub use curve::*;
pub use graph::*;
pub use plot::*;
