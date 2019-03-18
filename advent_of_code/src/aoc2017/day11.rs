use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn get_array(path: &Path) -> Vec<String> {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let array: Vec<String> = s.split(",").filter_map(|x| {
        if x == "" {
            None
        } else {
            Some(x.to_string())
        }
    }).collect();
    array
}

fn calc_step(x: i64, y: i64) -> i64 {
    let pos = [x.abs(), y.abs()];
    let max = pos.iter().max().unwrap();
    let min = pos.iter().min().unwrap();
    (max - min) / 2 + min
}

pub fn resolve() {
    let path = Path::new("advent_of_code").join("src").join("aoc2017").join("day11_data.txt");
    let array = get_array(&path);
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut max_step = 0;
    for dir in &array {
        match &dir[..] {
            "nw" => {
                x -= 1;
                y += 1;
            },
            "n" => {
                y += 2;
            },
            "ne" => {
                x += 1;
                y += 1;
            },
            "sw" => {
                x -= 1;
                y -= 1;
            },
            "s" => {
                y -= 2;
            },
            "se" => {
                x += 1;
                y -= 1;
            },
            _ => {}
        };
        let curr_step = calc_step(x, y);
        if curr_step > max_step {
            max_step = curr_step
        }
    };
    println!("Resolve 1: {}", calc_step(x, y));
    println!("Resolve 2: {}", max_step);
}

fn main() {
    resolve();
}