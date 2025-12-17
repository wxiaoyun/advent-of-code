#!/usr/bin/env python3

import argparse
import os
import subprocess
import sys
from datetime import datetime

INPUT_FOLDER_NAME = "input"


def input_path_of(year: int, day: int) -> str:
    return f"{INPUT_FOLDER_NAME}/{year}_{day:0>2}.txt"


def test_input_path_of(year: int, day: int, part: int) -> str:
    return f"{INPUT_FOLDER_NAME}/{year}_{day:0>2}_{part}.test.txt"


if __name__ == "__main__":
    parser = argparse.ArgumentParser(prog="AoC question scraper")
    parser.add_argument("-y", "--year", default=str(datetime.now().year), type=str)
    parser.add_argument("-d", "--days", default=str(datetime.now().day), type=str)
    parser.add_argument(
        "-s",
        "--session-token",
        help="Advent of Code session token, if not provided, will load from .env",
    )
    parser.add_argument("-m", "--max-retries", default=3, type=int)
    parser.add_argument("-D", "--delay", default=5, type=int)
    parser.add_argument("-o", "--output-dir", type=str)
    args = parser.parse_args()

    cargo_args = ["cargo", "run", "-p", "aoc-scraper", "--release"]
    scraper_args = [
        "--",
        "--year",
        str(args.year),
        "--day",
        str(args.days),
        "--max-retries",
        str(args.max_retries),
        "--delay",
        str(args.delay),
        "--output-dir",
        INPUT_FOLDER_NAME,
    ]

    if not args.session_token:
        import dotenv

        dotenv.load_dotenv()
        args.session_token = os.getenv("ADVENT_OF_CODE_SESSION")
        if not args.session_token:
            print("Error: ADVENT_OF_CODE_SESSION is not set")
            sys.exit(1)
        else:
            scraper_args.extend(["--session-token", args.session_token])

    cargo_args.extend(scraper_args)
    result = subprocess.run(cargo_args)
    sys.exit(result.returncode)
