use std::collections::HashSet;

const UP: (i64, i64) = (-1, 0);
const DOWN: (i64, i64) = (1, 0);
const LEFT: (i64, i64) = (0, -1);
const RIGHT: (i64, i64) = (0, 1);

const EMPTY: char = '.';
const MIRROR1: char = '\\';
const MIRROR2: char = '/';
const SPLITTER1: char = '-';
const SPLITTER2: char = '|';

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let grid = parse_input(input);
    let mut beams = vec![(0, 0, RIGHT)];

    let mut visited = HashSet::new();
    while let Some(state) = beams.pop() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());

        let (r, c, dir) = state;
        let new_beams = step(&grid, r, c, dir);
        beams.extend(new_beams);
    }

    visited
        .into_iter()
        .map(|(r, c, _)| (r, c))
        .collect::<HashSet<_>>()
        .len() as i64
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    0
}

fn parse_input(input: impl AsRef<str>) -> Vec<Vec<char>> {
    input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn step(
    grid: &Vec<Vec<char>>,
    r: usize,
    c: usize,
    dir: (i64, i64),
) -> Vec<(usize, usize, (i64, i64))> {
    let mut beams = vec![];

    match (dir, grid[r][c]) {
        (_, EMPTY) => beams.push((r, c, dir)),

        (RIGHT, MIRROR1) => beams.push((r, c, DOWN)),
        (UP, MIRROR1) => beams.push((r, c, LEFT)),
        (LEFT, MIRROR1) => beams.push((r, c, UP)),
        (DOWN, MIRROR1) => beams.push((r, c, RIGHT)),

        (RIGHT, MIRROR2) => beams.push((r, c, UP)),
        (DOWN, MIRROR2) => beams.push((r, c, LEFT)),
        (LEFT, MIRROR2) => beams.push((r, c, DOWN)),
        (UP, MIRROR2) => beams.push((r, c, RIGHT)),

        (UP | DOWN, SPLITTER1) => {
            beams.push((r, c, LEFT));
            beams.push((r, c, RIGHT));
        }
        (_, SPLITTER1) => beams.push((r, c, dir)),

        (RIGHT | LEFT, SPLITTER2) => {
            beams.push((r, c, UP));
            beams.push((r, c, DOWN));
        }
        (_, SPLITTER2) => beams.push((r, c, dir)),
        _ => unreachable!(),
    }

    beams
        .into_iter()
        .map(|(r, c, (dr, dc))| (r as i64 + dr, c as i64 + dc, (dr, dc)))
        .filter(|&(r, c, dir)| r.min(c) >= 0 && r < grid.len() as i64 && c < grid[0].len() as i64)
        .map(|(r, c, dir)| (r as usize, c as usize, dir))
        .collect()
}
