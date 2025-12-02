use std::collections;

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let mat = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nrow = mat.len();
    let ncol = mat[0].len();

    let mut occurances = collections::HashMap::new();
    mat.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &c)| {
            if c == '.' {
                return;
            }
            let mut e = occurances.entry(c);
            let mut v = e.or_insert(Vec::new());
            v.push((i, j));
        });
    });

    let mut used = collections::HashSet::new();
    let mut count = 0;
    occurances.iter().for_each(|(_, l)| {
        for i in 0..l.len() {
            for j in 0..l.len() {
                if i == j {
                    continue;
                }

                let (i1, j1) = (l[i].0 as i32, l[i].1 as i32);
                let (i2, j2) = (l[j].0 as i32, l[j].1 as i32);

                let (di, dj) = (i1 - i2, j1 - j2);

                let ii = i1 + di;
                let jj = j1 + dj;
                if ii.min(jj) >= 0
                    && ii < nrow as i32
                    && jj < ncol as i32
                    && !used.contains(&(ii, jj))
                {
                    count += 1;
                    used.insert((ii, jj));
                }
            }
        }
    });

    count as i64
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let mat = input
        .as_ref()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let nrow = mat.len();
    let ncol = mat[0].len();

    let mut occurances = collections::HashMap::new();
    mat.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, &c)| {
            if c == '.' {
                return;
            }
            let mut e = occurances.entry(c);
            let mut v = e.or_insert(Vec::new());
            v.push((i, j));
        });
    });

    let mut used = collections::HashSet::new();
    let mut count = 0;
    occurances.iter().for_each(|(_, l)| {
        for i in 0..l.len() {
            for j in 0..l.len() {
                if i == j {
                    continue;
                }

                let (i1, j1) = (l[i].0 as i32, l[i].1 as i32);
                let (i2, j2) = (l[j].0 as i32, l[j].1 as i32);

                let (di, dj) = (i1 - i2, j1 - j2);

                let mut ii = i1;
                let mut jj = j1;
                while ii.min(jj) >= 0 && ii < nrow as i32 && jj < ncol as i32 {
                    if !used.contains(&(ii, jj)) {
                        count += 1;
                        used.insert((ii, jj));
                    }
                    ii += di;
                    jj += dj;
                }
            }
        }
    });

    count as i64
}
