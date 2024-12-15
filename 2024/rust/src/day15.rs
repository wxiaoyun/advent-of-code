use std::{collections::HashSet, f64::consts::E, vec};

use crate::{get_input_for_day, get_test_input};

const WALL: char = '#';
const BOX: char = 'O';
const EMPTY: char = '.';
const ROBOT: char = '@';

pub fn part_one() {
    let input = get_input_for_day(15);
    let mut input = input.split("\n\n");

    let mut i = 0;
    let mut j = 0;
    let mut mat = input
        .next()
        .unwrap()
        .lines()
        .enumerate()
        .map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(|(c, ch)| {
                    if ch == ROBOT {
                        i = r;
                        j = c;
                    }
                    ch
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let operations = input
        .next()
        .unwrap()
        .lines()
        .flat_map(|l| l.chars())
        .collect::<Vec<_>>();

    fn operate(mat: &mut Vec<Vec<char>>, op: char, i: &mut usize, j: &mut usize) {
        let (dr, dc) = match op {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!("Invalid operation"),
        };

        fn helper(mat: &mut Vec<Vec<char>>, i: usize, j: usize, dr: i32, dc: i32) {
            if mat[i][j] == EMPTY {
                return;
            }

            let (ii, jj) = ((i as i32 + dr) as usize, (j as i32 + dc) as usize);
            if mat[ii][jj] == WALL {
                return;
            }

            helper(mat, ii, jj, dr, dc);

            if mat[ii][jj] == EMPTY {
                let tmp = mat[i][j];
                mat[i][j] = mat[ii][jj];
                mat[ii][jj] = tmp;
            }
        }

        helper(mat, *i, *j, dr, dc);

        if mat[*i][*j] != ROBOT {
            *i = (*i as i32 + dr) as usize;
            *j = (*j as i32 + dc) as usize;
        }
    }

    for op in operations {
        operate(&mut mat, op, &mut i, &mut j);
    }

    let mut gps_sum = 0;
    for (r, row) in mat.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == BOX {
                gps_sum += (r * 100) + c;
            }
        }
    }

    println!("{}", gps_sum);
}

const BOX_L: char = '[';
const BOX_R: char = ']';

pub fn part_two() {
    let input = get_input_for_day(15);
    let mut input = input.split("\n\n");

    let mut i = 0;
    let mut j = 0;
    let mut mat = input
        .next()
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    BOX => vec![BOX_L, BOX_R],
                    ROBOT => vec![ROBOT, EMPTY],
                    _ => vec![c, c],
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    print_mat(&mat);

    for (r, row) in mat.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == ROBOT {
                i = r;
                j = c;
            }
        }
    }

    let operations = input
        .next()
        .unwrap()
        .lines()
        .flat_map(|l| l.chars())
        .collect::<Vec<_>>();

    fn operate(mat: &mut Vec<Vec<char>>, op: char, i: usize, j: usize) -> (usize, usize) {
        if mat[i][j] != ROBOT {
            panic!("Invalid robot position");
        }

        let (dr, dc) = match op {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => panic!("Invalid operation"),
        };

        fn swap(mat: &mut Vec<Vec<char>>, i: usize, j: usize, dr: i32, dc: i32) {
            let (ii, jj) = ((i as i32 + dr) as usize, (j as i32 + dc) as usize);
            let tmp = mat[i][j];
            mat[i][j] = mat[ii][jj];
            mat[ii][jj] = tmp;
        }

        fn get_next(mat: &Vec<Vec<char>>, i: usize, j: usize, dr: i32) -> HashSet<(usize, usize)> {
            let mut l = HashSet::new();
            if mat[i][j] == EMPTY {
                return l;
            }

            l.insert((i, j));
            if dr == 0 {
                return l;
            }

            if mat[i][j] == BOX_L {
                l.insert((i, j + 1));
            } else if mat[i][j] == BOX_R {
                l.insert((i, j - 1));
            }
            l
        }

        let mut levels = vec![get_next(mat, i, j, dr)];

        loop {
            let prev = levels.last().unwrap();
            let mut next_levels = HashSet::new();

            for &(pi, pj) in prev {
                let (ii, jj) = ((pi as i32 + dr) as usize, (pj as i32 + dc) as usize);
                if mat[ii][jj] == WALL {
                    return (i, j); // Cannot push
                }

                if mat[ii][jj] == EMPTY {
                    continue;
                }

                next_levels.extend(get_next(mat, ii, jj, dr));
            }

            if next_levels.is_empty() {
                break;
            }

            levels.push(next_levels);
        }

        for level in levels.iter().rev() {
            for &(pi, pj) in level {
                swap(mat, pi, pj, dr, dc);
            }
        }

        ((i as i32 + dr) as usize, (j as i32 + dc) as usize)
    }

    for op in operations {
        let res = operate(&mut mat, op, i, j);
        i = res.0;
        j = res.1;
    }

    let mut gps_sum = 0;
    for (r, row) in mat.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == BOX_L {
                gps_sum += (r * 100) + c;
            }
        }
    }

    println!("{}", gps_sum);
}

fn print_mat(mat: &Vec<Vec<char>>) {
    for row in mat.iter() {
        for ch in row.iter() {
            print!("{}", ch);
        }
        println!();
    }
}
