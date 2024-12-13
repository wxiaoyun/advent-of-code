use std::{collections, iter};

use crate::{get_input_for_day, get_test_input};

pub fn part_one() {
    let mut mat = get_input_for_day(12)
        .lines()
        .map(|l| l.chars().map(|c| c.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    fn flood_mut(
        mat: &mut Vec<Vec<String>>,
        i: usize,
        j: usize,
        target: String,
        replace: String,
    ) -> u64 {
        let mut area = 0;

        let mut stack = vec![(i, j)];
        while let Some((ii, jj)) = stack.pop() {
            if mat[ii][jj] != target {
                continue;
            }

            area += 1;
            mat[ii][jj].clone_from(&replace);

            if ii > 0 {
                stack.push((ii - 1, jj));
            }
            if ii < mat.len() - 1 {
                stack.push((ii + 1, jj));
            }
            if jj > 0 {
                stack.push((ii, jj - 1));
            }
            if jj < mat[0].len() - 1 {
                stack.push((ii, jj + 1));
            }
        }

        area
    }

    fn calc_perimeter(mat: &Vec<Vec<String>>, c: String) -> u64 {
        let mut perimeter = 0;
        let rows = mat.len();
        let cols = mat[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if mat[i][j] == c {
                    // Check all four sides
                    if i == 0 || mat[i - 1][j] != c {
                        // Top
                        perimeter += 1;
                    }
                    if i == rows - 1 || mat[i + 1][j] != c {
                        // Bottom
                        perimeter += 1;
                    }
                    if j == 0 || mat[i][j - 1] != c {
                        // Left
                        perimeter += 1;
                    }
                    if j == cols - 1 || mat[i][j + 1] != c {
                        // Right
                        perimeter += 1;
                    }
                }
            }
        }

        perimeter
    }

    let mut count: collections::HashMap<String, u64> = collections::HashMap::new();
    let mut areas: collections::HashMap<String, u64> = collections::HashMap::new();

    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            let s = &mat[i][j].clone();
            if areas.contains_key(s) {
                continue;
            }

            let cnt = *count.get(s).unwrap_or(&0);
            count.insert(s.clone(), cnt + 1);
            let replace = format!("{}|{}", s, cnt);
            areas.insert(
                replace.clone(),
                flood_mut(&mut mat, i, j, s.clone(), replace),
            );
        }
    }

    let price = areas
        .into_iter()
        .map(|(c, area)| calc_perimeter(&mat, c.clone()) * area)
        .sum::<u64>();

    println!("{}", price);
}

const BORDER: char = '.';

pub fn part_two() {
    let wrapped = get_input_for_day(12)
        .lines()
        .map(|l| {
            let mut v = vec![BORDER];
            v.extend(l.chars());
            v.push(BORDER);
            v
        })
        .collect::<Vec<_>>();

    let border_row = iter::repeat(BORDER)
        .take(wrapped[0].len())
        .collect::<Vec<_>>();
    let mut mat = vec![border_row.clone()];
    mat.extend(wrapped);
    mat.push(border_row);

    // Assumes i, j is not a border index
    // Tip from reddit: the number of corners == number of edges
    fn count_corners(mat: &Vec<Vec<char>>, i: usize, j: usize, target: char) -> u64 {
        if mat[i][j] != target {
            return 0;
        }

        let mut corners = 0;
        // protruding corner: two immediate neighbors are not target
        // top left
        if mat[i - 1][j] != target && mat[i][j - 1] != target {
            corners += 1;
        }

        // top right
        if mat[i - 1][j] != target && mat[i][j + 1] != target {
            corners += 1;
        }

        // bottom left
        if mat[i + 1][j] != target && mat[i][j - 1] != target {
            corners += 1;
        }

        // bottom right
        if mat[i + 1][j] != target && mat[i][j + 1] != target {
            corners += 1;
        }

        // concave corner: two immediate neighbors are target, but diagonal is not
        // top left
        if mat[i - 1][j] == target && mat[i][j - 1] == target && mat[i - 1][j - 1] != target {
            corners += 1;
        }

        // top right
        if mat[i - 1][j] == target && mat[i][j + 1] == target && mat[i - 1][j + 1] != target {
            corners += 1;
        }

        // bottom left
        if mat[i + 1][j] == target && mat[i][j - 1] == target && mat[i + 1][j - 1] != target {
            corners += 1;
        }

        // bottom right
        if mat[i + 1][j] == target && mat[i][j + 1] == target && mat[i + 1][j + 1] != target {
            corners += 1;
        }

        corners
    }

    fn flood(
        mat: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
        target: char,
        i: usize,
        j: usize,
    ) -> u64 {
        if visited[i][j] {
            return 0;
        }

        let mut area = 0;
        let mut corners = 0;
        let mut stack = vec![(i, j)];
        while let Some((ii, jj)) = stack.pop() {
            if visited[ii][jj] {
                continue;
            }

            area += 1;
            corners += count_corners(mat, ii, jj, target);
            visited[ii][jj] = true;

            if mat[ii - 1][jj] == target {
                stack.push((ii - 1, jj))
            }

            if mat[ii + 1][jj] == target {
                stack.push((ii + 1, jj))
            }

            if mat[ii][jj - 1] == target {
                stack.push((ii, jj - 1));
            }

            if mat[ii][jj + 1] == target {
                stack.push((ii, jj + 1));
            }
        }

        area * corners
    }

    let mut price = 0;
    let mut visited = mat
        .iter()
        .map(|r| r.iter().map(|_| false).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // Skip border
    for i in 1..mat.len() - 1 {
        for j in 1..mat[0].len() - 1 {
            let target = mat[i][j];
            price += flood(&mat, &mut visited, target, i, j);
        }
    }

    println!("{}", price);
}
