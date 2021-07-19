extern crate regex;
#[macro_use] extern crate lazy_static;

use advent_of_code::get_str_array_from_file;
use regex::Regex;
use std::collections::HashMap;

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
  use Instruction::*;
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "14.txt"});
  let instructions: Vec<Instruction> = data.iter().map(|text| get_instruction(text)).collect();
  let mut current_mask: Option<Instruction> = None;
  let mut memory: HashMap<usize, usize> = HashMap::new();
  for ins in instructions.iter() {
    match ins {
      Mask(mask) => current_mask = Some(Mask(mask.clone())),
      Write((address, value)) => {
        let mut val = value.clone();
        if let Some(Mask(ref mask)) = current_mask {
          let len = mask.len();
          val = mask.iter().enumerate().fold(val, |acc, (index, m)| {
            match m {
              'X' => acc,
              '0' => {
                acc - (acc & (1 << (len - 1 - index)))
              },
              '1' => acc | (1 << (len - 1 - index)),
              _ => panic!("no no no"),
            }
          });
        }
        memory.insert(*address, val);
      }
    }
  }
  let sum: usize = memory.values().sum();
  println!("Paart 1: {}", sum);

  let mut current_mask: Option<Instruction> = None;
  let mut memory: HashMap<usize, usize> = HashMap::new();
  for ins in instructions.iter() {
    match ins {
      Mask(mask) => current_mask = Some(Mask(mask.clone())),
      Write((address, value)) => {
        let mut addresses = vec!{*address};
        if let Some(Mask(ref mask)) = current_mask {
          let len = mask.len();
          addresses = mask.iter().enumerate().fold(addresses, |acc, (index, m)| {
            let bit = len - 1 - index;
            match m {
              '0' => acc,
              '1' => acc.iter().map(|a| a | (1 << bit)).collect(),
              'X' => {
                acc.iter().flat_map(|a| {
                  vec!{
                    a - (a & (1 << bit)),
                    a | (1 << bit)
                  }
                }).collect()
              },
              _ => panic!("no no no"),
            }
          });
        }
        addresses.iter().for_each(|&a| {
          memory.insert(a, *value);
        });
      }
    }
  }
  let sum: usize = memory.values().sum();
  println!("Paart 2: {}", sum);
}
