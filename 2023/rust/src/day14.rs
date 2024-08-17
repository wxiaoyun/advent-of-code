use crate::{get_input_for_day, get_test_input, Result};

pub fn part_one() -> Result {
    let mut mat = get_input_for_day(14)
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    tilt_north(&mut mat);

    println!("Part one: {}", calculate_load(&mat));

    Ok(())
}

fn tilt_north(mat: &mut [Vec<char>]) {
    fn slide_up(mat: &mut [Vec<char>], i: usize, j: usize) {
        let mut count = 0;
        let mut k = i;

        while k < mat.len() && mat[k][j] != '#' {
            if mat[k][j] == 'O' {
                count += 1;
            }
            mat[k][j] = '.';
            k += 1;
        }

        for l in i..(i + count) {
            mat[l][j] = 'O';
        }
    };

    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if i == 0 || (i > 0 && mat[i - 1][j] == '#') {
                slide_up(mat, i, j);
            }
        }
    }
}

fn calculate_load(mat: &[Vec<char>]) -> u64 {
    mat.iter()
        .enumerate()
        .map(|(r, row)| {
            row.iter().fold(0, |acc, c| {
                if *c == 'O' {
                    acc + (mat.len() - r) as u64
                } else {
                    acc
                }
            })
        })
        .sum::<u64>()
}

pub fn part_two() -> Result {
    let mut mat = get_input_for_day(14)
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = std::collections::HashMap::new();
    let mut mapping = std::collections::HashMap::new();
    let mut cycle_start = -1;
    let mut cycle_len = -1;
    for i in 0..1_000_000_000 {
        if let Some(j) = visited.get(&mat) {
            cycle_start = *j;
            cycle_len = i - *j;
            println!("Found cycle at {} with length {}", cycle_start, cycle_len);
            break;
        }
        mapping.insert(i, mat.clone());
        visited.insert(mat.clone(), i);

        for _ in 0..4 {
            tilt_north(&mut mat);
            mat = rotate_90_deg_clockwise(mat);
        }
    }

    let idx = (1_000_000_000 - cycle_start) % cycle_len;
    let mat = mapping.get(&(cycle_start + idx)).unwrap();
    println!("Part two: {}", calculate_load(mat));

    Ok(())
}

fn rotate_90_deg_clockwise(mat: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = mat.len();
    let m = mat[0].len();
    let mut new_mat = vec![vec!['.'; n]; m];

    for i in 0..n {
        for j in 0..m {
            new_mat[j][n - i - 1] = mat[i][j];
        }
    }

    new_mat
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_90_deg_clockwise() {
        let mat = vec![
            vec!['#', '#', '.'],
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
        ];
        let expected = vec![
            vec!['.', '.', '#'],
            vec!['.', '.', '#'],
            vec!['.', '.', '.'],
        ];
        assert!(rotate_90_deg_clockwise(mat) == expected);

        let mat = vec![vec!['#', '.', '.'], vec!['#', '.', '.']];
        let expected = vec![vec!['#', '#'], vec!['.', '.'], vec!['.', '.']];
        assert!(rotate_90_deg_clockwise(mat) == expected);
    }
}
