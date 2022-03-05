#![feature(linked_list_cursors)]
use std::{collections::{LinkedList, linked_list::{CursorMut}}, iter::FromIterator, thread::current};


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
  println!("{:?}", available);
  match available.iter().find(|&v| *v < target) {
    Some(&label) => label,
    None => available[0]
  }
}

fn safe_get_current(cursor: &mut CursorMut<usize>, should_remove: bool) -> usize {
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

fn main() {
  let mut list = LinkedList::from([7, 2, 5, 8, 4, 1, 9, 3, 6]);
  let mut cursor = list.cursor_front_mut();
  move_cursor(&mut cursor, true);
  move_cursor(&mut cursor, true);
  move_cursor(&mut cursor, true);
  move_cursor(&mut cursor, true);
  move_cursor(&mut cursor, true);
  move_cursor(&mut cursor, true);
  let current_cup = cursor.current().unwrap().to_owned();
  move_cursor(&mut cursor, true);
  let picked_cups: Vec<usize> = (0..3).map(|_x| {
    safe_get_current(&mut cursor, true)
  }).collect();
  let next_cup = safe_get_current(&mut cursor, false);
  let destination_cup = find_destination_cup(&picked_cups, current_cup);
  println!("picked: {:?}, current: {}, dest: {}, next: {}", picked_cups, current_cup, destination_cup, next_cup);
}
