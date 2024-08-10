use crate::{get_input_for_day, Result};

pub fn part_one() -> Result {
    let mut src = (-1, -1);
    let mut rows = -1;
    let mut cols = -1;

    let lookup = get_input_for_day(10)
        .lines()
        .map(|l| l.trim().chars().enumerate().collect::<Vec<_>>())
        .enumerate()
        .flat_map(|(i, vec)| {
            rows = rows.max(i as i32 + 1);
            vec.into_iter()
                .map(|(j, c)| {
                    if c == 'S' {
                        src = (i as i32, j as i32);
                    }
                    cols = cols.max(j as i32 + 1);
                    ((i as i32, j as i32), c)
                })
                .collect::<Vec<_>>()
        })
        .collect::<std::collections::HashMap<_, _>>();

    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    let mut longest = 0;

    queue.push_front((src, 0));

    while let Some((node, dist)) = queue.pop_back() {
        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);
        longest = longest.max(dist);
        let c = *lookup.get(&node).unwrap();
        let (i, j) = node;

        // Go up
        if i > 0 && "S|JL".contains(c) && "|7F".contains(*lookup.get(&(i - 1, j)).unwrap()) {
            queue.push_front(((i - 1, j), dist + 1));
        }

        // Go down
        if i < rows - 1 && "S|F7".contains(c) && "|JL".contains(*lookup.get(&(i + 1, j)).unwrap()) {
            queue.push_front(((i + 1, j), dist + 1));
        }

        // Go left
        if j > 0 && "S-7J".contains(c) && "-LF".contains(*lookup.get(&(i, j - 1)).unwrap()) {
            queue.push_front(((i, j - 1), dist + 1));
        }

        // Go right
        if j < cols - 1 && "S-LF".contains(c) && "-7J".contains(*lookup.get(&(i, j + 1)).unwrap()) {
            queue.push_front(((i, j + 1), dist + 1));
        }
    }

    println!("Longest loop: {}", longest);

    Ok(())
}
