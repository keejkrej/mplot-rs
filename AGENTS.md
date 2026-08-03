# AGENTS.md

Humans review this file. Agents treat it as project law when changing mplot.

## Purpose

**mplot** — native Rust 2D plotting with **matplotlib parity** as the north star.

- **Defaults** match matplotlib (rcParams, geometry, title placement, tick/label layout, colors, fonts). When in doubt, open sibling `../matplotlib` and copy behavior — do not invent a new visual system.
- **API** exposes the same *concepts* as matplotlib (`Figure`, axes title/labels/scales/limits, series styles, gridspec, save dpi/format) via an idiomatic **builder pattern**, not a 1:1 method-name port of pyplot.
- **Rendering fidelity** is verified against matplotlib goldens (`scripts/mpl_reference.py` → `tests/fidelity/`; rc mapping in `src/render/mpl_style.rs`).

## Matplotlib alignment (non-negotiable)

| Concern | Matplotlib rule (follow this) |
|---------|--------------------------------|
| Axes title | Centered on the **Axes box** (`transAxes` x=0.5), not the full panel cell including tick gutters. `axes.titlelocation=center`, `axes.titlepad=6`. Source: `lib/matplotlib/axes/_base.py` / `_axes.py` `set_title`. |
| Tick / spine geometry | Match mpl tick size, pad, axes linewidth, label pad. |
| Fonts / sizes | Default face DejaVu Sans; sizes from `mpl_style` constants synced with `scripts/mpl_reference.py`. |
| Series defaults | Line width, boxplot face/edge/median colors, grid color — matplotlib-like. |
| Figure size / dpi | Size in **inches**; export dpi configurable (fidelity goldens use 100). |

If plotters (or any backend) cannot express an mpl default (e.g. arbitrary tick rotation, title-on-axes-box), **implement the mpl behavior in mplot** rather than accepting the backend’s approximation.

## API shape

```text
Figure::builder() → .panel(...) { series + AxesStyle } → .build()? → .save(path, SaveOptions)
```

- Prefer fluent builders (`AxesStyle::new().title(...).x_label(...)`) over free functions that mirror `plt.*`.
- Name methods after matplotlib *intent* where natural (`x_scale(Scale::Log)`, `x_tick_labels`, title/labels), not after plotters internals.

## Rules

- **`README.md`** — user-facing quick start; keep short.
- **`src/render/mpl_style.rs`** — single source of rc-like constants; update the mapping table when adding a default.
- **Fidelity** — after visual/default changes that affect goldens, regenerate with `scripts/mpl_reference.py` and re-run `cargo test --test fidelity`.
- **No new chart defaults** that diverge from matplotlib without an explicit comment and, preferably, an rcParams reference.

## Layout (key paths)

| Path | Role |
|------|------|
| `src/panel.rs` | Public axes/tick/series style surface |
| `src/render/canvas.rs` | Plotters mesh + custom draw (rotated ticks, titles, …) |
| `src/render/mpl_style.rs` | Matplotlib default constants |
| `scripts/mpl_reference.py` | Golden PNG generator |
| `tests/fidelity/` | Pixel/regression against mpl |

## Status note

Early development. Known gaps vs matplotlib should be fixed toward parity (example: subplot titles must center on the plot box, not the full cell — plotters `.caption` is not sufficient).
