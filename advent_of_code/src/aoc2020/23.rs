#![feature(linked_list_cursors)]
use std::{collections::{LinkedList, linked_list::{CursorMut}}, iter::FromIterator};


fn move_cursor<T>(cursor: &mut CursorMut<T>, next: bool) {
  if next {
    cursor.move_next();
  } else {
    cursor.move_prev();
  }
  match cursor.current() {
    Some(_) => (),
    None => {
      if next {
        cursor.move_next();
      } else {
        cursor.move_prev();
      }
    }
  }
}

fn find_destination_cup(picked_cups: &Vec<usize>, target: usize) -> usize {
  let mut available = Vec::from_iter((1..=9));
  available.retain(|v| !picked_cups.contains(v));
  available.sort();
  available.reverse();
  match available.iter().find(|&v| *v < target) {
    Some(&label) => label,
    None => available[0]
  }
}

fn safe_get_current<T: ToOwned<Owned = T>>(cursor: &mut CursorMut<T>, should_remove: bool) -> T {
  match cursor.current() {
    Some(_) => {
      if should_remove {
        cursor.remove_current().unwrap().to_owned()
      } else {
        cursor.current().unwrap().to_owned()
      }
    },
    None => {
      move_cursor(cursor, true);
      safe_get_current(cursor, should_remove)
    }
  }
}

fn move_cursor_to_target<T: Eq + ToOwned<Owned = T>>(cursor: &mut CursorMut<T>, target: T) {
  while safe_get_current(cursor, false) != target {
      move_cursor(cursor, true);
  }
}

fn round(mut cursor: &mut CursorMut<usize>) {
  let current_cup = cursor.current().unwrap().to_owned();
  move_cursor(&mut cursor, true);
  let picked_cups: Vec<usize> = (0..3).map(|_x| {
    safe_get_current(&mut cursor, true)
  }).collect();
  let next_cup = safe_get_current(&mut cursor, false);
  let destination_cup = find_destination_cup(&picked_cups, current_cup);
  move_cursor_to_target(&mut cursor, destination_cup);
  cursor.splice_after(LinkedList::from_iter(picked_cups.iter().map(|v| v.to_owned())));
  move_cursor_to_target(&mut cursor, next_cup);
  // println!("picked: {:?}, current: {}, dest: {}, next: {}", picked_cups, current_cup, destination_cup, next_cup);
}

fn main() {
  let mut list = LinkedList::from([5, 6, 2, 8, 9, 3, 1, 4, 7]);
  let mut cursor = list.cursor_front_mut();
  for _ in 0..100 {
      round(&mut cursor)
  }
  move_cursor_to_target(&mut cursor, 1);
  let result1 = (0..8).map(|_| {
    move_cursor(&mut cursor, true);
    safe_get_current(&mut cursor, false).to_string()
  }).collect::<Vec<String>>().join("");
  println!("Part 1: {}", result1);
}
