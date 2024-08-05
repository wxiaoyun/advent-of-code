import os
from typing import Dict


def generate_question_directories(qn: Dict):
  year: int = qn["year"]
  day: int = qn["day"]
  header: str = qn["header"]
  body: str = qn["body"]

  # Create the directory
  directory = f"./{year}/{day:02d}"
  os.makedirs(directory, exist_ok=True)

  # Write the header
  with open(f"{directory}/README.md", "w") as f:
    f.write(f"# [{header}](https://adventofcode.com/{year}/day/{day})\n\n")
    f.write(body)