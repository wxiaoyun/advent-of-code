#!/usr/bin/env python3

import os
import sys
import time
import re
import requests
from typing import Dict
from dotenv import load_dotenv
from bs4 import BeautifulSoup


def generate_question_directories(qn: Dict):
    year: int = qn["year"]
    day: int = qn["day"]
    header: str = qn["header"]
    body: str = qn["body"]
    input_data: str = qn["input_data"]

    directory = f"./{year}/questions"
    os.makedirs(directory, exist_ok=True)

    with open(f"{directory}/{day:02d}.md", "w") as f:
        print(f"Saving question for Advent of Code {year} Day {day}...")
        f.write(f"# [{header}](https://adventofcode.com/{year}/day/{day})\n\n")
        f.write(body)

    with open(f"{directory}/{day:02d}.txt", "w") as f:
        print(f"Saving input for Advent of Code {year} Day {day}...")
        f.write(input_data)


def scrape_advent_of_code(
    year: int | str, day: int, session_token: str | None = None, max_retries=3, delay=5
):
    print(f"Scraping Advent of Code {year} Day {day}...")

    headers = {"Cookie": f"session={session_token}"}

    for attempt in range(max_retries):
        try:
            url = f"https://adventofcode.com/{year}/day/{day}"
            response = requests.get(url, headers=headers, timeout=10)
            response.raise_for_status()

            soup = BeautifulSoup(response.text, "html.parser")

            day_desc = soup.find("article", class_="day-desc")
            if not day_desc:
                print(f"Couldn't find the day-desc element for day {day}")
                return None

            # Extract BODY
            body = day_desc.get_text(strip=True)

            # Extract HEADER
            header_element = day_desc.find("h2")
            if header_element:
                header = header_element.get_text(strip=True)
                header = re.sub(r"---", "", header)
            else:
                header = ""

            input_url = f"https://adventofcode.com/{year}/day/{day}/input"
            input_response = requests.get(input_url, headers=headers, timeout=10)
            input_response.raise_for_status()

            input_data = input_response.text.strip()

            return {
                "year": year,
                "day": day,
                "header": header,
                "body": body,
                "input_data": input_data,
            }

        except requests.RequestException as e:
            if e.response is not None and e.response.status_code == 404:
                print(f"No question found for day {day}")
                return None

            print(f"Attempt {attempt + 1} failed for day {day}: {str(e)}")
            if attempt < max_retries - 1:
                print(f"Retrying in {delay} seconds...")
                time.sleep(delay)
            else:
                print(f"Failed to scrape day {day} after {max_retries} attempts")
                return None


if __name__ == "__main__":
    args = sys.argv

    if len(args) < 2:
        raise Exception("Please provide the year of the advent of code")

    year = args[1]
    day = args[2] if len(args) >= 3 else ""
    load_dotenv()

    if day:
        qn = scrape_advent_of_code(
            year, int(day), session_token=os.getenv("ADVENT_OF_CODE_SESSION")
        )

        if not qn:
            raise Exception(f"Failed to scrape day {day}")

        generate_question_directories(qn)
        sys.exit(0)

    for i in range(1, 26):
        qn = scrape_advent_of_code(
            year, i, session_token=os.getenv("ADVENT_OF_CODE_SESSION")
        )

        if not qn:
            print(f"Failed to scrape day {i}.")
            break

        generate_question_directories(qn)
    sys.exit(0)
