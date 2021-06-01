#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::{ Regex };
use advent_of_code::get_str_array_from_file;

fn check_policy(text: &str) -> bool {
  lazy_static! {
    static ref RE_PASSWORD: Regex = Regex::new(
      r"(?P<min>\d+)-(?P<max>\d+)\s(?P<letter>[a-z]):\s(?P<password>[a-z]+)"
    ).unwrap();
  }

  let captured = RE_PASSWORD.captures(text).unwrap();
  let min: usize = captured.name("min").unwrap().as_str().parse().unwrap();
  let max: usize = captured.name("max").unwrap().as_str().parse().unwrap();
  let letter: char = captured.name("letter").unwrap().as_str().chars().nth(0).unwrap();
  let password = captured.name("password").unwrap().as_str().to_string();

  let count = password.chars().filter(|&x| x == letter).count();
  count >= min && count <= max
}

fn main() {
  let password_list = get_str_array_from_file(&vec!{"aoc2020", "data", "2.txt"});
  let valid_passwords: Vec<String> = password_list.iter().filter(|x| check_policy(x))
    .map(|x| x.to_owned()).collect();
  println!("Part 1: {}", valid_passwords.len());
}
