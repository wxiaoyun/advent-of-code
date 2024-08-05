import time
import requests
from bs4 import BeautifulSoup
import re

def scrape_advent_of_code(year: int, day: int, session_token: str | None = None, max_retries=3, delay=5):
    print(f"Scraping Advent of Code {year} Day {day}...")

    headers = {'Cookie': f'session={session_token}'}

    for attempt in range(max_retries):
        try:
            url = f"https://adventofcode.com/{year}/day/{day}"
            response = requests.get(url, headers=headers, timeout=10)
            response.raise_for_status()
            
            soup = BeautifulSoup(response.text, 'html.parser')
            
            day_desc = soup.find('article', class_='day-desc')
            if not day_desc:
                print(f"Couldn't find the day-desc element for day {day}")
                return None
            
            # Extract BODY
            body = day_desc.get_text(strip=True)
            
            # Extract HEADER
            header_element = day_desc.find('h2')
            if header_element:
                header = header_element.get_text(strip=True)
                header = re.sub(r'---', '', header)
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
                "input_data": input_data
            }
        
        except requests.RequestException as e:
            print(f"Attempt {attempt + 1} failed for day {day}: {str(e)}")
            if attempt < max_retries - 1:
                print(f"Retrying in {delay} seconds...")
                time.sleep(delay)
            else:
                print(f"Failed to scrape day {day} after {max_retries} attempts")
                return None