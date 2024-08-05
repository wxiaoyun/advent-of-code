#[derive(PartialEq, Debug, Clone)]
enum Value {
    Symbol(char),
    Number(u32),
    Period,
}

impl From<char> for Value {
    fn from(s: char) -> Self {
        match s {
            '.' => Value::Period,
            c if c.is_ascii_digit() => Value::Number(c.to_digit(10).unwrap()),
            c => Value::Symbol(c),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part_one()?;
    part_two()?;

    Ok(())
}

fn part_one() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("../input.txt")?;

    let val_mat: Vec<Vec<Value>> = input
        .lines()
        .map(|line| line.chars().map(Value::from).collect())
        .collect();
    let mut skip_mat: Vec<Vec<bool>> = val_mat
        .iter()
        .map(|line| line.iter().map(|v| *v == Value::Period).collect())
        .collect();

    let mut sum = 0;
    for (r, row) in val_mat.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            if skip_mat[r][c] {
                continue;
            }

            let Value::Number(_) = val else { continue };

            let mut tmp_builder = 0;
            let mut is_symbol_arround = false;

            for i in c..row.len() {
                tmp_builder = tmp_builder * 10
                    + match val_mat[r][i] {
                        Value::Number(n) => {
                            skip_mat[r][i] = true;
                            n
                        }
                        _ => break, // break if not a number
                    };

                if !is_symbol_arround
                    && check_arround(&val_mat, r, i, |v| matches!(v, Value::Symbol(_)))
                {
                    is_symbol_arround = true;
                }
            }

            if is_symbol_arround {
                sum += tmp_builder;
            }
        }
    }

    println!("(Part one) Sum of engine schematic {:?}", sum);

    Ok(())
}

fn check_arround<F>(val_mat: &[Vec<Value>], r: usize, c: usize, pred: F) -> bool
where
    F: Fn(&Value) -> bool,
{
    let deltas = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in deltas {
        let new_r = r as i32 + dx;
        let new_c = c as i32 + dy;

        if new_r < 0 || new_r >= val_mat.len() as i32 {
            continue;
        }

        if new_c < 0 || new_c >= val_mat[new_r as usize].len() as i32 {
            continue;
        }

        if pred(&val_mat[new_r as usize][new_c as usize]) {
            println!(
                ":{r}:{c} found matching pred at :{new_r}:{new_c} {:?}",
                val_mat[new_r as usize][new_c as usize]
            );
            return true;
        }
    }

    false
}

fn part_two() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("../input.txt")?;

    let val_mat: Vec<Vec<Value>> = input
        .lines()
        .map(|line| line.chars().map(Value::from).collect())
        .collect();

    let mut sum = 0;
    for (r, row) in val_mat.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            let Value::Symbol(s) = val_mat[r][c] else {
                continue;
            };
            let (n1, n2) = find_two_numbers(&val_mat, r, c).unwrap_or((0, 0));
            if n1 == 0 || n2 == 0 {
                continue;
            }
            println!("Found {} at :{}:{} n1: {:?} n2: {:?}", s, r, c, n1, n2);
            sum += n1 * n2;
        }
    }

    println!("(Part two) Sum of engine schematic {:?}", sum);

    Ok(())
}

fn find_two_numbers(val_mat: &[Vec<Value>], r: usize, c: usize) -> Option<(u32, u32)> {
    let deltas = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut nums = std::collections::HashSet::new();

    for (dx, dy) in deltas {
        let new_r = r as i32 + dx;
        let new_c = c as i32 + dy;

        if new_r < 0 || new_r >= val_mat.len() as i32 {
            continue;
        }

        if new_c < 0 || new_c >= val_mat[new_r as usize].len() as i32 {
            continue;
        }

        if let Value::Number(_) = val_mat[new_r as usize][new_c as usize] {
            nums.insert(find_number(val_mat, new_r as usize, new_c as usize));
        }
    }

    if nums.len() == 2 {
        let mut nums = nums.into_iter();
        return Some((nums.next().unwrap(), nums.next().unwrap()));
    }

    None
}

fn find_number(val_mat: &[Vec<Value>], r: usize, c: usize) -> u32 {
    let mut c = c;

    // keep moving left until its no longer a number
    while c > 0 {
        if let Value::Number(_) = val_mat[r][c - 1] {
            c -= 1;
        } else {
            break;
        }
    }

    let mut tmp_builder = 0;
    while c < val_mat[r].len() {
        if let Value::Number(n) = val_mat[r][c] {
            tmp_builder = tmp_builder * 10 + n;
            c += 1;
        } else {
            break;
        }
    }

    tmp_builder
}
