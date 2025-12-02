#[derive(Debug, Clone)]
struct Machine {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    out: Vec<usize>,
    instructions: Vec<u8>,
}

impl Machine {
    fn combo_value(&self, v: u8) -> usize {
        match v {
            v if (0..=3).contains(&v) => v as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }
}

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

fn run(m: &mut Machine) -> bool {
    let inst_idx = m.pc;
    if inst_idx >= m.instructions.len() {
        return true; // Done
    }

    let opcode = m.instructions[inst_idx];
    let operand = m.instructions[inst_idx + 1];

    let mut jumped = false;
    match opcode {
        ADV => {
            let numerator = m.a as f64;
            let denominator = 2_f64.powf(m.combo_value(operand) as f64);
            let res = (numerator / denominator).trunc() as usize;
            m.a = res;
        }
        BXL => {
            let a = operand as usize;
            let b = m.b;
            let res = a ^ b;
            m.b = res;
        }
        BST => {
            let res = m.combo_value(operand) % 8;
            m.b = res;
        }
        JNZ => 'block: {
            if m.a == 0 {
                break 'block;
            }

            m.pc = operand as usize;
            jumped = true;
        }
        BXC => {
            let res = m.b ^ m.c;
            m.b = res;
        }
        OUT => {
            let res = m.combo_value(operand) % 8;
            m.out.push(res);
        }
        BDV => {
            let numerator = m.a as f64;
            let denominator = 2_f64.powf(m.combo_value(operand) as f64);
            let res = (numerator / denominator).trunc() as usize;
            m.b = res;
        }
        CDV => {
            let numerator = m.a as f64;
            let denominator = 2_f64.powf(m.combo_value(operand) as f64);
            let res = (numerator / denominator).trunc() as usize;
            m.c = res;
        }
        _ => unreachable!(),
    }

    if !jumped {
        m.pc += 2;
    }

    false
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let input = input.as_ref();
    let mut input = input.split("\n\n");

    let mut reg = input.next().unwrap().lines().map(|l| {
        l.split(':')
            .nth(1)
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap()
    });
    let instructions = input
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim()
        .split(',')
        .map(|n| n.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    let mut machine = Machine {
        a: reg.next().unwrap(),
        b: reg.next().unwrap(),
        c: reg.next().unwrap(),
        pc: 0,
        out: Vec::new(),
        instructions,
    };

    while !run(&mut machine) {}

    // Original output is a comma-separated string; encode it into a numeric value.
    let res = machine
        .out
        .iter()
        .fold(0_i64, |acc, n| acc * 10 + *n as i64);

    res
}
