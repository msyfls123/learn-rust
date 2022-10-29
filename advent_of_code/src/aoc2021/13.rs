use std::collections::{HashSet};

use advent_of_code::get_group_str_from_file;
use itertools::Itertools;

type Paper = HashSet<(isize, isize)>;

#[derive(Debug, PartialEq)]
enum FoldInstruction {
  Up(isize),
  Left(isize),
}

fn get_paper(lines: &Vec<String>) -> Paper {
  let mut paper = HashSet::new();
  lines.iter().for_each(|l| {
    paper.insert(l.split(",").map(|v| v.parse::<isize>().unwrap()).collect_tuple().unwrap());
  });
  paper
}

fn get_fold_instructions(lines: &Vec<String>) -> Vec<FoldInstruction> {
  lines.iter().map(|l| {
    let substr: String = l.chars().skip(11).collect();
    let value = substr.chars().skip(2).collect::<String>().parse::<isize>().unwrap();
    if substr.starts_with("y") {
      FoldInstruction::Up(value)
    } else {
      FoldInstruction::Left(value)
    }
  }).collect()
}

#[test]
fn test_get_fold_instructions() {
  assert_eq!(get_fold_instructions(&vec!{String::from("fold along y=7")}), vec!{FoldInstruction::Up(7)});
}

fn fold(paper: &Paper, fold_instruction: &FoldInstruction) -> Paper {
  let mut new_paper: Paper = HashSet::new();
  paper.iter().for_each(|pos| {
    match fold_instruction {
      FoldInstruction::Left(x) => {
        if pos.0 < *x {
          new_paper.insert(pos.clone());
        } else {
          new_paper.insert((2 * x - pos.0, pos.1));
        }
      },
      FoldInstruction::Up(y) => {
        if pos.1 < *y {
          new_paper.insert(pos.clone());
        } else {
          new_paper.insert((pos.0, 2 * y - pos.1));
        }
      },
    }
  });
  new_paper
}

#[test]
fn test_fold() {
  let lines = r#"6,10
  0,14
  9,10
  0,3
  10,4
  4,11
  6,0
  6,12
  4,1
  0,13
  10,12
  3,4
  3,0
  8,4
  1,10
  2,14
  8,10
  9,0"#.lines().into_iter().map(|t| t.trim().to_string()).collect();
  let paper = get_paper(&lines);
  let new_paper = fold(&paper, &FoldInstruction::Up(7));
  assert_eq!(new_paper.len(), 17);
}

fn print_paper(paper: &Paper) {
  let x_list: Vec<isize> = paper.iter().map(|(x, _)| *x).collect();
  let y_list: Vec<isize> = paper.iter().map(|(_, y,)| *y).collect();
  let x_min = x_list.iter().min().unwrap().to_owned();
  let x_max = x_list.iter().max().unwrap().to_owned();
  let y_min = y_list.iter().min().unwrap().to_owned();
  let y_max = y_list.iter().max().unwrap().to_owned();
  (y_min..=y_max).for_each(|y| {
    (x_min..=x_max).for_each(|x| {
      if paper.contains(&(x, y)) {
        print!("#");
      } else {
        print!(".");
      }
    });
    println!("");
  });
}

fn main() {
  let group_lines = get_group_str_from_file(&vec!{"aoc2021", "data", "13.txt"});
  let paper = get_paper(&group_lines[0]);
  let fold_instructions = get_fold_instructions(&group_lines[1]);

  let first_fold_paper = fold(&paper, &fold_instructions[0]);
  println!("Part 1: {}", first_fold_paper.len());

  let fold_paper = fold_instructions.iter().fold(paper, |acc, instruction| {
    fold(&acc, &instruction)
  });
  println!("Part 2:");
  print_paper(&fold_paper);
}
