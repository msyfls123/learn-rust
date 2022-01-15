extern crate regex;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use advent_of_code::get_group_str_from_file;
use regex::Regex;

type Border = Vec<bool>;

type TileId = usize;

#[derive(Debug)]
struct Tile {
  id: TileId,
  borders: Vec<Border>,
  matrix: Vec<Border>,
}

#[derive(Debug)]
struct Adjacent {
  tile_id: TileId,
  border: Border,
}

type AdjacentMap = HashMap<TileId, Vec<Adjacent>>;

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
      matrix,
      borders: vec!{top, right, bottom, left}
    }
  }

  fn adjacent(&self, other: &Self) -> Option<Border> {
    self.borders.iter().find_map(|border| {
      other.borders.iter().find_map(|other_border| {
        if border == other_border {
          Some(border.to_owned())
        } else {
          let mut rev_border = border.clone();
          rev_border.reverse();
          if &rev_border == other_border {
            Some(border.to_owned())
          } else {
            None
          }
        }
      })
    })
  }
}

fn main() {
  let data = get_group_str_from_file(&vec!{"aoc2020", "data", "20.txt"});
  let tiles: Vec<Tile> = data.iter().map(|g| Tile::from_texts(g)).collect();
  let tile_map: HashMap<_, _> = tiles.iter().map(|t| (t.id, t)).collect();
  let mut adjacent_map: AdjacentMap = HashMap::new();
  tiles.iter().enumerate().for_each(|(index, tile)| {
    let entry = adjacent_map.entry(tile.id).or_insert(vec!{});
    let mut adjacent_tiles = tiles.iter().filter_map(|other| {
      if tile.id != other.id {
        match tile.adjacent(other) {
          Some(border) => Some(Adjacent {
            tile_id: other.id,
            border
          }),
          None => None
        }
      } else {
        None
      }
    }).collect();
    entry.append(&mut adjacent_tiles);
  });
  let corner_adjacents = adjacent_map.iter().filter(|(_key, value)| &value.len() == &2).collect::<Vec<_>>();
  let corners: Vec<usize> = corner_adjacents.iter()
    .map(|(&key, _value)| key).collect();
  println!("Part 1: {}", &corners.iter().fold(1, |acc, v| acc * v));
}
