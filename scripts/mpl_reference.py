#!/usr/bin/env python3
"""Matplotlib reference figures mirroring mplot examples (for fidelity comparison)."""

from __future__ import annotations

from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

OUT = Path(__file__).resolve().parent.parent / "tests" / "fidelity" / "golden"
OUT.mkdir(parents=True, exist_ok=True)


def apply_mplot_rcparams() -> None:
    """Shared styling contract with mplot (see src/render/mpl_style.rs)."""
    matplotlib.rcParams.update(
        {
            "figure.dpi": 100,
            "font.size": 10,
            "axes.titlesize": 10,
            "axes.labelsize": 10,
            "xtick.labelsize": 10,
            "ytick.labelsize": 10,
            "axes.linewidth": 0.8,
            "lines.linewidth": 1.5,
            "font.family": "DejaVu Sans",
        }
    )


def save(fig, name: str) -> None:
    fig.tight_layout()
    fig.savefig(OUT / name, dpi=100)
    plt.close(fig)


def write_simple_line() -> None:
    x = [0.0, 1.0, 2.0, 3.0, 4.0]
    y = [0.0, 1.0, 4.0, 9.0, 16.0]
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.plot(x, y, color="#1f77b4", label="y = x²")
    ax.set_title("Simple line plot")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    save(fig, "simple_line.png")


def write_subplot_lines() -> None:
    fig, axes = plt.subplots(2, 2, squeeze=False, figsize=(10.0, 8.0))
    xs = [0.0, 1.0, 2.0, 3.0, 4.0]
    panels = [
        ("Panel A", "#1f77b4", [0.0, 0.5, 1.5, 2.0, 2.5]),
        ("Panel B", "#ff7f0e", [0.0, 0.8, 1.2, 1.8, 2.2]),
        ("Panel C", "#2ca02c", [0.0, 0.3, 1.0, 1.4, 1.9]),
        ("Panel D", "#d62728", [0.0, 0.6, 1.1, 1.6, 2.4]),
    ]
    for ax, (title, color, ys) in zip(axes.flatten(), panels):
        ax.plot(xs, ys, color=color)
        ax.set_title(title)
        ax.set_xlabel("x")
        ax.set_ylabel("y")
        ax.set_ylim(0.0, 3.0)
    save(fig, "subplot_lines.png")


def write_boxplot_linear() -> None:
    groups = [
        [1.2, 1.5, 1.8, 2.0, 2.1],
        [2.0, 2.3, 2.5, 2.8, 3.0, 3.2],
        [0.8, 1.0, 1.1, 1.4],
    ]
    labels = ["Group A\n(n=5)", "Group B\n(n=6)", "Group C\n(n=4)"]
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.boxplot(groups, tick_labels=labels)
    ax.set_title("Boxplot (linear y)")
    ax.set_xlabel("category")
    ax.set_ylabel("value")
    save(fig, "boxplot_linear.png")


def write_boxplot_log() -> None:
    groups = [
        [12.0, 18.0, 25.0, 31.0],
        [8.0, 15.0, 22.0, 29.0, 35.0],
        [10.0, 20.0, 40.0, 55.0],
    ]
    labels = ["Group A\n(n=4)", "Group B\n(n=5)", "Group C\n(n=4)"]
    fig, ax = plt.subplots(figsize=(8.0, 6.0))
    ax.boxplot(groups, tick_labels=labels)
    ax.set_xlabel("category")
    ax.set_ylabel("value")
    ax.set_yscale("log")
    ax.set_title("Boxplot (log y)")
    save(fig, "boxplot_log.png")


def write_gallery_line() -> None:
    x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0]
    y = [1.0, 1.4, 1.8, 2.2, 2.6, 3.0]
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.plot(x, y, color="#2ca02c")
    ax.set_title("Line plot")
    ax.set_xlabel("x")
    ax.set_ylabel("f(x)")
    save(fig, "gallery_line.png")


def write_gallery_subplots() -> None:
    fig, axes = plt.subplots(1, 2, squeeze=False, figsize=(10.0, 4.0))
    xs = [0.0, 2.0, 4.0, 6.0, 8.0]
    panels = [
        ("Sine-ish", "#1f77b4", [0.0, 0.9, 1.4, 1.2, 0.8]),
        ("Ramp", "#ff7f0e", [0.0, 0.5, 1.0, 1.5, 2.0]),
    ]
    for ax, (title, color, ys) in zip(axes.flatten(), panels):
        ax.plot(xs, ys, color=color)
        ax.set_title(title)
        ax.set_xlabel("x")
        ax.set_ylabel("y")
    save(fig, "gallery_subplots.png")


def write_gallery_boxplot() -> None:
    groups = [[2.0, 3.0, 4.0, 5.0], [4.0, 5.0, 6.0, 7.0, 8.0]]
    labels = ["Low", "High"]
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.boxplot(groups, tick_labels=labels)
    ax.set_title("Two-group boxplot")
    ax.set_xlabel("group")
    ax.set_ylabel("measurement")
    save(fig, "gallery_boxplot.png")


def write_bar_chart() -> None:
    x = [1.0, 2.0, 3.0, 4.0]
    heights = [3.0, 7.0, 5.0, 9.0]
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.bar(x, heights, width=0.8, color="#1f77b4")
    ax.set_title("Bar chart")
    ax.set_xlabel("x")
    ax.set_ylabel("height")
    save(fig, "bar_chart.png")


def write_histogram() -> None:
    data = [1.0, 1.5, 2.0, 2.2, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0]
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.hist(data, bins=10, color="#ff7f0e")
    ax.set_title("Histogram")
    ax.set_xlabel("value")
    ax.set_ylabel("count")
    save(fig, "histogram.png")


def write_fill_between() -> None:
    x = [0.0, 1.0, 2.0, 3.0, 4.0]
    y1 = [0.0, 1.0, 2.0, 1.0, 0.0]
    y2 = [0.5, 1.5, 2.5, 1.5, 0.5]
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.fill_between(x, y1, y2, color="#2ca02c", alpha=0.3)
    ax.set_title("Fill between")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    save(fig, "fill_between.png")


def write_image_viridis() -> None:
    data = [
        [0.0, 0.5, 1.0],
        [0.25, 0.75, 1.0],
        [0.0, 0.5, 1.0],
    ]
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.imshow(data, extent=(0.0, 3.0, 0.0, 3.0), cmap="viridis", origin="lower")
    ax.set_title("Image (viridis)")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    save(fig, "image_viridis.png")


def write_contour() -> None:
    import numpy as np

    xs = np.linspace(0.0, 3.0, 4)
    ys = np.linspace(0.0, 3.0, 4)
    x, y = np.meshgrid(xs, ys)
    z = 0.25 * (x + y)
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.contour(x, y, z, levels=[0.25, 0.5, 0.75], colors="#1f77b4")
    ax.set_title("Contour plot")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    save(fig, "contour.png")


def write_line_log_x() -> None:
    x = [1.0, 10.0, 100.0, 1000.0]
    y = [1.0, 2.0, 3.0, 4.0]
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.plot(x, y, color="#1f77b4")
    ax.set_xscale("log")
    ax.set_title("Log x line plot")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    save(fig, "line_log_x.png")


def write_line_log_log() -> None:
    x = [1.0, 10.0, 100.0]
    y = [10.0, 100.0, 1000.0]
    fig, ax = plt.subplots(figsize=(6.4, 4.8))
    ax.plot(x, y, color="#ff7f0e")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_title("Log-log line plot")
    ax.set_xlabel("x")
    ax.set_ylabel("y")
    save(fig, "line_log_log.png")


if __name__ == "__main__":
    apply_mplot_rcparams()
    write_simple_line()
    write_subplot_lines()
    write_boxplot_linear()
    write_boxplot_log()
    write_gallery_line()
    write_gallery_subplots()
    write_gallery_boxplot()
    write_bar_chart()
    write_histogram()
    write_fill_between()
    write_image_viridis()
    write_contour()
    write_line_log_x()
    write_line_log_log()
    print(f"wrote reference figures under {OUT}")
