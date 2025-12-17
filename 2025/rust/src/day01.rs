pub fn part_one(input: impl AsRef<str>) -> i64 {
    let rotations = input.as_ref().lines().map(|l| {
        let dir = match l.chars().next().unwrap() {
            'L' => -1,
            'R' => 1,
            _ => unreachable!(),
        };

        let step = l[1..].parse::<i32>().unwrap();
        (dir, step)
    });

    let mut zero_cnt = 0;
    let mut cur_dial = 50;
    for (dir, step) in rotations {
        cur_dial = (cur_dial + dir * step).rem_euclid(100);
        if cur_dial == 0 {
            zero_cnt += 1;
        }
    }

    zero_cnt
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let rotations = input.as_ref().lines().map(|l| {
        let dir = match l.chars().next().unwrap() {
            'L' => -1,
            'R' => 1,
            _ => unreachable!(),
        };

        let step = l[1..].parse::<i32>().unwrap();
        (dir, step)
    });

    let mut zero_cnt = 0;
    let mut cur_dial = 50;
    for (dir, step) in rotations {
        let n_cross_over = match dir {
            0.. => (cur_dial + step) / 100,
            ..0 => ((100 - cur_dial).rem_euclid(100) + step) / 100,
        };
        zero_cnt += n_cross_over;
        cur_dial = (cur_dial + dir * step).rem_euclid(100)
    }

    zero_cnt as i64
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = include_str!("../../../input/2025_01_1.test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(3, super::part_one(TEST_INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, super::part_two(TEST_INPUT));
    }
}
