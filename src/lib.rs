//! Native Rust plotting library with a plotpy-compatible API.
//!
//! plotpy generates Python/Matplotlib scripts; mplot renders figures directly in Rust.

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
