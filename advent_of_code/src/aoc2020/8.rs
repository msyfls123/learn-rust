#![feature(linked_list_cursors)]
use std::collections::linked_list::{LinkedList, Cursor};
use advent_of_code::get_str_array_from_file;
use itertools::Itertools;
use std::iter::FromIterator;

#[derive(Debug)]
struct Instruction<'a> {
  operation: &'a str,
  argument: i32,
}

fn run(cursor: &mut Cursor<Instruction>, accumulator: i32) -> i32 {
  let instruction = cursor.current().unwrap();
  match instruction.operation {
    "nop" => {
      cursor.move_next();
      accumulator
    },
    "acc" => {
      cursor.move_next();
      accumulator + instruction.argument
    },
    "jmp" => {
      let to_next = instruction.argument > 0;
      let step = instruction.argument.abs() as usize;
      (0..step).for_each(|_| {
        if to_next {
          cursor.move_next();
        } else {
          cursor.move_prev();
        }
      });
      accumulator
    }
    _ => unreachable!{}
  }
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "8.txt"});
  let list = LinkedList::from_iter(data.iter().map(|line| {
    let (operation, argument) = line.split(" ").collect_tuple().unwrap();
    Instruction {
      operation,
      argument: argument.parse::<i32>().unwrap(),
    }
  }));
  let mut visited: Vec<usize> = vec!{0};
  let mut accumulator = 0i32;
  let mut cursor = list.cursor_front();
  loop {
    accumulator = run(&mut cursor, accumulator);
    let index = cursor.index().unwrap();
    if visited.contains(&index) {
      break
    } else {
      visited.push(index);
    }
  }
  println!("Part 1: {}", accumulator);
}
