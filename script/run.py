#!/usr/bin/env python3

import sys
import subprocess
import argparse
from datetime import datetime


def run_rust(year: int, day: int, part: int, fast: bool = False) -> None:
    args = ["cargo", "run", "-p", f"aoc-{year}"]
    if fast == True:
        args.extend(["--release"])

    sub_cmd_args = ["--", "--day", str(day), "--part", str(part)]
    args.extend(sub_cmd_args)

    completed_process = subprocess.run(args)
    sys.exit(completed_process.returncode)


if __name__ == "__main__":
    runners = {"rust": run_rust}

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
