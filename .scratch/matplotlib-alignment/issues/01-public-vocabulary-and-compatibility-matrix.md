# Define the public vocabulary and compatibility matrix schema

Type: research
Status: resolved
Blocked by: none

## Question

Against the pinned Matplotlib checkout, what is the exhaustive mapping from mplot's currently public concepts and names to the finite Matplotlib object-oriented vocabulary, and what matrix schema will unambiguously classify each method, option, default, return-value adaptation, and unsupported boundary as `Supported`, `Adapted`, or `Unsupported` with source references?

The answer must inventory the existing Rust public surface, propose the target type/module/method names (including `Panel` to `Axes` domain changes), distinguish exact names from deliberate typed/builder adaptations, and identify any accidental public internals that should become private. It must not choose per-method behavioral baselines that belong to later tickets.

## Answer

Resolved in [Public vocabulary and compatibility-matrix schema](../research/01-public-vocabulary-and-compatibility-matrix.md).

The exhaustive inventory adopts Matplotlib's OO nouns and spellings while retaining `Figure::builder()`, closure-configured `AxesBuilder`, typed method-derived option builders, and build-time validation. Old aliases are not retained. `PanelBuilder` becomes `AxesBuilder`; `panel`, `line`, `histogram`, `image`, and `save` become `add_subplot`, `plot`, `hist`, `imshow`, and `savefig`; style option spellings likewise become `linewidth`, `linestyle`, `bottom`, `whis`, `orientation`, `cmap`, and `norm`.

The inventory also identifies the wildcard-leaked `AsVector`, `vector_to_f64`, `linspace`, public `Series` representation/constructors, `PanelBuilder::series`, old wrapper types, and low-level representation/evaluation helpers as accidental public internals to remove or privatize.

The checked-in matrix must use one atomic row for each type, method, option, default, return, validation, continuation, and unsupported boundary. `Supported` requires both canonical surface and finite behavioral parity; `Adapted` must state the exact Rust typing/ownership/builder/return difference; `Unsupported` must state the finite boundary and migration replacement. Parent status never classifies child defaults, options, or returns, and every inspected in-scope row must cite the pinned Matplotlib source.
