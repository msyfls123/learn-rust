#[macro_use] extern crate lazy_static;
extern crate regex;

use advent_of_code::get_str_array_from_file;
use regex::{ Regex, Captures };
use std::ops::Add;

#[derive(Debug, Clone)]
struct Command {
    horizontal: isize,
    vertical: isize,
}

impl Add for Command {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Command {
            horizontal: self.horizontal + rhs.horizontal,
            vertical: self.vertical + rhs.vertical,
        }
    }
}

fn get_step(cap: Captures) -> isize {
    cap.name("step").unwrap().as_str().parse::<isize>().unwrap()
}

fn get_command(text: &str) -> Command {
    lazy_static! {
        static ref RE_FORWARD: Regex = Regex::new(r"forward\s(?P<step>\d+)").unwrap();
        static ref RE_DOWN: Regex = Regex::new(r"down\s(?P<step>\d+)").unwrap();
        static ref RE_UP: Regex = Regex::new(r"up\s(?P<step>\d+)").unwrap();
    }

    match RE_FORWARD.captures(text) {
        Some(cap) => Command {
            horizontal: get_step(cap),
            vertical: 0,
        },
        None => {
            match RE_DOWN.captures(text) {
                Some(cap) => Command {
                    horizontal: 0,
                    vertical: get_step(cap),
                },
                None => {
                    match RE_UP.captures(text) {
                        Some(cap) => Command {
                            horizontal: 0,
                            vertical: -get_step(cap)
                        },
                        None => panic!("not reachable!"),
                    }
                }
            }
        }
    }
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2021", "data", "2.txt"});
    let commands: Vec<Command> = data.iter().map(|text| get_command(text)).collect();

    let position = commands.iter().fold(Command { horizontal: 0, vertical: 0 }, |acc, v| {
        acc + v.clone()
    });
    println!("Part 1: {}", position.horizontal * position.vertical);
}