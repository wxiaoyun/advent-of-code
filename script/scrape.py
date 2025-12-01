#!/usr/bin/env python3

import sys
import os
import subprocess
import argparse
from datetime import datetime
import dotenv

if __name__ == "__main__":
    parser = argparse.ArgumentParser(prog="AoC question scraper")
    parser.add_argument("-y", "--year", default=datetime.now().year, type=int)
    parser.add_argument("-d", "--day", type=int)
    parser.add_argument(
        "-s",
        "--session-token",
        help="Advent of Code session token, if not provided, will load from .env",
    )
    parser.add_argument("-m", "--max-retries", default=3, type=int)
    parser.add_argument("-D", "--delay", default=5, type=int)
    parser.add_argument("-o", "--output-dir", type=str)
    args = parser.parse_args()

    cargo_args = ["cargo", "run", "-p", "scraper", "--release"]
    scraper_args = [
        "--",
        "--year",
        str(args.year),
        "--max-retries",
        str(args.max_retries),
        "--delay",
        str(args.delay),
    ]

    if not args.session_token:
        dotenv.load_dotenv()
        args.session_token = os.getenv("ADVENT_OF_CODE_SESSION")
        if not args.session_token:
            print("Error: ADVENT_OF_CODE_SESSION is not set")
            sys.exit(1)
        else:
            scraper_args.extend(["--session-token", args.session_token])

    if args.day:
        scraper_args.extend(["--day", str(args.day)])

    if args.output_dir:
        scraper_args.extend(["--output-dir", args.output_dir])

    cargo_args.extend(scraper_args)
    result = subprocess.run(cargo_args)

    sys.exit(result.returncode)
