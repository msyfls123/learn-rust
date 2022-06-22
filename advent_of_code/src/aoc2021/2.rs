#[macro_use] extern crate lazy_static;
extern crate regex;

use advent_of_code::get_str_array_from_file;
use itertools::Itertools;
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

#[derive(Debug, Clone)]
struct Position {
    horizontal: isize,
    depth: isize,
    aim: isize,
}

#[derive(Debug)]
enum CommandType {
    Down,
    Up,
    Forward,
}

#[derive(Debug)]
struct RawCommand {
    t: CommandType,
    x: isize,
}

fn get_raw_command(text: &str) -> RawCommand {
    lazy_static! {
        static ref RE_FORWARD: Regex = Regex::new(r"forward\s(?P<step>\d+)").unwrap();
        static ref RE_DOWN: Regex = Regex::new(r"down\s(?P<step>\d+)").unwrap();
        static ref RE_UP: Regex = Regex::new(r"up\s(?P<step>\d+)").unwrap();
    }

    match RE_FORWARD.captures(text) {
        Some(cap) => RawCommand { t: CommandType::Forward, x: get_step(cap) },
        None => {
            match RE_DOWN.captures(text) {
                Some(cap) => RawCommand { t: CommandType::Down, x: get_step(cap) },
                None => {
                    match RE_UP.captures(text) {
                        Some(cap) => RawCommand { t: CommandType::Up, x: get_step(cap) },
                        None => panic!("not reachable!"),
                    }
                }
            }
        }
    }
}

fn calc_new_position(pos: &Position, command: &RawCommand) -> Position {
    match command.t {
        CommandType::Down => Position {
            horizontal: pos.horizontal,
            depth: pos.depth,
            aim: pos.aim + command.x,
        },
        CommandType::Up => Position {
            horizontal: pos.horizontal,
            depth: pos.depth,
            aim: pos.aim - command.x,
        },
        CommandType::Forward => Position {
            horizontal: pos.horizontal + command.x,
            depth: pos.depth + pos.aim * command.x,
            aim: pos.aim,
        }
    }
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

    let raw_commands: Vec<RawCommand> = data.iter().map(|text| get_raw_command(text)).collect();

    let position = raw_commands.iter().fold(Position { horizontal: 0, depth: 0, aim: 0 }, |pos, command| {
        calc_new_position(&pos, command)
    });

    println!("Part 2: {}", position.horizontal * position.depth);
}