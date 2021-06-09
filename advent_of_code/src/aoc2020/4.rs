#[macro_use] extern crate lazy_static;
extern crate regex;
use std::fmt::{Debug};

use regex::{ Regex };
use itertools::Itertools;
use advent_of_code::get_str_from_file;

const FIELDS: [&str; 7] = [
  "byr",
  "iyr",
  "eyr",
  "hgt",
  "hcl",
  "ecl",
  "pid",
];



fn digit_validator(min: usize, max: usize) -> impl Fn(&str) -> bool {
  move |text| {
    match text.parse::<usize>() {
      Ok(num) => num >= min && num <= max,
      Err(_) => false,
    }
  }
}

fn height_validator(text: &str) -> bool {
  let len = text.len();
  if len <= 2 {
    return false
  }
  match String::from(text).split_at(len - 2) {
    (num, "cm") => digit_validator(150, 193)(num),
    (num, "in") => digit_validator(59, 76)(num),
    _ => false
  }
}

fn color_validator(text: &str) -> bool {
  lazy_static! {
    static ref RE_COLOR: Regex = Regex::new(
      r"^#[0-9a-f]{6}$"
    ).unwrap();
  }
  RE_COLOR.is_match(text)
}

fn enum_validator(list: &'static Vec<&str>) -> impl Fn(&str) -> bool {
  move |text| {
    list.to_owned().contains(&text)
  }
}

fn passport_id_validator(text: &str) -> bool {
  lazy_static! {
    static ref RE_ID: Regex = Regex::new(
      r"^\d{9}$"
    ).unwrap();
  }
  RE_ID.is_match(text)
}

fn gen_validate(validator: impl Fn(&str) -> bool, size: usize) -> impl Fn(&str) -> usize {
  move |text| {
    if validator(text) {
      1 << size
    } else {
      0
    }
  }
}

fn validate(text: &str) -> usize {
  lazy_static! {
    static ref EYE_COLORS: Vec<&'static str> = vec!{"amb", "blu", "brn", "gry", "grn", "hzl", "oth"};
  }
  match &text.split(":").collect::<Vec<&str>>()[0..2] {
    ["byr", text] => gen_validate(digit_validator(1920, 2002), 0)(text),
    ["iyr", text] => gen_validate(digit_validator(2010, 2020), 1)(text),
    ["eyr", text] => gen_validate(digit_validator(2020, 2030), 2)(text),
    ["hgt", text] => gen_validate(height_validator, 3)(text),
    ["hcl", text] => gen_validate(color_validator, 4)(text),
    ["ecl", text] => gen_validate(enum_validator(&EYE_COLORS), 5)(text),
    ["pid", text] => gen_validate(passport_id_validator, 6)(text),
    ["cid", _text] => 0,
    _ => panic!("not implemented")
  }
}

fn main() {
  let data: Vec<String> = get_str_from_file(&vec!{"aoc2020", "data", "4.txt"})
    .lines()
    .map(|line| line.to_string()).collect();
  let passports: Vec<Vec<String>> = data.into_iter()
    .group_by(|line| line == "")
    .into_iter()
    .filter_map(|(_key, group)| {
      let line: Vec<String> = group.collect();
      if line == vec!{String::from("")} {
        return None
      }
      Some(line)
    })
    .map(|line| {
      line.into_iter()
        .intersperse(String::from(" "))
        .collect::<String>()
        .split(" ")
        .map(|x| x.to_string())
        .collect()
    })
    .collect();

  let valid_passports: Vec<Vec<String>> = passports.iter().filter_map(|passport| {
    match FIELDS.iter().all(|field| {
      passport.iter().any(|pair| pair.starts_with(field))
    }) {
      true => Some(passport.to_owned()),
      _ => None,
    }
  }).collect();
  println!("Part 1: {}", valid_passports.len());


  let valid_passports: Vec<Vec<String>> = passports.iter().filter_map(|passport| {
    match passport.iter().fold(0, |acc, field| {
      acc | validate(field)
    }) {
      0b1111111 => Some(passport.to_owned()),
      _ => None,
    }
  }).collect();
  println!("Part 2: {}", valid_passports.len());
}
