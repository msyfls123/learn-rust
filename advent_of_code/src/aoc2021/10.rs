#![feature(linked_list_cursors)]

use std::collections::{LinkedList, linked_list::CursorMut};

use advent_of_code::get_str_array_from_file;

type Chunks = LinkedList<isize>;

type ChunksCursor<'a> = CursorMut<'a, isize>;

enum GetLineMode {
  ILLEGAL,
  COMPLETION,
}

fn get_line(line: &str, mode: GetLineMode) -> Vec<isize> {
  line.chars().map(|c| {
    match mode {
      GetLineMode::COMPLETION => {
        match c {
          '(' => -1,
          ')' => 1,
          '[' => -2,
          ']' => 2,
          '{' => -3,
          '}' => 3,
          '<' => -4,
          '>' => 4,
          _ => panic!("should not go here"),
        }
      },
      GetLineMode::ILLEGAL => {
        match c {
          '(' => -3,
          ')' => 3,
          '[' => -57,
          ']' => 57,
          '{' => -1197,
          '}' => 1197,
          '<' => -25137,
          '>' => 25137,
          _ => panic!("should not go here"),
        }
      }
    }
    
  }).collect()
}

#[test]
fn test_try_push_chunk() {
  let mut chunks: Chunks = LinkedList::new();
  let mut cursor = chunks.cursor_front_mut();
  assert_eq!(try_push_chunk(&mut cursor, -2), true);
  assert_eq!(try_push_chunk(&mut cursor, -3), true);
  assert_eq!(try_push_chunk(&mut cursor, -1), true);
  assert_eq!(try_push_chunk(&mut cursor, 1), true);
  assert_eq!(try_push_chunk(&mut cursor, 3), true);
  assert_eq!(try_push_chunk(&mut cursor, 2), true);
  assert_eq!(try_push_chunk(&mut cursor, 1), false);
}

fn try_push_chunk<'a>(cursor: &mut ChunksCursor, num: isize) -> bool {
  match cursor.current() {
    Some(v) => {
      if num < 0 {
        cursor.push_back(num);
        cursor.move_next();
        true
      } else if *v + num == 0 {
        cursor.remove_current();
        cursor.move_prev();
        true
      } else {
        false
      }
    },
    None => {
      if num > 0 {
        return false
      }
      cursor.push_back(num);
      cursor.move_next();
      true
    }
  }
}

#[test]
fn test_find_first_illeal_character() {
  let line = get_line("{([(<{}[<>[]}>{[]{[(<()>", GetLineMode::ILLEGAL);
  assert_eq!(find_first_illeal_character(&line), Some(1197));
}

fn find_first_illeal_character(line: &Vec<isize>) -> Option<isize> {
  let mut chunks: Chunks = LinkedList::new();
  let mut cursor = chunks.cursor_front_mut();
  line.iter().find(|&&num| !try_push_chunk(&mut cursor, num)).map(|v| v.to_owned())
}

fn calculate_score(completion_str: &Vec<isize>) -> isize {
  completion_str.iter().fold(0, |acc, v| acc * 5 + v)
}

fn find_closing_characters(line: &Vec<isize>) -> Option<Vec<isize>> {
  let mut chunks: Chunks = LinkedList::new();
  let mut cursor = chunks.cursor_front_mut();
  match line.iter().all(|&num| try_push_chunk(&mut cursor, num)) {
    true => {
      Some(chunks.iter().rev().map(|v| 0 - v).collect())
    },
    false => None,
  }
}

#[test]
fn test_find_closing_characters() {
  let line = get_line("[({(<(())[]>[[{[]{<()<>>", GetLineMode::COMPLETION);
  let completion_str = find_closing_characters(&line).unwrap();
  assert_eq!(calculate_score(&completion_str), 288957);
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2021", "data", "10.txt"});
  let lines: Vec<Vec<isize>> = data.iter().map(|line| get_line(line, GetLineMode::ILLEGAL)).collect();
  let first_illeal_characters: Vec<isize> = lines.iter().filter_map(|line| {
    find_first_illeal_character(line)
  }).collect();

  println!("Part 1: {}", first_illeal_characters.iter().sum::<isize>());

  let lines: Vec<Vec<isize>> = data.iter().map(|line| get_line(line, GetLineMode::COMPLETION)).collect();
  let mut completion_scores: Vec<isize> = lines.iter().filter_map(|line| {
    find_closing_characters(line)
  }).map(|chars| calculate_score(&chars)).collect();
  completion_scores.sort();
  let mid_index = completion_scores.len() / 2;
  println!("Part 2: {}", completion_scores[mid_index]);
}
