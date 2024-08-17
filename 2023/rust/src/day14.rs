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
