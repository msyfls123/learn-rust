#![feature(linked_list_cursors)]
use std::collections::linked_list::{LinkedList, Cursor};
use std::cell::{RefCell};

static RECEIPES_COUNT: usize = 306281;
type List = LinkedList<u32>;

fn move_cursor<T>(cursor: &mut Cursor<T>, next: bool) {
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

struct Scoreboard<'a> {
  list: RefCell<List>,
  first: Cursor<'a, u32>,
  second: Cursor<'a, u32>,
  length: usize,
}

impl Scoreboard<'_> {
  fn append(&mut self, elem: u32) {
    self.list.borrow_mut().push_back(elem);
    self.length += 1;
  }

  fn get_current_scores(&self) -> (u32, u32) {
    (*self.first.current().unwrap(), *self.second.current().unwrap())
  }

  fn forward(&mut self, scores: (u32, u32)) {
    let (first_score, second_score) = scores;
    (0..first_score + 1).for_each(|_| {
      move_cursor(&mut self.first, true);
    });
    (0..second_score + 1).for_each(|_| {
      move_cursor(&mut self.second, true);
    });
  }

  fn create(&mut self) {
    let scores = self.get_current_scores();
    let digits: Vec<u32> = (scores.0 + scores.1).to_string().chars().map(|x| x.to_digit(10).unwrap()).collect();
    digits.iter().for_each(|&x| {
      self.append(x);
    });
    self.forward(scores);
  }
}

fn main() {
  let raw_list: List = LinkedList::new();
  unsafe {
    let rc_list = RefCell::new(raw_list);
    rc_list.borrow_mut().push_back(3);
    let first = rc_list.as_ptr().as_ref().unwrap().cursor_back();
    rc_list.borrow_mut().push_back(7);
    let second = rc_list.as_ptr().as_ref().unwrap().cursor_back();
    let mut score_board = Scoreboard {
      list: rc_list,
      first,
      second,
      length: 2,
    };
    while score_board.length < RECEIPES_COUNT + 10 {
      score_board.create();
    }
    let mut list = score_board.list.into_inner();
    let recipes = list.split_off(RECEIPES_COUNT);
    let scores = recipes.iter().take(10).fold(String::from(""), |mut acc, x| {
      acc.push_str(&x.to_string());
      acc
    });
    println!("Part 1: the scores of the ten recipes immediately after the number of recipes is {}.", scores);
  }
}
