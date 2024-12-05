use crate::{get_input_for_day, get_test_input, Result};

pub fn part_one() -> Result {
    let matrix = get_input_for_day(4)
        .split("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn try_find(
        matrix: &Vec<Vec<char>>,
        nrow: usize,
        ncol: usize,
        word: &str,
        direction: (i32, i32),
        r: usize,
        c: usize,
    ) -> bool {
        let mut r = r as i32;
        let mut c = c as i32;
        let (dy, dx) = direction;

        for ch in word.chars() {
            if r.min(c) < 0 || r >= nrow as i32 || c >= ncol as i32 {
                return false;
            }

            if ch != matrix[r as usize][c as usize] {
                return false;
            }

            r += dy;
            c += dx;
        }

        true
    }

    let nrow = matrix.len();
    let ncol = matrix[0].len();
    let directions = [
        (-1, 0),  // UP
        (1, 0),   // DOWN
        (0, -1),  // LEFT
        (0, 1),   // RIGHT
    ];

    let mut xmas_count = 0;
    for r in 0..nrow {
        for c in 0..ncol {
            for dir in directions {
                if try_find(&matrix, nrow, ncol, "XMAS", dir, r, c) {
                    xmas_count += 1;
                }
            }
        }
    }

    println!("{}", xmas_count);

    Ok(())
}

pub fn part_two() -> Result {
    let matrix = get_input_for_day(4)
        .split("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn try_find(matrix: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
        if matrix[r][c] != 'A' {
            return false;
        }

        let target = "MAS";
        let directions = [
          (-1, -1), // UP LEFT
          (-1, 1),  // UP RIGHT
          (1, -1),  // DOWN LEFT
          (1, 1),   // DOWN RIGHT
        ];
        let mut count = 0;

        'outer: for (dy, dx) in directions {
          let mut nr = r as i32 - dy;
          let mut nc = c as i32- dx;

          for ch in target.chars() {
            if matrix[nr as usize][nc as usize] != ch {
              continue 'outer;
            }
            nr += dy;
            nc += dx;
          }

          count += 1;
        };

        count >= 2
    }

    let nrow = matrix.len();
    let ncol = matrix[0].len();

    let mut xmas_count = 0;
    for r in 1..nrow - 1 {
        for c in 1..ncol - 1 {
            if try_find(&matrix, r, c) {
                xmas_count += 1;
            }
        }
    }

    println!("{}", xmas_count);

    Ok(())
}
