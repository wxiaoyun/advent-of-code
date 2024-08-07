import os
from typing import Dict


def generate_question_directories(qn: Dict):
  year: int = qn["year"]
  day: int = qn["day"]
  header: str = qn["header"]
  body: str = qn["body"]
  input_data: str = qn["input_data"]

  # Create the directory
  directory = f"./{year}/questions"
  os.makedirs(directory, exist_ok=True)

  # Write the header
  with open(f"{directory}/{day:02d}.md", "w") as f:
    print(f"Saving question for Advent of Code {year} Day {day}...")
    f.write(f"# [{header}](https://adventofcode.com/{year}/day/{day})\n\n")
    f.write(body)
  
  with open(f"{directory}/{day:02d}.txt", "w") as f:
    print(f"Saving input for Advent of Code {year} Day {day}...")
    f.write(input_data)