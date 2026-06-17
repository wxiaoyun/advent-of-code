use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_one(input: impl AsRef<str>) -> i64 {
    let mut coords = parse_input(input);
    coords.sort_by(|a, b| a.0.2.cmp(&b.0.2));

    let mut covered = vec![false; coords.len()];
    let mut n_support = vec![0; coords.len()];
    let mut support_list = vec![Vec::new(); coords.len()];
    let mut height_map = HashMap::new();
    let mut highest_blk = HashMap::new();
    for (i, ((sx, sy, sz), (ex, ey, ez))) in coords.iter().copied().enumerate() {
        let mut highest_below = -1;
        let mut highest_below_list = HashSet::new();

        for x in sx..=ex {
            for y in sy..=ey {
                let Some(z) = highest_blk.get(&(x, y)).copied() else {
                    continue;
                };

                if z < highest_below {
                    continue;
                }

                let j = height_map.get(&(x, y)).copied().unwrap();
                if z == highest_below {
                    highest_below_list.insert(j);
                } else {
                    highest_below = z;
                    highest_below_list.clear();
                    highest_below_list.insert(j);
                }
            }
        }

        n_support[i] = highest_below_list.len();
        for j in highest_below_list {
            // println!("i {:?} is above j {:?}", coords[i], coords[j]);
            covered[j] = true;
            support_list[j as usize].push(i);
        }

        let height = ez - sz + 1;
        let new_z = highest_below + height;

        for x in sx..=ex {
            for y in sy..=ey {
                highest_blk.insert((x, y), new_z);
                height_map.insert((x, y), i);
            }
        }
    }

    // println!("{:?}", coords.iter().enumerate().collect::<Vec<_>>());
    // println!("Cover {covered:?}");
    // println!("Support {n_support:?}");

    (0..coords.len())
        .map(|i| {
            let not_supporting = !covered[i];
            let all_supported_have_alternatives =
                support_list[i].iter().copied().all(|j| n_support[j] > 1);

            if not_supporting || all_supported_have_alternatives {
                1
            } else {
                0
            }
        })
        .sum::<i64>()
}

pub fn part_two(input: impl AsRef<str>) -> i64 {
    let mut coords = parse_input(input);
    coords.sort_by(|a, b| a.0.2.cmp(&b.0.2));

    let mut covered = vec![false; coords.len()];
    let mut supported_list = vec![0; coords.len()];
    let mut support_list = vec![Vec::new(); coords.len()];
    let mut height_map = HashMap::new();
    let mut highest_blk = HashMap::new();
    for (i, ((sx, sy, sz), (ex, ey, ez))) in coords.iter().copied().enumerate() {
        let mut highest_below = -1;
        let mut highest_below_list = HashSet::new();

        for x in sx..=ex {
            for y in sy..=ey {
                let Some(z) = highest_blk.get(&(x, y)).copied() else {
                    continue;
                };

                if z < highest_below {
                    continue;
                }

                let j = height_map.get(&(x, y)).copied().unwrap();
                if z == highest_below {
                    highest_below_list.insert(j);
                } else {
                    highest_below = z;
                    highest_below_list.clear();
                    highest_below_list.insert(j);
                }
            }
        }

        supported_list[i] = highest_below_list.len();
        for j in highest_below_list {
            // println!("i {:?} is above j {:?}", coords[i], coords[j]);
            covered[j] = true;
            support_list[j as usize].push(i);
        }

        let height = ez - sz + 1;
        let new_z = highest_below + height;

        for x in sx..=ex {
            for y in sy..=ey {
                highest_blk.insert((x, y), new_z);
                height_map.insert((x, y), i);
            }
        }
    }

    // println!("{:?}", coords.iter().enumerate().collect::<Vec<_>>());
    // println!("Cover {covered:?}");
    // println!("Support {support_list:?}");
    // println!("Supported {supported_list:?}");

    (0..coords.len())
        .map(|i| {
            let mut n_fell = 0;
            let mut fall_q = VecDeque::new();
            let mut seen = HashSet::new();
            fall_q.push_back(i);

            let mut supported_list = supported_list.clone();

            while let Some(j) = fall_q.pop_front() {
                if seen.contains(&j) {
                    continue;
                }
                seen.insert(j);
                n_fell += 1;

                support_list[j].iter().copied().for_each(|k| {
                    supported_list[k] -= 1;
                    if supported_list[k] == 0 {
                        fall_q.push_back(k);
                    }
                });
            }

            println! {"{i} {}", n_fell - 1};

            n_fell - 1
        })
        .sum::<i64>()
}

fn parse_input(input: impl AsRef<str>) -> Vec<((i64, i64, i64), (i64, i64, i64))> {
    input
        .as_ref()
        .lines()
        .map(|l| {
            let nums = l
                .split(['~', ','])
                .map(|s| s.parse::<i64>().ok())
                .collect::<Option<Vec<_>>>()
                .unwrap();
            ((nums[0], nums[1], nums[2]), (nums[3], nums[4], nums[5]))
        })
        .collect::<Vec<_>>()
}
