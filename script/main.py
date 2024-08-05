from scrapper import scrape_advent_of_code
from generator import generate_question_directories

if __name__ == "__main__":
    for i in range(1, 26):
        qn = scrape_advent_of_code(2023, i)

        if not qn:
          raise Exception(f"Failed to scrape day {i}")
        
        generate_question_directories(qn)