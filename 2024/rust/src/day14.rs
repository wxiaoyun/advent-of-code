use image::{Rgb, RgbImage};
use std::iter;

use crate::{get_input_for_day, get_test_input};

struct Robot {
    pos_x: i64,
    pos_y: i64,
    vel_x: i64,
    vel_y: i64,
}

impl Robot {
    fn simulate(&mut self, w: i64, h: i64) {
        self.pos_x = (self.pos_x + self.vel_x + w) % w;
        self.pos_y = (self.pos_y + self.vel_y + h) % h;
    }

    fn simulate_n(&mut self, w: i64, h: i64, n: usize) {
        for _ in 0..n {
            self.simulate(w, h);
        }
    }
}

pub fn part_one() {
    let safety_factor: u64 = get_input_for_day(14)
        .lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();
            let mut pos = parts.next().unwrap()[2..].split(',');
            let mut vel = parts.next().unwrap()[2..].split(',');

            Robot {
                pos_x: pos.next().unwrap().parse::<i64>().unwrap(),
                pos_y: pos.next().unwrap().parse::<i64>().unwrap(),
                vel_x: vel.next().unwrap().parse::<i64>().unwrap(),
                vel_y: vel.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .map(|mut r| {
            r.simulate_n(101, 103, 100);
            r
        })
        .fold([0, 0, 0, 0], |mut acum, r| {
            if r.pos_x == 50 || r.pos_y == 51 {
                return acum;
            }

            let i = r.pos_x / 51;
            let j = r.pos_y / 52;
            acum[(i * 2 + j) as usize] += 1;
            acum
        })
        .iter()
        .product();

    println!("{}", safety_factor);
}

pub fn part_two() {
    fn draw_simulate(robots: &mut Vec<Robot>, canvas: &mut Vec<Vec<u8>>, tick: usize) {
        for r in robots.iter() {
            canvas[r.pos_y as usize][r.pos_x as usize] = 0;
        }

        for r in robots.iter_mut() {
            r.simulate(101, 103);
        }

        for r in robots.iter() {
            canvas[r.pos_y as usize][r.pos_x as usize] = 255;
        }

        let mut img = RgbImage::new(canvas.len() as u32, canvas[0].len() as u32);

        for (r, row) in canvas.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                img.put_pixel(r as u32, c as u32, Rgb([v, 0, 0]));
            }
        }

        let file_name = format!("./tmp/{}.jpg", tick);
        std::fs::create_dir_all("./tmp").unwrap();
        std::fs::File::create(&file_name).unwrap();
        img.save(file_name).unwrap();
    }

    let mut robots = get_input_for_day(14)
        .lines()
        .map(|l| {
            let mut parts = l.split_ascii_whitespace();
            let mut pos = parts.next().unwrap()[2..].split(',');
            let mut vel = parts.next().unwrap()[2..].split(',');

            Robot {
                pos_x: pos.next().unwrap().parse::<i64>().unwrap(),
                pos_y: pos.next().unwrap().parse::<i64>().unwrap(),
                vel_x: vel.next().unwrap().parse::<i64>().unwrap(),
                vel_y: vel.next().unwrap().parse::<i64>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let mut ticks = 0;
    let row = iter::repeat(0).take(101).collect::<Vec<_>>();
    let mut canvas = iter::repeat(row.clone()).take(103).collect::<Vec<_>>();
    for _ in 0..10000 {
        draw_simulate(&mut robots, &mut canvas, ticks);
        println!("{} ticks", ticks);
        ticks += 1;
    }
}
