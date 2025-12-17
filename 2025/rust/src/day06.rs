use std::str::FromStr;

#[derive(Clone, Copy)]
enum Op {
    Add,
    Mul,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => return Err(()),
        };
        Ok(op)
    }
}

impl Op {
    fn operate(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
        }
    }

    const fn identity(&self) -> i64 {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }
}

fn parse_input(input: impl AsRef<str>) -> (Vec<Vec<i64>>, Vec<Op>) {
    let mut mat: Vec<Vec<_>> = input
        .as_ref()
        .lines()
        .map(|l| l.split_ascii_whitespace().collect())
        .collect();

    let operands: Vec<Vec<_>> = mat
        .iter()
        .take(mat.len() - 1)
        .map(|row| {
            row.iter()
                .map(|op| op.parse::<i64>().ok())
                .collect::<Option<_>>()
        })
        .collect::<Option<_>>()
        .unwrap();

    let operators: Vec<_> = mat
        .pop()
        .and_then(|op| {
            op.into_iter()
                .map(|s| s.parse::<Op>().ok())
                .collect::<Option<_>>()
        })
        .unwrap();

    (operands, operators)
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let (operands, operators) = parse_input(input);

    operators.iter().enumerate().fold(0, |acc, (i, op)| {
        acc + operands
            .iter()
            .fold(op.identity(), |acc, operand| op.operate(acc, operand[i]))
    })
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let mut mat: Vec<Vec<_>> = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let m = mat.iter().map(|v| v.len()).max().unwrap();
    mat.iter_mut().for_each(|v| v.resize(m, ' '));

    let op_indices: Vec<_> = mat
        .pop()
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i, ch)| {
            let op = match ch {
                '+' => Op::Add,
                '*' => Op::Mul,
                _ => return None,
            };

            Some((i, op))
        })
        .collect();

    let mut result = 0;
    for (i, &(idx, op)) in op_indices.iter().enumerate() {
        let start_idx = idx;
        let end_idx = if i + 1 < op_indices.len() {
            op_indices[i + 1].0 - 1
        } else {
            m
        };

        let mut operands = Vec::with_capacity(4);
        for row in mat.iter() {
            for (j, &ch) in row[start_idx..end_idx].iter().enumerate() {
                if j >= operands.len() {
                    operands.resize(j + 1, 0);
                }
                if ch.is_numeric() {
                    operands[j] = operands[j] * 10 + (ch as i64 - b'0' as i64);
                }
            }
        }

        result += operands
            .into_iter()
            .fold(op.identity(), |acc, operand| op.operate(acc, operand))
    }

    result
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("../../../input/2025_06_1.test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(4277556, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(3263827, super::part_two(TEST_INPUT));
    }
}
