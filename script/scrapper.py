import requests
from bs4 import BeautifulSoup
import re

def scrape_advent_of_code(year:int, day: int):
    url = f"https://adventofcode.com/{year}/day/{day}"
    response = requests.get(url)
    
    if response.status_code != 200:
        print(f"Failed to retrieve the page for day {day}")
        return None
    
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
        header = re.sub(r'---', '', header)  # Remove "---"
    else:
        header = ""
      
    # input_url = f"https://adventofcode.com/{year}/day/{day}/input"
    # response = requests.get(input_url)
    # if response.status_code != 200:
    #     print(f"Failed to retrieve the input for day {day}")
    #     return None
    
    # input = response.text.strip()
    
    return {
        "year": year,
        "day": day,
        "header": header,
        "body": body,
    }
