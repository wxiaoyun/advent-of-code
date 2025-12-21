#!/usr/bin/env python3

import argparse
import os
import subprocess
import sys
from datetime import datetime
from typing import IO


def get_input_io(args: argparse.Namespace) -> IO:
    if not sys.stdin.isatty():
        return sys.stdin

    if args.input_file:
        return open(args.input_file, "r")

    import scrape

    scraped_file_path = (
        scrape.input_path_of(args.year, args.day)
        if args.test == 0
        else scrape.test_input_path_of(args.year, args.day, args.test)
    )

    if args.auto_download and not os.path.exists(scraped_file_path):
        print(f"Downloading input for year {args.year}, day {args.day}")
        result = scrape.scrape(
            args.year,
            args.day,
        )
        if result.returncode != 0:
            sys.exit(result.returncode)

    try:
        return open(scraped_file_path, "r")
    except:
        print(
            f"Error opening scraped input file at {scraped_file_path}, please run the scraper first.",
            file=sys.stderr,
        )
        sys.exit(1)


def run_rust(args: argparse.Namespace, input: IO) -> None:
    cargo_args = ["cargo", "run", "-p", f"aoc-{args.year}"]
    if args.fast:
        cargo_args.extend(["--release"])

    sub_cmd_args = ["--", str(args.day), str(args.part)]
    cargo_args.extend(sub_cmd_args)

    result = subprocess.run(cargo_args, stdin=input)
    sys.exit(result.returncode)


def run_zig(args: argparse.Namespace, input: IO) -> None:
    zig_args = ["zig", "build", f"aoc-{args.year}-{args.day:0>2}"]
    if args.fast:
        zig_args.append("--release=fast")

    sub_cmd_args = ["--", str(args.part)]
    zig_args.extend(sub_cmd_args)

    result = subprocess.run(zig_args, stdin=input)
    sys.exit(result.returncode)


def run_ocaml(args: argparse.Namespace, input: IO) -> None:
    opam_args = ["opam", "exec"]
    dune_args = [
        "--",
        "dune",
        "exec",
        f"{args.year}/ocaml/day{args.day:0>2}.exe",
    ]
    sub_cmd_args = [
        "--",
        str(args.part),
    ]
    opam_args.extend(dune_args)
    opam_args.extend(sub_cmd_args)

    result = subprocess.run(opam_args, stdin=input)
    sys.exit(result.returncode)


runners = {
    "rust": run_rust,
    "zig": run_zig,
    "ocaml": run_ocaml,
}

if __name__ == "__main__":
    parser = argparse.ArgumentParser(prog="AoC solution runner")
    parser.add_argument("language", choices=runners.keys())
    parser.add_argument("-y", "--year", default=datetime.now().year, type=int)
    parser.add_argument("-d", "--day", default=datetime.now().day, type=int)
    parser.add_argument("-p", "--part", choices=[1, 2], default=1, type=int)
    parser.add_argument(
        "-I",
        "--input-file",
        help="Input file path. Defaults to <year>/input/<day>.txt",
        type=str,
    )
    parser.add_argument(
        "-a",
        "--auto-download",
        help="Automatically download the input files to the default location and pipe to the program",
        default=True,
        action=argparse.BooleanOptionalAction,
    )
    parser.add_argument(
        "-t",
        "--test",
        help="Run with test input",
        type=int,
        default=0,
    )
    parser.add_argument(
        "-f",
        "--fast",
        help="Run optimised code, if available.",
        default=False,
        action=argparse.BooleanOptionalAction,
    )

    args = parser.parse_args()
    runner = runners[args.language]
    input = get_input_io(args)
    runner(args, input)
