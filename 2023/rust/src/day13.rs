use crate::{get_input_for_day, get_test_input, Result};

pub fn part_one() -> Result {
    let res = get_input_for_day(13)
        .split("\n\n")
        .map(|chunk| chunk.trim())
        .filter(|chunk| !chunk.is_empty())
        .map(|chunk| {
            chunk
                .lines()
                .map(|l| l.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .map(summarize)
        .sum::<u64>();

    println!("Day 13 Part 1: {}", res);

    Ok(())
}

fn summarize(mat: Vec<Vec<char>>) -> u64 {
    fn is_symmetric_vertically(mat: &Vec<Vec<char>>, l: i64, r: i64) -> bool {
        if l.min(r) < 0 || l.max(r) as usize >= mat[0].len() {
            return true; // vacuously true
        }

        let l = l as usize;
        let r = r as usize;

        if mat.iter().all(|row| row[l] == row[r]) {
            is_symmetric_vertically(mat, l as i64 - 1, r as i64 + 1)
        } else {
            false
        }
    }

    fn is_symmetric_horizontally(mat: &Vec<Vec<char>>, t: i64, b: i64) -> bool {
        if t.min(b) < 0 || t.max(b) as usize >= mat.len() {
            return true; // vacuously true
        }

        let t = t as usize;
        let b = b as usize;

        if mat[t].iter().zip(mat[b].iter()).all(|(&a, &b)| a == b) {
            is_symmetric_horizontally(mat, t as i64 - 1, b as i64 + 1)
        } else {
            false
        }
    }

    let mut summary = 0;
    for row in 1..mat.len() {
        if is_symmetric_horizontally(&mat, row as i64 - 1, row as i64) {
            summary += (row as u64) * 100;
            break;
        }
    }
    for col in 1..mat[0].len() {
        if is_symmetric_vertically(&mat, col as i64 - 1, col as i64) {
            summary += col as u64;
            break;
        }
    }

    summary
}
