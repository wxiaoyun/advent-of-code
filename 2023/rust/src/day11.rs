use crate::{get_input_for_day, get_test_input, Result};

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
