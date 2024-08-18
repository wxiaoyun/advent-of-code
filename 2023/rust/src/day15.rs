use std::str::FromStr;

use crate::{get_input_for_day, get_test_input, Result};

pub fn part_one() -> Result {
    let res = get_input_for_day(15)
        .split(",")
        .map(|part| part.trim())
        .filter(|l| !l.is_empty())
        .map(|l| hash(l.to_owned()))
        .sum::<u64>();

    println!("Day 15, Part 1: {}", res);

    Ok(())
}

fn hash(s: String) -> u64 {
    s.chars().fold(0, |accum, cur| {
        let hash = accum + cur as u64;
        let hash = hash * 17;
        hash % 256
    })
}

pub fn part_two() -> Result {
    let (res, _) = get_input_for_day(15)
        .split(",")
        .map(|part| part.trim())
        .filter(|part| !part.is_empty())
        .map(|part| part.parse::<Instruction>().unwrap())
        .fold(
            (
                vec![std::collections::LinkedList::new(); 256],
                std::collections::HashMap::new(),
            ),
            |(mut boxes, mut label_to_box), inst| match inst {
                Instruction::Add(label, focal_len) => {
                    let box_idx = hash(label.clone()) as usize;

                    let mut has_replaced = false;
                    boxes[box_idx] = boxes[box_idx]
                        .iter()
                        .map(|(other_label, other_fl): &(String, u64)| {
                            if label == *other_label {
                                has_replaced = true;
                                (label.clone(), focal_len)
                            } else {
                                (other_label.clone(), *other_fl)
                            }
                        })
                        .collect::<std::collections::LinkedList<_>>();

                    if has_replaced {
                        return (boxes, label_to_box);
                    }

                    boxes[box_idx].push_back((label.clone(), focal_len));
                    label_to_box.insert(label, box_idx);
                    (boxes, label_to_box)
                }
                Instruction::Rm(label) => {
                    if let Some(&box_idx) = label_to_box.get(&label) {
                        boxes[box_idx] = boxes[box_idx]
                            .iter()
                            .filter(|(other_label, _)| label != *other_label)
                            .map(|(other_label, other_fl)| (other_label.clone(), *other_fl))
                            .collect::<std::collections::LinkedList<_>>();
                    };

                    (boxes, label_to_box)
                }
            },
        );

    let res = res.into_iter().enumerate().fold(0, |acum, (bx_idx, bx)| {
        acum + bx.into_iter().enumerate().fold(0, |acum, (idx, (_, fl))| {
            acum + ((bx_idx as u64 + 1) * (idx as u64 + 1) * fl)
        })
    });

    println!("Day 15, Part 2: {}", res);
    Ok(())
}

enum Instruction {
    Rm(String),
    Add(String, u64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.contains("=") {
            let mut parts = s.split("=");
            let label = parts.next().unwrap();
            let focal_len = parts.next().unwrap().parse::<u64>().unwrap();
            return Ok(Instruction::Add(label.to_owned(), focal_len));
        }

        if s.contains("-") {
            return Ok(Instruction::Rm(s[0..s.len() - 1].to_owned()));
        }

        Err(format!("Invalid instruction: {}", s))
    }
}
