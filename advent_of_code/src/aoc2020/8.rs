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

#[derive(Debug)]
enum TerminateStatus {
  InfiniteLoop(i32),
  Normal(i32)
}

fn move_cursor(cursor: &mut Cursor<Instruction>, argument: i32) {
  let to_next = argument > 0;
  let step = argument.abs() as usize;
  (0..step).for_each(|_| {
    if to_next {
      cursor.move_next();
    } else {
      cursor.move_prev();
    }
  });
}

fn run(cursor: &mut Cursor<Instruction>, accumulator: i32, corrupted: bool) -> i32 {
  let instruction = cursor.current().unwrap();
  match instruction.operation {
    "nop" => {
      if corrupted {
        move_cursor(cursor, instruction.argument);
        return accumulator
      }
      cursor.move_next();
      accumulator
    },
    "acc" => {
      cursor.move_next();
      accumulator + instruction.argument
    },
    "jmp" => {
      if corrupted {
        cursor.move_next();
        return accumulator
      }
      move_cursor(cursor, instruction.argument);
      accumulator
    }
    _ => unreachable!{}
  }
}

fn terminate(
  list: &LinkedList<Instruction>,
  corrupted_index: usize
) -> TerminateStatus {
  let mut visited: Vec<usize> = vec!{0};
  let mut accumulator = 0i32;
  let mut cursor = list.cursor_front();
  loop {
    let current_index = cursor.index().unwrap();
    accumulator = run(&mut cursor, accumulator, current_index == corrupted_index);
    match cursor.index() {
      Some(index) => {
        if visited.contains(&index) {
          return TerminateStatus::InfiniteLoop(accumulator)
        } else {
          visited.push(index);
        }
      },
      None => {
        return TerminateStatus::Normal(accumulator)
      }
    }
  }
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "8.txt"});
  let list = data.iter().map(|line| {
    let (operation, argument) = line.split(" ").collect_tuple().unwrap();
    Instruction {
      operation,
      argument: argument.parse::<i32>().unwrap(),
    }
  });
  let instructions = LinkedList::from_iter(list.to_owned());
  let len = instructions.len();
  let termate_status = terminate(&instructions, len);
  println!("Part 1: {:?}", termate_status);
  let nop_or_jmp_indexs: Vec<usize> = list.enumerate().filter_map(|(index, instruction)| {
    if ["jmp", "nop"].contains(&instruction.operation) {
      Some(index)
    } else {
      None
    }
  }).collect();
  let accumulator = nop_or_jmp_indexs.iter().find_map(|&index| {
    match terminate(&instructions, index) {
      TerminateStatus::Normal(acc) => Some(acc),
      _ => None,
    }
  });
  println!("Part 2: {:?}", accumulator);
}
