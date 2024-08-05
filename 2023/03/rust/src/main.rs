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
    let input = std::fs::read_to_string("../input.txt")?;

    let val_mat: Vec<Vec<Value>> = input
        .lines()
        .map(|line| line.chars().map(Value::from).collect())
        .collect();
    let mut skip_mat: Vec<Vec<bool>> = val_mat
        .iter()
        .map(|line| line.iter().map(|v| *v == Value::Period).collect())
        .collect();

    println!("{:?}", val_mat);

    let mut sum = 0;
    for (r, row) in val_mat.iter().enumerate() {
        for (c, val) in row.iter().enumerate() {
            if skip_mat[r][c] {
                println!(":{r}:{c} skipped {:?}", val_mat[r][c]);
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
                println!(":{r}:{i} {:?}", val_mat[r][i]);

                if !is_symbol_arround && check_symbol_arround(&val_mat, r, i) {
                    is_symbol_arround = true;
                }
            }

            if is_symbol_arround {
                println!("Added number {tmp_builder}");
                sum += tmp_builder;
            }
        }
    }

    println!("Sum of engine schematic {:?}", sum);

    Ok(())
}

fn check_symbol_arround(val_mat: &[Vec<Value>], r: usize, c: usize) -> bool {
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

        if let Value::Symbol(_) = val_mat[new_r as usize][new_c as usize] {
            println!(
                ":{r}:{c} got symbol arround at :{new_r}:{new_c} {:?}",
                val_mat[new_r as usize][new_c as usize]
            );
            return true;
        }
    }

    false
}
