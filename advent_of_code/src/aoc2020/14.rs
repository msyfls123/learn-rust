extern crate regex;
#[macro_use] extern crate lazy_static;

use advent_of_code::get_str_array_from_file;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
  Mask(Vec<char>),
  Write((usize, usize))
}

fn get_instruction(text: &str) -> Instruction {
  lazy_static! {
    static ref RE_MASK: Regex = Regex::new(r"mask\s=\s(?P<mask>[X01]{36})").unwrap();
    static ref RE_WRITE: Regex = Regex::new(r"mem\[(?P<address>\d+)\]\s=\s(?P<value>\d+)").unwrap();
  }
  if let Some(captured) = RE_MASK.captures(text) {
    let mask = captured.name("mask").unwrap().as_str().chars().collect();
    return Instruction::Mask(mask)
  } else if let Some (captured) = RE_WRITE.captures(text) {
    let address = captured.name("address").unwrap().as_str().parse().unwrap();
    let value = captured.name("value").unwrap().as_str().parse().unwrap();
    return Instruction::Write((address, value))
  }
  panic!("not here");
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "14.txt"});
  let instructions: Vec<Instruction> = data.iter().map(|text| get_instruction(text)).collect();
  println!("{:?}", instructions);
}
