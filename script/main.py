import os
import sys
from dotenv import load_dotenv
from scrapper import scrape_advent_of_code
from generator import generate_question_directories

if __name__ == "__main__":
    args = sys.argv

    if len(args) < 1:
        raise Exception("Please provide the year of the advent of code")

    year = args[1]
    load_dotenv()

    for i in range(1, 26):
        qn = scrape_advent_of_code(
            year, i, session_token=os.getenv("ADVENT_OF_CODE_SESSION")
        )

        if not qn:
            raise Exception(f"Failed to scrape day {i}")

        generate_question_directories(qn)
