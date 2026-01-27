pub fn part_one(input: impl AsRef<str>) -> i64 {
    let mut buf = Vec::with_capacity(2);
    input
        .as_ref()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<_>>()
        })
        .map(|digits| largest_n_digit_number(2, digits, &mut buf))
        .sum()
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let mut buf = Vec::with_capacity(12);
    input
        .as_ref()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<_>>()
        })
        .map(|digits| largest_n_digit_number(12, digits, &mut buf))
        .sum()
}

pub fn largest_n_digit_number(n: usize, digits: impl AsRef<[i8]>, buf: &mut Vec<i8>) -> i64 {
    fn include_digit(builder: &mut [i8], digit: i8, i: usize) {
        if i >= builder.len() || digit < builder[i] {
            return;
        }

        let msd = builder[i];
        builder[i] = digit;
        include_digit(builder, msd, i + 1);
    }

    fn build_digit(builder: &[i8]) -> i64 {
        let mut i = 0;
        let mut acc = 0i64;
        while i < builder.len() {
            acc = acc * 10 + builder[i] as i64;
            i += 1;
        }
        acc
    }

    buf.resize(n, -1);
    buf.fill(-1);
    digits
        .as_ref()
        .iter()
        .rev()
        .copied()
        .enumerate()
        .fold(-1, |acc, (i, digit)| {
            if i < n {
                buf[n - 1 - i] = digit;
            } else {
                include_digit(buf, digit, 0);
            }
            acc.max(build_digit(buf))
        })
}
