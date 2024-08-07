#![feature(btree_cursors)]
#![allow(unused)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod prelude;

pub use prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day01::part_one()?;
    Ok(())
}
