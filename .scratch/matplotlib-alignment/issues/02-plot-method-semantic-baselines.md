# Select semantic baselines for the eight plot methods

Type: research
Status: resolved
Blocked by: Define the public vocabulary and compatibility matrix schema

## Question

For `Axes.plot`, `boxplot`, `bar`, `hist`, `fill_between`, `imshow`, `contour`, and `text` at the pinned Matplotlib commit, which finite typed inputs and canonical options will release 1 support, adapt, or reject, and what exact defaults, validation outcomes, data-limit effects, state interactions, and rendering semantics must each selected option reproduce?

The answer must fill the approved compatibility matrix per method, prioritize existing internal capabilities plus high-value options required by a representative reference corpus, identify Matplotlib source locations, and explicitly exclude unselected kwargs/overloads. It must cover option-builder names such as `PlotOptions`, `HistOptions`, and `ImshowOptions`, while keeping Artist/container handles out of scope.

## Answer

Resolved in [`../research/02-plot-method-semantic-baselines.md`](../research/02-plot-method-semantic-baselines.md).

Release 1 uses one explicit typed Rust input shape per advertised family and canonical method/option names, while adapting every Matplotlib Artist/container return to `&mut AxesBuilder` and rejecting all unlisted Python overloads and kwargs. The finite matrix pins each selected default, validation rule, data-limit/state effect, and rendering semantic. In particular it requires `bottom + height` bars, NumPy-compatible histogram edges/ranges, Matplotlib box statistics/styles, missing-data path breaks, upper-origin/tight-default `imshow`, locator-derived `contour` levels with norm scaled to levels, retained color cycles, and text that does not affect data limits.
