pub fn part_one(input: impl AsRef<str>) -> i64 {
    input
        .as_ref()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<_>>()
        })
        .map(|digits| largest_n_digit_number(2, digits))
        .sum()
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    input
        .as_ref()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as i8)
                .collect::<Vec<_>>()
        })
        .map(|digits| largest_n_digit_number(12, digits))
        .sum()
}

pub fn largest_n_digit_number(n: usize, digits: impl AsRef<[i8]>) -> i64 {
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

    let mut builder = vec![-1i8; n];
    digits
        .as_ref()
        .iter()
        .rev()
        .copied()
        .enumerate()
        .fold(-1, |acc, (i, digit)| {
            if i < n {
                builder[n - 1 - i] = digit;
            } else {
                include_digit(&mut builder, digit, 0);
            }
            acc.max(build_digit(&builder))
        })
}

#[cfg(test)]
mod test {
    use indoc::indoc;

    const TEST_INPUT: &str = indoc! {"
        987654321111111
        811111111111119
        234234234234278
        818181911112111
    "};

    #[test]
    fn test_part1() {
        assert_eq!(16927, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(167384358365132, super::part_two(TEST_INPUT));
    }
}
