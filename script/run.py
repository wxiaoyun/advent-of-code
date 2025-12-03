#!/usr/bin/env python3

import sys
import subprocess
import argparse
from datetime import datetime


def get_input_path(args: argparse.Namespace) -> str:
    if args.input_file:
        return args.input_file

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
    zig_args = ["zig", "build", f"aoc-{args.year}-{args.day:0>2}"]
    if args.fast == True:
        zig_args.append("--release=fast")

    sub_cmd_args = ["--", str(args.part)]
    zig_args.extend(sub_cmd_args)

    result = subprocess.run(zig_args, stdin=open(get_input_path(args), "r"))
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

    result = subprocess.run(opam_args, stdin=open(get_input_path(args), "r"))
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
    parser.add_argument(
        "-d", "--day", choices=range(1, 26), default=datetime.now().day, type=int
    )
    parser.add_argument("-p", "--part", choices=[1, 2], default=1, type=int)
    parser.add_argument(
        "-I",
        "--input-file",
        help="Input file path. Defaults to <year>/input/<day>.txt",
        type=str,
    )
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
