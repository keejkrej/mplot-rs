#!/usr/bin/env python3
"""Matplotlib reference figures mirroring mplot examples (for fidelity comparison)."""

from __future__ import annotations

from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

OUT = Path(__file__).resolve().parent.parent / "tests" / "fidelity" / "golden"
OUT.mkdir(parents=True, exist_ok=True)


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


if __name__ == "__main__":
    write_simple_line()
    write_subplot_lines()
    write_boxplot_linear()
    write_boxplot_log()
    write_gallery_line()
    write_gallery_subplots()
    write_gallery_boxplot()
    print(f"wrote reference figures under {OUT}")
