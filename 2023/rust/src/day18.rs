use std::collections::HashSet;

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let plan = parse_input(input);

    let mut vertices = Vec::new();
    let (mut r, mut c) = (0, 0);
    let mut boundary_len = 0;
    for (dir, step, _) in plan {
        let (dr, dc) = match dir {
            'U' => (-1, 0),
            'D' => (1, 0),
            'R' => (0, 1),
            'L' => (0, -1),
            _ => unreachable!(),
        };

        r = r + dr * step;
        c = c + dc * step;
        vertices.push((r, c));
        boundary_len += step;
    }

    // println!("{:?}", vertices);

    // Shoelace formula to calculate area
    let area = (0..vertices.len() as i64)
        .map(|i| {
            (vertices[i as usize].0
                * vertices[(i + 1).rem_euclid(vertices.len() as i64) as usize].1)
                - (vertices[i as usize].0
                    * vertices[(i - 1).rem_euclid(vertices.len() as i64) as usize].1)
        })
        .sum::<i64>()
        .abs()
        / 2;

    // Pick's theorem to calculate size
    area + boundary_len / 2 + 1
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let plan = parse_input(input).into_iter().map(|(_, _, s)| {
        let chars = s.chars().skip(2).take(6).collect::<Vec<_>>();
        let dir = match chars.last().unwrap() {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            c => unreachable!(),
        };

        let step = chars
            .iter()
            .take(5)
            .fold(0, |acc, c| acc * 16 + c.to_digit(16).unwrap()) as i64;

        (dir, step, ' ')
    });

    let mut vertices = Vec::new();
    let (mut r, mut c) = (0, 0);
    let mut boundary_len = 0;
    for (dir, step, _) in plan {
        let (dr, dc) = match dir {
            'U' => (-1, 0),
            'D' => (1, 0),
            'R' => (0, 1),
            'L' => (0, -1),
            _ => unreachable!(),
        };

        r = r + dr * step;
        c = c + dc * step;
        vertices.push((r, c));
        boundary_len += step;
    }

    // println!("{:?}", vertices);

    // Shoelace formula to calculate area
    let area = (0..vertices.len() as i64)
        .map(|i| {
            (vertices[i as usize].0
                * vertices[(i + 1).rem_euclid(vertices.len() as i64) as usize].1)
                - (vertices[i as usize].0
                    * vertices[(i - 1).rem_euclid(vertices.len() as i64) as usize].1)
        })
        .sum::<i64>()
        .abs()
        / 2;

    // Pick's theorem to calculate size
    area + boundary_len / 2 + 1
}

fn parse_input(input: impl AsRef<str>) -> Vec<(char, i64, String)> {
    input
        .as_ref()
        .lines()
        .map(|l| {
            use std::str::FromStr;

            let mut it = l.split_ascii_whitespace();
            let (dir, step, clr) = (
                it.next().unwrap().chars().nth(0).unwrap(),
                it.next()
                    .and_then(|c| <i64 as FromStr>::from_str(c).ok())
                    .unwrap(),
                it.next().unwrap().to_owned(),
            );
            (dir, step, clr)
        })
        .collect()
}
