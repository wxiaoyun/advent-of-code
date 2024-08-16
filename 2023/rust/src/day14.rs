use crate::{get_input_for_day, get_test_input, Result};

pub fn part_one() -> Result {
    let mut mat = get_input_for_day(14)
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // Stores the row index of the closest '#' above the current cell
    let mut prefix_sum = mat[0]
        .iter()
        .map(|_| vec![-1; mat.len()])
        .collect::<Vec<_>>();

    for (i, row) in mat.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == '#' {
                prefix_sum[i][j] = i as i64;
            }

            if i > 0 {
                prefix_sum[i][j] = prefix_sum[i][j].max(prefix_sum[i - 1][j]);
            }
        }
    }

    let rows = mat.len();
    let cols = mat[0].len();

    for i in 0..rows {
        for j in 0..cols {
            let c = mat[i][j];
            if c != 'O' {
                continue;
            }

            let hash_row = prefix_sum[i][j];
            let new_row = (hash_row + 1) as usize;

            // Swap the current cell with the new cell
            mat[i][j] = '.';
            mat[new_row][j] = 'O';

            // Update the prefix sum
            for k in new_row..mat.len() {
                if mat[k][j] == '#' {
                    break;
                }
                prefix_sum[k][j] = new_row as i64;
            }
        }
    }

    let res = mat
        .into_iter()
        .enumerate()
        .map(|(r, row)| {
            row.into_iter().fold(0, |acc, c| {
                if c == 'O' {
                    acc + (rows - r) as u64
                } else {
                    acc
                }
            })
        })
        .sum::<u64>();

    println!("Part one: {}", res);

    Ok(())
}
