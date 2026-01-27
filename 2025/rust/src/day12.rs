use std::collections::HashMap;

use rayon::prelude::*;
use regex::Regex;

fn parse_input(input: impl AsRef<str>) -> (Vec<Vec<Vec<bool>>>, Vec<(usize, usize, Vec<usize>)>) {
    let shape_reg = Regex::new(r"\d+:\n((?:[#\.]+\n)+)").unwrap();
    let region_reg = Regex::new(r"(\d+)x(\d+):\s([\s\d]+\n)").unwrap();

    let shapes = shape_reg
        .captures_iter(input.as_ref())
        .map(|r| {
            r.get(1)
                .map(|s| {
                    s.as_str()
                        .lines()
                        .map(|l| l.chars().map(|c| c == '#').collect())
                })
                .unwrap()
                .collect()
        })
        .collect();

    let regions = region_reg
        .captures_iter(input.as_ref())
        .map(|cap| {
            let ncol = cap
                .get(1)
                .and_then(|m| m.as_str().parse::<usize>().ok())
                .unwrap();
            let nrow = cap
                .get(2)
                .and_then(|m| m.as_str().parse::<usize>().ok())
                .unwrap();
            let shape_cnt = cap
                .get(3)
                .and_then(|m| {
                    m.as_str()
                        .split_ascii_whitespace()
                        .map(|s| s.parse::<usize>().ok())
                        .collect::<Option<_>>()
                })
                .unwrap();

            (ncol, nrow, shape_cnt)
        })
        .collect();

    (shapes, regions)
}

fn rotate_once<T>(shape: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Default + Clone,
{
    let nrow = shape.len();
    let ncol = shape[0].len();

    let mut tmp = vec![vec![T::default(); nrow]; ncol];

    for i in 0..nrow {
        let tmp_col = nrow - 1 - i;
        for j in 0..ncol {
            let tmp_row = j;
            tmp[tmp_row][tmp_col] = shape[i][j].clone();
        }
    }

    tmp
}

fn try_fit(
    mat: &mut Vec<Vec<bool>>,
    shape: &Vec<Vec<bool>>,
    start_r: usize,
    start_c: usize,
    set_to: bool,
) -> bool {
    let mat_nrow = mat.len();
    let mat_ncol = mat[0].len();
    let shape_nrow = shape.len();
    let shape_ncol = shape[0].len();

    if start_r + shape_nrow > mat_nrow || start_c + shape_ncol > mat_ncol {
        return false;
    }

    let mut undo_log = vec![];
    let mut conflict = false;

    'outer: for r in 0..shape_nrow {
        let dest_r = start_r + r;
        for c in 0..shape_ncol {
            let dest_c = start_c + c;

            if !shape[r][c] {
                continue;
            }

            // shape[r][c] == true
            if mat[dest_r][dest_c] == set_to {
                conflict = true;
                break 'outer;
            }

            mat[dest_r][dest_c] = set_to;
            undo_log.push((dest_r, dest_c));
        }
    }

    if conflict {
        undo_log.into_iter().for_each(|(r, c)| {
            mat[r][c] = !set_to;
        });
    }

    !conflict
}

fn try_solve(
    mat: &mut Vec<Vec<bool>>,
    shape_rotations: &HashMap<(usize, usize), Vec<Vec<bool>>>,
    expected_shapes: &mut [usize],
    shape_idx: usize,
) -> bool {
    if shape_idx >= expected_shapes.len() {
        return true;
    }

    if expected_shapes[shape_idx] == 0 {
        return try_solve(mat, shape_rotations, expected_shapes, shape_idx + 1);
    }

    let nrow = mat.len();
    let ncol = mat[0].len();

    for r in 0..nrow {
        for c in 0..ncol {
            for rotation in 0..4 {
                let shape = shape_rotations.get(&(shape_idx, rotation)).unwrap();
                if !try_fit(mat, shape, r, c, true) {
                    continue;
                }
                expected_shapes[shape_idx] -= 1;
                if try_solve(mat, shape_rotations, expected_shapes, shape_idx) {
                    return true;
                }
                expected_shapes[shape_idx] += 1;
                try_fit(mat, shape, r, c, false);
            }
        }
    }

    false
}

enum Outcome {
    DefinitelyPossible,
    DefinitelyImpossible,
    Undetermined,
}

// Hint from reddit user u/fireymike
// Basically all input cases naively fall into definitely possible and definitely impossible cases
// Although in general, tiling problems cannot be solved naively this way.
fn naive_check(
    shape: &Vec<Vec<Vec<bool>>>,
    expected_shapes: &[usize],
    nrow: usize,
    ncol: usize,
) -> Outcome {
    let maximum_rows = nrow / 3;
    let maximum_cols = ncol / 3;
    let total_shapes: usize = expected_shapes.iter().copied().sum();

    if total_shapes <= maximum_rows * maximum_cols {
        return Outcome::DefinitelyPossible;
    }

    let mut minimum_cells = 0;
    for (idx, cnt) in expected_shapes.iter().copied().enumerate() {
        let n_occupied: usize = shape[idx]
            .iter()
            .map(|r| {
                r.iter()
                    .copied()
                    .fold(1, |acc, b| acc + if b { 1 } else { 0 })
            })
            .sum();
        minimum_cells += n_occupied * cnt;
    }

    if minimum_cells > nrow * ncol {
        return Outcome::DefinitelyImpossible;
    }

    Outcome::Undetermined
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let (shapes, regions) = parse_input(input);
    let shape_rotations: HashMap<_, _> = shapes
        .iter()
        .enumerate()
        .flat_map(|(i, shape)| {
            let mut shape_rotations = vec![((i, 0usize), shape.clone())];
            shape_rotations.reserve(3);

            for _ in 1..4 {
                let ((_, rot), shape) = shape_rotations.last().unwrap();
                let shape_rotated = rotate_once(shape);
                shape_rotations.push(((i, rot + 1), shape_rotated));
            }

            shape_rotations
        })
        .collect();

    regions
        .into_par_iter()
        .map(|(nrow, ncol, mut expected_shapes)| {
            match naive_check(&shapes, &expected_shapes, nrow, ncol) {
                Outcome::DefinitelyPossible => return 1,
                Outcome::DefinitelyImpossible => return 0,
                _ => (),
            };

            let mut mat = vec![vec![false; ncol]; nrow];
            if try_solve(&mut mat, &shape_rotations, &mut expected_shapes, 0) {
                1
            } else {
                0
            }
        })
        .sum()
}

pub fn part_two(_: impl AsRef<str>) -> i64 {
    0
}
