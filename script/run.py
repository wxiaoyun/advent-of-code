#!/usr/bin/env python3

import sys
import subprocess
import argparse
from datetime import datetime


def get_input_path(args: argparse.Namespace) -> str:
    from scrape import INPUT_FOLDER_NAME

    return f"{args.year}/{INPUT_FOLDER_NAME}/{args.day}.txt"


def run_rust(args: argparse.Namespace) -> None:
    cargo_args = ["cargo", "run", "-p", f"aoc-{args.year}"]
    if args.fast == True:
        cargo_args.extend(["--release"])

    sub_cmd_args = ["--", str(args.day), str(args.part)]
    cargo_args.extend(sub_cmd_args)

    result = subprocess.run(cargo_args, stdin=open(get_input_path(args), "r"))
    sys.exit(result.returncode)


def run_zig(args: argparse.Namespace) -> None:
    zig_args = ["zig", "build", f"aoc-{args.year}-{args.day}"]
    if args.fast == True:
        zig_args.append("--release=fast")

    sub_cmd_args = ["--", str(args.part)]
    zig_args.extend(sub_cmd_args)

    result = subprocess.run(zig_args, stdin=open(get_input_path(args), "r"))
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
    runner(args)
