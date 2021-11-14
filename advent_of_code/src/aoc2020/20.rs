extern crate regex;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use advent_of_code::get_group_str_from_file;
use regex::Regex;

type Border = Vec<bool>;

#[derive(Debug)]
struct Tile {
  id: usize,
  borders: Vec<Border>
}

type AdjacentMap = HashMap<usize, usize>;

impl Tile {
  fn from_texts<T: AsRef<str>>(texts: &[T]) -> Self {
    lazy_static! {
      static ref RE_ID: Regex = Regex::new(r"Tile\s(?P<id>\d+):").unwrap();
    }
    let id = RE_ID.captures(&texts[0].as_ref()).unwrap()
      .name("id").unwrap().as_str().parse::<usize>().unwrap();
    let matrix: Vec<Border> = texts[1..].iter().map(|l| {
      l.as_ref().chars().map(|c| c == '#').collect()
    }).collect();
    let len = matrix.len();
    let (top, bottom): (Border, Border) = (0..len).map(|x| {
      (matrix[0][x], matrix[len - 1][x])
    }).unzip();
    let (left, right): (Border, Border) = (0..len).map(|y| {
      (matrix[y][0], matrix[y][len - 1])
    }).unzip();
    Tile {
      id,
      borders: vec!{top, right, bottom, left}
    }
  }

  fn adjacent(&self, other: &Self) -> bool {
    self.borders.iter().any(|border| {
      other.borders.iter().any(|other_border| {
        if border == other_border {
          true
        } else {
          let mut rev_border = border.clone();
          rev_border.reverse();
          &rev_border == other_border
        }
      })
    })
  }
}

fn main() {
  let data = get_group_str_from_file(&vec!{"aoc2020", "data", "20.txt"});
  let tiles: Vec<Tile> = data.iter().map(|g| Tile::from_texts(g)).collect();
  let mut map: AdjacentMap = HashMap::new();
  tiles.iter().enumerate().for_each(|(index, tile)| {
    let entry = map.entry(tile.id).or_insert(0);
    *entry += tiles.iter().filter(|other| {
      tile.id != other.id && tile.adjacent(other)
    }).count();
  });
  let corners: Vec<usize> = map.iter().filter(|&(_key, &value)| value == 2)
    .map(|(key, _value)| *key).collect();
  println!("Part 1: {}", &corners.iter().fold(1, |acc, v| acc * v));
}
