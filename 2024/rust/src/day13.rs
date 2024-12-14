use crate::{get_input_for_day, get_test_input};

pub fn part_one() {
    fn solve(target_x: u64, target_y: u64, a_x: u64, a_y: u64, b_x: u64, b_y: u64) -> u64 {
        let mut tokens = u64::MAX;

        for i in 0..=100 {
            for j in 0..=100 {
                if a_x * i + b_x * j == target_x && a_y * i + b_y * j == target_y {
                    tokens = tokens.min(i * 3 + j);
                    break;
                }
            }
        }

        if tokens == u64::MAX {
            0
        } else {
            tokens
        }
    }

    let re = regex::Regex::new(r"[XY][+=](\d+)").unwrap();
    let tokens = get_input_for_day(13)
        .split("\n\n")
        .map(|p| {
            let mut l = p.lines();
            let mut a = l.next().unwrap().split(',');
            let mut b = l.next().unwrap().split(',');
            let mut price = l.next().unwrap().split(',');

            let a_x = re.captures(a.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let a_y = re.captures(a.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let b_x = re.captures(b.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let b_y = re.captures(b.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let target_x = re.captures(price.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let target_y = re.captures(price.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();

            solve(target_x, target_y, a_x, a_y, b_x, b_y)
        })
        .sum::<u64>();

    println!("{}", tokens);
}

pub fn part_two() {
    // Gaussian elimination
    fn solve(mut mat: [[f64; 3]; 2], margin_of_error: f64) -> u64 {
        for j in 0..2 {
            let k = mat[j][j];
            for i in 0..3 {
                mat[j][i] /= k;
            }

            for i in 0..2 {
                if i != j {
                    let k = mat[i][j];
                    for l in 0..3 {
                        mat[i][l] -= k * mat[j][l];
                    }
                }
            }
        }

        let mut tokens = 0;
        let token_cost = [3_u64, 1];
        for i in 0..2 {
            let rounded = mat[i][2].round();
            if (mat[i][2] - rounded).abs() > margin_of_error {
                return 0;
            }
            tokens += rounded as u64 * token_cost[i];
        }
        tokens
    }

    let re = regex::Regex::new(r"[XY][+=](\d+)").unwrap();
    let tokens = get_input_for_day(13)
        .split("\n\n")
        .map(|p| {
            let mut l = p.lines();
            let mut a = l.next().unwrap().split(',');
            let mut b = l.next().unwrap().split(',');
            let mut price = l.next().unwrap().split(',');

            let a_x = re.captures(a.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let a_y = re.captures(a.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let b_x = re.captures(b.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let b_y = re.captures(b.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let target_x = re.captures(price.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();
            let target_y = re.captures(price.next().unwrap()).unwrap()[1]
                .parse::<u64>()
                .unwrap();

            [
                [
                    a_x as f64,
                    b_x as f64,
                    10000000000000_f64 + (target_x as f64),
                ],
                [
                    a_y as f64,
                    b_y as f64,
                    10000000000000_f64 + (target_y as f64),
                ],
            ]
        })
        .map(|mut m| solve(m, 0.01))
        .sum::<u64>();

    println!("{}", tokens);
}
