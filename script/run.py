#!/usr/bin/env python3

import sys
import subprocess
import argparse
from datetime import datetime


def run_rust(year: int, day: int, part: int, fast: bool = False) -> None:
    cargo_args = ["cargo", "run", "-p", f"aoc-{year}"]
    if fast == True:
        cargo_args.extend(["--release"])

    sub_cmd_args = ["--", "--day", str(day), "--part", str(part)]
    cargo_args.extend(sub_cmd_args)

    result = subprocess.run(cargo_args)
    sys.exit(result.returncode)


def run_zig(year: int, day: int, part: int, fast: bool = False) -> None:
    zig_args = ["zig", "build", f"aoc-{year}-{day}"]
    if fast == True:
        zig_args.append("--release=fast")

    sub_cmd_args = ["--", "-p", str(part)]
    zig_args.extend(sub_cmd_args)

    result = subprocess.run(zig_args)
    sys.exit(result.returncode)


runners = {
    "rust": run_rust,
    "zig": run_zig,
}

if __name__ == "__main__":
    parser = argparse.ArgumentParser(prog="AoC solution runner")
    parser.add_argument("language", choices=runners.keys())
    parser.add_argument("-y", "--year", default=datetime.now().year, type=int)
    parser.add_argument("-d", "--day", default=datetime.now().day, type=int)
    parser.add_argument("-p", "--part", default=1, type=int)
    parser.add_argument(
        "-f",
        "--fast",
        help="Run optimised code, if available.",
        type=bool,
        default=False,
    )

    args = parser.parse_args()
    runner = runners[args.language]
    runner(args.year, args.day, args.part, args.fast)
