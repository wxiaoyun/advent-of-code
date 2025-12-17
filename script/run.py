#!/usr/bin/env python3

import argparse
import subprocess
import sys
from datetime import datetime
from typing import IO


def pipe_input(args: argparse.Namespace) -> IO:
    if not sys.stdin.isatty():
        return sys.stdin

    if args.input_file:
        return open(args.input_file, "r")

    import scrape

    try:
        scraped_file_path = (
            scrape.input_path_of(args.year, args.day)
            if args.test == 0
            else scrape.test_input_path_of(args.year, args.day, args.test)
        )
        return open(scraped_file_path, "r")
    except:
        print(
            f"Error opening scraped input file at {scraped_file_path}, please run the scraper first.",
            file=sys.stderr,
        )
        sys.exit(1)


def run_rust(args: argparse.Namespace) -> None:
    cargo_args = ["cargo", "run", "-p", f"aoc-{args.year}"]
    if args.fast:
        cargo_args.extend(["--release"])

    sub_cmd_args = ["--", str(args.day), str(args.part)]
    cargo_args.extend(sub_cmd_args)

    result = subprocess.run(cargo_args, stdin=pipe_input(args))
    sys.exit(result.returncode)


def run_zig(args: argparse.Namespace) -> None:
    zig_args = ["zig", "build", f"aoc-{args.year}-{args.day:0>2}"]
    if args.fast:
        zig_args.append("--release=fast")

    sub_cmd_args = ["--", str(args.part)]
    zig_args.extend(sub_cmd_args)

    result = subprocess.run(zig_args, stdin=pipe_input(args))
    sys.exit(result.returncode)


def run_ocaml(args: argparse.Namespace) -> None:
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

    result = subprocess.run(opam_args, stdin=pipe_input(args))
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
        action=argparse.BooleanOptionalAction
    )

    args = parser.parse_args()
    runner = runners[args.language]
    runner(args)
