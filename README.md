# Advent of Code Solutions

This repository contains my solutions to [Advent of Code](https://adventofcode.com/) puzzles, implemented in multiple programming languages (Rust, Zig, OCaml, Go, TypeScript).

## Prerequisites

This project uses [mise](https://mise.jdx.dev/) (formerly rtx) for tool version management and task automation.
Refer to the [mise documentation](https://mise.jdx.dev/installing-mise.html) for installation and usage instructions.

### Initial Setup

1. **Clone the repository:**
   ```bash
   git clone https://github.com/wxiaoyun/advent-of-code.git aoc
   cd aoc
   ```

2. **Install tools and dependencies:**
   ```bash
   mise install
   ```

3. **Set up Advent of Code session token:**

   Create a `.env` file in the root directory:
   ```bash
   echo "ADVENT_OF_CODE_SESSION=your_session_token_here" > .env
   ```

   To get your session token:
   - Log in to [adventofcode.com](https://adventofcode.com/)
   - Open browser developer tools (F12)
   - Go to Application/Storage → Cookies
   - Copy the value of the `session` cookie

## Repository Structure

```
.
├── mise.toml              # mise configuration (tools, tasks, env vars)
├── .env                   # Environment variables (gitignored)
├── input/                 # Downloaded puzzle inputs (gitignored)
│   ├── 2024_01.txt
│   ├── 2024_01.test_1.txt
│   └── ...
├── <year>/                # Solutions for <year>
│   ├── <language_1>/
│   └── <language_2>/
├── crate/                 # Shared Rust crates
│   ├── aoc-scraper/       # Input scraper tool
│   ├── util/              # Utility library
│   └── util-macro/        # Utility macros
├── ocaml/                 # Shared OCaml utilities
└── zig/                   # Shared Zig utilities
```

## Using the Mise Task Runner

This repository provides two main tasks via mise:

### 1. Running Solutions (`mise run aoc`)

Run your Advent of Code solutions with automatic input handling.

**Basic Usage:**
```bash
# Example: Run day 15, part 1 of 2024, using test input 1, fast mode
mise run aoc rust --year 2024 --day 15 --part 1 --fast --test 1

# You can also pipe input to the command
cat input/2025_03.test_1.txt | mise run aoc ocaml -y 2025 -d 3 -p 1
```

### 2. Scraping Inputs (`mise run scrape`)

Download puzzle inputs from Advent of Code. You don't need to run this task manually, it will be run automatically when you run the `aoc` task.

**Basic Usage:**
```bash
# Example: Scrape day 15 of 2024
mise run scrape --year 2024 --day 15
```

## License

MIT License - see [LICENSE](LICENSE) file for details.
