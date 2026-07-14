# Prove the deferred-validation and Axes state model

Type: prototype
Status: open
Blocked by: Select semantic baselines for the eight plot methods; Select Figure, Axes, layout, and ancillary semantic baselines

## Question

What internal builder/state model can preserve infallible Rust chaining while faithfully accumulating Matplotlib-like Axes state and surfacing every fatal supported-input problem as a contextual `FigureBuilder::build()` error?

Use a throwaway design or runnable spike as needed to settle: error accumulation and context (`Axes`/subplot, originating method, option/data); no silent clamps or drops; property-cycle progression; multi- and mixed-series autoscaling; insertion order and selected `zorder`; legend label filtering/order; and deterministic continuation for cases where pinned Matplotlib warns. The answer is an architectural decision and evidence, not production implementation.

## Answer

_Unresolved._
