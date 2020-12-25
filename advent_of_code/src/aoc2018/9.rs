#![feature(linked_list_cursors)]
use std::collections::linked_list::{LinkedList, CursorMut};
use std::collections::HashMap;

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

fn get_score(players_count: usize, last: usize) -> usize {
  let mut list: LinkedList<usize> = LinkedList::new();
  let mut marble = 0;
  let mut scores: HashMap<usize, usize> = HashMap::new();
  let mut play_id = 0;
  list.push_back(marble);
  let mut cursor = list.cursor_back_mut();
  loop {
    if marble == last {
      break
    }
    play_id = (play_id + 1) % players_count;
    marble += 1;
    if marble % 23 == 0 {
      (0..7).for_each(|_x| {
        move_cursor(&mut cursor, false);
      });
      let removed = cursor.remove_current().unwrap();
      let entry = scores.entry(play_id).or_insert(0);
      *entry += removed + marble;
    } else {
      move_cursor(&mut cursor, true);
      cursor.insert_after(marble);
      move_cursor(&mut cursor, true);
    }
  }
  let high_score = scores.iter().map(|s| s.1).max().unwrap();
  *high_score
}

fn main() {
  println!("{}", get_score(10, 1618));
  println!("{}", get_score(13, 7999));
  println!("{}", get_score(17, 1104));
  println!("{}", get_score(21, 6111));
  println!("{}", get_score(30, 5807));
  println!("Part 1: the winning Elf's score is {}.", get_score(424, 71144));
  println!("Part 2: the new winning Elf's score be if the number of the last marble were 100 times larger is {}.", get_score(424, 7114400));
}
