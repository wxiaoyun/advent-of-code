use std::collections::HashSet;

const PATH: char = '.';
const FOREST: char = '#';

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let grid = parse_input(input);
    let (m, n) = (grid.len(), grid[0].len());
    let mut max_steps = 0;

    let mut stack = vec![(0, 0, 1, 0, 0)];
    while let Some(a) = stack.pop() {
        let (dist, r, c, fr, fc) = a;

        if (r, c) == (m - 1, n - 2) {
            max_steps = max_steps.max(dist);
            continue;
        }

        let mut nexts = Vec::with_capacity(4);
        match grid[r][c] {
            PATH => {
                if (r, c) == (0, 1) {
                    stack.push((dist + 1, r + 1, c, r, c))
                } else {
                    [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                        .into_iter()
                        .for_each(|coord| nexts.push(coord))
                }
            }
            FOREST => continue,
            '>' => nexts.push((r, c + 1)),
            '<' => nexts.push((r, c - 1)),
            'v' => nexts.push((r + 1, c)),
            '^' => nexts.push((r - 1, c)),
            _ => unreachable!(),
        }

        let nexts = nexts
            .into_iter()
            .filter(|&coord| coord != (fr, fc))
            .map(|coord| (dist + 1, coord.0, coord.1, r, c));
        stack.extend(nexts);
    }

    max_steps
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let grid = parse_input(input);

    fn dfs(
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        r: usize,
        c: usize,
        dist: i64,
    ) -> i64 {
        let (m, n) = (grid.len(), grid[0].len());
        if (r, c) == (m - 1, n - 2) {
            return dist;
        }

        visited[r][c] = true;

        let res = match grid[r][c] {
            FOREST => 0,
            _ => {
                if (r, c) == (0, 1) {
                    dfs(grid, visited, r + 1, c, dist + 1)
                } else {
                    let mut max_dist = 0;
                    for (rr, cc) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)] {
                        if visited[rr][cc] || grid[rr][cc] == FOREST {
                            continue;
                        }
                        max_dist = max_dist.max(dfs(grid, visited, rr, cc, dist + 1));
                    }
                    max_dist
                }
            }
        };

        visited[r][c] = false;
        res
    }

    let (m, n) = (grid.len(), grid[0].len());
    let mut visited = vec![vec![false; n]; m];
    dfs(&grid, &mut visited, 0, 1, 0)
}

fn parse_input(input: impl AsRef<str>) -> Vec<Vec<char>> {
    input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}
