use crate::{get_input_for_day, get_test_input, Result};

pub fn part_one() -> Result {
    let universe_mat = get_input_for_day(11)
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let galaxy_coords = expand_universe(universe_mat)
        .into_iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.into_iter()
                .enumerate()
                .map(|(j, c)| if c == '#' { Some((i, j)) } else { None })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut total_distance = 0;

    for i in 0..galaxy_coords.len() {
        let (x1, y1) = galaxy_coords[i];
        for (x2, y2) in galaxy_coords.iter().skip(i + 1) {
            let distance = (x1 as i64 - *x2 as i64).abs() + (y1 as i64 - *y2 as i64).abs();
            total_distance += distance as u64;
        }
    }

    println!("Total distance: {}", total_distance);

    Ok(())
}

fn expand_universe(universe: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_universe = Vec::new();

    // Construct new universe and expand row
    for row in universe.into_iter() {
        let is_all_dot = row.iter().all(|&x| x == '.');
        if is_all_dot {
            new_universe.push(row.clone());
        }
        new_universe.push(row);
    }

    let mut new_universe2 = new_universe.iter().map(|_| Vec::new()).collect::<Vec<_>>();

    // Expand column
    for j in 0..new_universe.first().unwrap().len() {
        let is_all_dot = new_universe.iter().all(|x| x[j] == '.');
        if is_all_dot {
            for row in new_universe2.iter_mut() {
                row.push('.');
            }
        }
        for i in 0..new_universe.len() {
            new_universe2[i].push(new_universe[i][j]);
        }
    }

    new_universe2
}

pub fn part_two() -> Result {
    let universe_mat = get_input_for_day(11)
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let expended_universe = expand_universe2(universe_mat);
    let galaxies = expended_universe
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, c)| if c == "#" { Some((i, j)) } else { None })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();

    let mut total_distance = 0;

    for i in 0..galaxies.len() {
        let (x1, y1) = galaxies[i];
        for (x2, y2) in galaxies.iter().skip(i + 1) {
            let start_x = x1.min(*x2);
            let end_x = x1.max(*x2);
            let start_y = y1.min(*y2);
            let end_y = y1.max(*y2);

            for i in start_x..end_x {
                let distance = expended_universe[i][start_y].parse::<u64>().unwrap_or(1); // We could cross a galaxy which is denoted as "#" and we need to consider that as 1
                total_distance += distance;
            }
            for j in start_y..end_y {
                let distance = expended_universe[end_x][j].parse::<u64>().unwrap_or(1);
                total_distance += distance;
            }
        }
    }

    println!("Total distance: {}", total_distance);

    Ok(())
}

fn expand_universe2(mut universe: Vec<Vec<char>>) -> Vec<Vec<String>> {
    // Construct new universe and expand row
    let mut new_universe = universe
        .into_iter()
        .map(|row| {
            let is_all_dot = row.iter().all(|&x| x == '.');
            row.into_iter()
                .map(|c| {
                    if is_all_dot {
                        // 1 million unit length
                        "1000000".to_owned()
                    } else if c == '.' {
                        "1".to_owned()
                    } else {
                        c.to_string()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Expand column
    for j in 0..new_universe.first().unwrap().len() {
        let is_all_dot = new_universe.iter().all(|x| x[j].starts_with("1"));
        if is_all_dot {
            new_universe
                .iter_mut()
                .for_each(|row| row[j] = "1000000".to_owned());
        }
    }

    new_universe
}
