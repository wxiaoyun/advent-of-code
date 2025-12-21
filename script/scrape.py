#!/usr/bin/env python3

import argparse
import os
import subprocess
import sys
from datetime import datetime
from typing import Optional

INPUT_FOLDER_NAME = "input"


def input_path_of(year: int, day: int) -> str:
    return f"{INPUT_FOLDER_NAME}/{year}_{day:0>2}.txt"


def test_input_path_of(year: int, day: int, part: int) -> str:
    return f"{INPUT_FOLDER_NAME}/{year}_{day:0>2}_{part}.test.txt"


def scrape(
    year: str,
    day: str,
    max_retries: int = 3,
    delay: int = 1,
    output_dir: str = INPUT_FOLDER_NAME,
    session_token: Optional[str] = None,
):
    if not session_token:
        import dotenv

        dotenv.load_dotenv()
        session_token = os.getenv("ADVENT_OF_CODE_SESSION")
        if not session_token:
            print("Error: ADVENT_OF_CODE_SESSION is not set")
            sys.exit(1)

    cargo_args = ["cargo", "run", "-p", "aoc-scraper", "--release"]
    scraper_args = [
        "--",
        "--year",
        str(year),
        "--day",
        str(day),
        "--max-retries",
        str(max_retries),
        "--delay",
        str(delay),
        "--output-dir",
        str(output_dir),
        "--session-token",
        str(session_token),
    ]

    cargo_args.extend(scraper_args)
    result = subprocess.run(cargo_args)
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(prog="AoC question scraper")
    parser.add_argument("-y", "--year", default=str(datetime.now().year), type=str)
    parser.add_argument("-d", "--day", default=str(datetime.now().day), type=str)
    parser.add_argument(
        "-s",
        "--session-token",
        help="Advent of Code session token, if not provided, will load from .env",
    )
    parser.add_argument("-m", "--max-retries", default=3, type=int)
    parser.add_argument("-D", "--delay", default=5, type=int)
    parser.add_argument("-o", "--output-dir", type=str)
    args = parser.parse_args()

    result = scrape(
        args.year,
        args.day,
        args.max_retries,
        args.delay,
        session_token=args.session_token,
    )
    sys.exit(result.returncode)
