#!/usr/bin/env python3

import re
import argparse
import matplotlib.pyplot as plt


def parse_time_ns(value: str, unit: str) -> float:
    value = float(value)

    if unit == "ns":
        return value
    if unit in ("µs", "us"):
        return value * 1_000.0
    if unit == "ms":
        return value * 1_000_000.0
    if unit == "s":
        return value * 1_000_000_000.0

    raise ValueError(f"unknown time unit: {unit}")


def parse_bandwidth_gbs(value: str, unit: str) -> float:
    value = float(value)

    if unit == "GB/s":
        return value
    if unit == "MB/s":
        return value / 1000.0

    raise ValueError(f"unknown bandwidth unit: {unit}")


def parse_measurement_line(line: str, metric: str) -> float:
    if metric == "time":
        matches = re.findall(r"([\d.]+)\s+(ns|µs|us|ms|s)", line)
        if len(matches) < 3:
            raise ValueError(f"could not parse median time line: {line}")

        value, unit = matches[2]  # median
        return parse_time_ns(value, unit)

    if metric == "gbs":
        matches = re.findall(r"([\d.]+)\s+(GB/s|MB/s)", line)
        if len(matches) < 3:
            raise ValueError(f"could not parse median bandwidth line: {line}")

        value, unit = matches[2]  # median
        return parse_bandwidth_gbs(value, unit)

    raise ValueError(f"unknown metric: {metric}")


def parse_results(path: str, routine: str, metric: str):
    out = {"lak": [], "blas": []}
    current = None
    pending_n = None

    header_re = re.compile(r"[├╰]─\s+(lak|blas)_(\w+)")
    row_re = re.compile(r"[│ ]+[├╰]─\s+(\d+)\s+")

    with open(path, "r", encoding="utf-8") as f:
        for line in f:
            header = header_re.search(line)
            if header:
                lib, name = header.groups()
                current = lib if name == routine else None
                pending_n = None
                continue

            if current is None:
                continue

            row = row_re.search(line)
            if row:
                pending_n = int(row.group(1))

                if metric == "time":
                    out[current].append((pending_n, parse_measurement_line(line, metric)))
                    pending_n = None

                continue

            if metric == "gbs" and pending_n is not None and ("GB/s" in line or "MB/s" in line):
                out[current].append((pending_n, parse_measurement_line(line, metric)))
                pending_n = None

    return out


def plot_benchmark(data, routine: str, metric: str, output: str):
    plt.rcParams.update({
        "font.family": "serif",
        "mathtext.fontset": "cm",
        "font.size": 10,
    })

    fig, ax = plt.subplots(figsize=(6.5, 4.0))

    series = [
        ("lak", "lak", "#000000", "s"),
        ("blas", "OpenBLAS", "#000000", "^"),
    ]

    for key, label, color, marker in series:
        xs = [x for x, _ in data[key]]
        ys = [y for _, y in data[key]]

        ax.plot(
            xs,
            ys,
            color=color,
            marker=marker,
            markersize=3,
            linewidth=0.8,
            label=label,
        )

    ax.set_xscale("log", base=2)

    ax.set_xlabel(r"$n$")
    ax.set_ylabel("time [ns]" if metric == "time" else "GB/s")

    if metric == "time":
        ax.set_yscale("log")

    ax.legend(loc="lower right", frameon=False, fontsize=8)

    ax.text(
        0.0,
        -0.10,
        "apple m4 / rustc 1.94.0-nightly",
        transform=ax.transAxes,
        fontsize=8,
        alpha=0.55,
        ha="left",
        va="top",
        color="black",
    )

    ax.grid(False)
    fig.tight_layout()
    fig.savefig(output, dpi=300)
    plt.close(fig)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("file")
    parser.add_argument("routine")
    parser.add_argument("--metric", choices=["time", "gbs"], default="gbs")
    parser.add_argument("-o", "--output", default=None)

    args = parser.parse_args()

    suffix = "time" if args.metric == "time" else "gbs"
    output = args.output or f"plots/l1/{args.routine}_{suffix}.png"

    data = parse_results(args.file, args.routine, args.metric)

    if not data["lak"]:
        raise ValueError(f"no lak_{args.routine} results found")
    if not data["blas"]:
        raise ValueError(f"no blas_{args.routine} results found")

    plot_benchmark(data, args.routine, args.metric, output)


if __name__ == "__main__":
    main()
