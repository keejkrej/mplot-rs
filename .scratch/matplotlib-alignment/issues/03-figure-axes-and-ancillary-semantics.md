# Select Figure, Axes, layout, and ancillary semantic baselines

Type: research
Status: resolved
Blocked by: Define the public vocabulary and compatibility matrix schema

## Question

What finite Matplotlib-compatible semantics will release 1 claim for `Figure`/`Axes` construction and setters, subplot and GridSpec/span addressing, scales and ticks, legends, colorbars, color/colormap inputs, layout selection, and `savefig` across PNG/SVG/PDF?

The answer must fill the compatibility matrix for every currently advertised ancillary feature; establish pristine pinned-Matplotlib defaults (including DPI, bbox/layout, margins, cycles, and save behavior); separate exact support from Rust adaptations; and state the finite supported layout cases without implying general layout-engine equivalence.

## Answer

Resolved in [Figure, Axes, layout, and ancillary semantic baseline](../research/03-figure-axes-and-ancillary-semantics.md).

Release 1 adopts canonical Figure/Axes/GridSpec/SubplotSpec and setter names
through retained Rust builders, restores pristine pinned defaults (including
100 DPI, standard save bbox, 0.1-inch tight padding, 5% margins, independent
`tab10` cycles, and no automatic layout), and validates queued contextual
errors at `build()`. Layout support is expressly limited to enumerated 1x1,
small equal-grid, single-span, and right-colorbar cases; it is not a claim of a
general tight/constrained engine. PNG is the normative visual oracle, while
SVG/PDF share the semantic scene and receive structural/rasterization smoke
coverage. All other overloads, engines, Artist handles, and arbitrary kwargs
are explicitly unsupported.
