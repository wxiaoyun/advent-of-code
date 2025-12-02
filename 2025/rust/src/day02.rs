use extfn::extfn;

#[extfn]
fn digits(self: i64) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut cur = self;
    while cur > 0 {
        digits.push((cur % 10) as u8);
        cur /= 10;
    }
    digits
}

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let ranges = input.as_ref().split(',').map(|r| {
        let mut range = r.split('-');
        let mut extract_num = || {
            range
                .next()
                .and_then(|s| s.trim().parse::<i64>().ok())
                .unwrap()
        };
        (extract_num(), extract_num())
    });

    struct InvalidFinder {
        left: i64,
        bound: i64,
    }

    impl InvalidFinder {
        fn new(l: i64, r: i64) -> Self {
            let l_digits = l.digits().len() as u32;
            let left = if l_digits.is_multiple_of(2) {
                let modd = 10i64.pow(l_digits / 2);
                let l_num = l / modd;
                let r_num = l % modd;

                if l_num < r_num { l_num + 1 } else { l_num }
            } else {
                let half_len = l_digits / 2;
                10i64.pow(half_len)
            };

            InvalidFinder { left, bound: r }
        }
    }

    impl Iterator for InvalidFinder {
        type Item = i64;

        fn next(&mut self) -> Option<Self::Item> {
            let left_digits = self.left.digits();
            let next_invalid = left_digits
                .iter()
                .rev()
                .cycle()
                .take(left_digits.len() * 2)
                .copied()
                .fold(0, |acc, n| acc * 10 + n as i64);

            if next_invalid > self.bound {
                return None;
            }

            self.left += 1;
            Some(next_invalid)
        }
    }

    ranges.flat_map(|(l, r)| InvalidFinder::new(l, r)).sum()
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    input
        .as_ref()
        .split(',')
        .map(|r| {
            let mut range = r.split('-');
            let mut extract_num = || {
                range
                    .next()
                    .and_then(|s| s.trim().parse::<i64>().ok())
                    .unwrap()
            };
            (extract_num(), extract_num())
        })
        .flat_map(|(l, r)| {
            fn is_valid(n: i64) -> bool {
                let digits = n.digits();
                let len = digits.len();

                'check_step: for step in 1..=(len / 2) {
                    if !len.is_multiple_of(step) {
                        continue;
                    }

                    for s in 0..step {
                        let digit = digits[s];
                        let mut i = s;
                        while i < len {
                            if digits[i] != digit {
                                continue 'check_step;
                            }
                            i += step;
                        }
                    }

                    return true;
                }

                false
            }

            (l..=r).filter(|&n| is_valid(n))
        })
        .sum()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = "\
        11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\
        1698522-1698528,446443-446449,38593856-38593862,565653-565659,\
        824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(1227775554, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4174379265, super::part_two(TEST_INPUT));
    }
}
