#![feature(linked_list_cursors)]
use std::collections::linked_list::{LinkedList, Cursor};
use std::cell::{RefCell};

static PUZZLE_INPUT: usize = 306281;
type List = LinkedList<u32>;
type Recipes = Vec<u32>;

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

  fn match_recipes(&self, recipes: &Recipes) -> bool {
    let n = recipes.len();
    if self.length < n {
      return false;
    }
    let borrowed = self.list.borrow();
    let mut cursor = borrowed.cursor_back();
    (0..n).all(|i| {
      if i > 0 {
        cursor.move_prev();
      }
      *cursor.current().unwrap() == recipes[n - 1 - i]
    })
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

  fn create(&mut self, recipes: &Recipes) -> Option<usize> {
    let scores = self.get_current_scores();
    let digits: Vec<u32> = (scores.0 + scores.1).to_string().chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut found_length: Option<usize> = None;
    digits.iter().for_each(|&x| {
      self.append(x);
      if self.match_recipes(recipes) {
        found_length = Some(self.length)
      }
    });
    self.forward(scores);
    found_length
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
    while score_board.length < PUZZLE_INPUT + 10 {
      score_board.create(&vec!{});
    }
    let mut list = score_board.list.into_inner();
    let recipes = list.split_off(PUZZLE_INPUT);
    let scores = recipes.iter().take(10).fold(String::from(""), |mut acc, x| {
      acc.push_str(&x.to_string());
      acc
    });
    println!("Part 1: the scores of the ten recipes immediately after the number of recipes is {}.", scores);
  }
  let raw_list: List = LinkedList::new();
  let digits = PUZZLE_INPUT.to_string().chars().map(|x| x as u32 - 48).collect();
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
    'outer: loop {
      let found_length = score_board.create(&digits);
      match found_length {
        Some(len) => {
          println!("Part 2: {} recipes appear on the scoreboard to the left of {}.", len - digits.len(), PUZZLE_INPUT);
          break 'outer;
        },
        _ => {}
      }
    }
  }
}
