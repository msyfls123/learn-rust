#[macro_use] extern crate lazy_static;
extern crate regex;

use advent_of_code::get_str_array_from_file;
use regex::{ Regex };

type Instruction = [char; 2];

fn get_instruction(text: &str) -> Instruction {
  lazy_static! {
    static ref RE_INSTRUCTION: Regex = Regex::new(r"Step\s(?P<before>[A-Z])\smust\sbe\sfinished\sbefore\sstep\s(?P<after>[A-Z])\scan\sbegin").unwrap();
  }
  let captured = RE_INSTRUCTION.captures(text).unwrap();
  let before = captured.name("before").unwrap().as_str().parse::<char>().unwrap();
  let after = captured.name("after").unwrap().as_str().parse::<char>().unwrap();
  [before, after]
}

fn main() {
  let array = get_str_array_from_file(&vec!{"aoc2018", "data", "7.txt"});
  let instructions: Vec<Instruction> = array.iter().map(|t| get_instruction(t)).collect();
  println!("{:?}", instructions);
}
