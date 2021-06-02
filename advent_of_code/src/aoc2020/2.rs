#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::{ Regex };
use advent_of_code::get_str_array_from_file;

struct Policy {
  min: usize,
  max: usize,
  letter: char,
  password: String
}

fn get_policy(text: &str) -> Policy {
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

  Policy {
    min,
    max,
    letter,
    password,
  }
}

fn check_in_range(policy: &Policy) -> bool {
  let Policy { password, min, max, letter } = policy;
  let count = password.chars().filter(|x| x == letter).count();
  count >= *min && count <= *max
}

fn check_in_position(policy: &Policy) -> bool {
  let Policy { password, min, max, letter } = policy;
  let chars = password.chars();
  (chars.clone().nth(*min - 1).unwrap() == *letter) ^ (chars.clone().nth(*max - 1).unwrap() == *letter)
}

fn main() {
  let password_list = get_str_array_from_file(&vec!{"aoc2020", "data", "2.txt"});
  let valid_passwords: Vec<String> = password_list.iter().filter(|x| check_in_range(&get_policy(x)))
    .map(|x| x.to_owned()).collect();
  println!("Part 1: {}", valid_passwords.len());
  let valid_passwords: Vec<String> = password_list.iter().filter(|x| check_in_position(&get_policy(x)))
    .map(|x| x.to_owned()).collect();
  println!("Part 2: {}", valid_passwords.len());
}
