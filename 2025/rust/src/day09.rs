use std::collections::VecDeque;

fn parse_input(input: impl AsRef<str>) -> Vec<Vec<i64>> {
    input
        .as_ref()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<i64>())
                .collect::<Result<_, _>>()
        })
        .collect::<Result<_, _>>()
        .unwrap()
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let coords = parse_input(input);
    let n = coords.len();

    let mut largest_area = 0;
    for i in 0..n {
        let (ir, ic) = (coords[i][0], coords[i][1]);
        for j in (i + 1)..n {
            let (jr, jc) = (coords[j][0], coords[j][1]);
            largest_area = largest_area.max(((ir - jr).abs() + 1) * ((ic - jc).abs() + 1))
        }
    }

    largest_area
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let coords = parse_input(input);
    let n = coords.len();
    let nrow = coords.iter().map(|coord| coord[0]).max().unwrap() as usize + 2;
    let ncol = coords.iter().map(|coord| coord[1]).max().unwrap() as usize + 2;

    let mut mat = vec![vec![0i8; ncol]; nrow];
    for i in 0..n {
        let j = (i + 1) % n;
        let (r1, c1) = (coords[i][0] as usize, coords[i][1] as usize);
        let (r2, c2) = (coords[j][0] as usize, coords[j][1] as usize);

        let r_min = r1.min(r2);
        let r_max = r1.max(r2);
        let c_min = c1.min(c2);
        let c_max = c1.max(c2);

        for r in r_min..=r_max {
            for c in c_min..=c_max {
                mat[r][c] = 1;
            }
        }
    }

    let mut queue = VecDeque::new();
    if mat[0][0] == 0 {
        mat[0][0] = 2;
        queue.push_back((0, 0));
    }

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    while let Some((r, c)) = queue.pop_front() {
        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;

            if nr >= 0 && nr < nrow as isize && nc >= 0 && nc < ncol as isize {
                let nr = nr as usize;
                let nc = nc as usize;
                if mat[nr][nc] == 0 {
                    mat[nr][nc] = 2;
                    queue.push_back((nr, nc));
                }
            }
        }
    }

    let mut largest_area = 0;
    for i in 0..n {
        let (ir, ic) = (coords[i][0], coords[i][1]);
        'outer: for j in (i + 1)..n {
            let (jr, jc) = (coords[j][0], coords[j][1]);
            let (r_top, r_bot) = (ir.min(jr), ir.max(jr));
            let (c_left, c_right) = (ic.min(jc), ic.max(jc));

            let current_area = (r_bot - r_top + 1) * (c_right - c_left + 1);
            if current_area <= largest_area {
                continue;
            }

            // Check all four sizes is red or green
            for r in r_top..(r_bot + 1) {
                for c in [c_left, c_right] {
                    if mat[r as usize][c as usize] == 2 {
                        continue 'outer;
                    }
                }
            }

            for c in c_left..(c_right + 1) {
                for r in [r_top, r_bot] {
                    if mat[r as usize][c as usize] == 2 {
                        continue 'outer;
                    }
                }
            }

            largest_area = current_area
        }
    }

    largest_area
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        7,1
        11,1
        11,7
        9,7
        9,5
        2,5
        2,3
        7,3
    "};

    #[test]
    fn test_part1() {
        assert_eq!(50, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(24, super::part_two(TEST_INPUT));
    }
}
