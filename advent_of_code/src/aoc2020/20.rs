extern crate regex;
#[macro_use] extern crate lazy_static;

use std::collections::HashMap;
use std::fmt;
use advent_of_code::get_group_str_from_file;
use itertools::Itertools;
use regex::Regex;
use advent_of_code::geometry::{clockwise, flip, FlipAxis};

type Border = Vec<bool>;
type TileId = usize;
type TileImage = Vec<Vec<TileId>>;
type TileMap = HashMap<TileId, Tile>;

#[derive(Debug, Clone)]
struct Tile {
  id: TileId,
  matrix: Vec<Border>,
}

// #[derive(Debug)]
struct Adjacent {
  tile_id: TileId,
  border: Border,
}

type AdjacentMap = HashMap<TileId, Vec<Adjacent>>;

#[derive(Debug)]
enum AdjacentEdge {
  Top,
  Left,
  Right,
  Bottom,
}

fn check_adjacent(border: &Border, target: &Border) -> bool {
  let mut rev_border = border.clone();
  rev_border.reverse();
  border == target || &rev_border == target
}

fn fmt_border(border: &Border) -> String {
  format!("Border: {}", border.iter().map(|&v| if v { '#' } else { '.' }).collect::<String>())
}

impl fmt::Display for Tile {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "Tile {} :", self.id);
    self.matrix.iter().for_each(|line| {
      writeln!(f, "{}", line.iter().map(|&v| if v { '#' } else { '.' }).collect::<String>());
    });
    write!(f, "========")
  }
}

impl fmt::Debug for Adjacent {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "Adjacent {}", self.tile_id);
    writeln!(f, "border {}", fmt_border(&self.border));
    write!(f, "========")
  }
}

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
    
    Tile {
      id,
      matrix,
    }
  }

  fn clockwise(&mut self, opposite: bool) {
    self.matrix = clockwise(&self.matrix, opposite);
  }

  fn flip(&mut self) {
    self.matrix = flip(&self.matrix, FlipAxis::Horizontal);
  }

  fn tweak_for_arrangement(&mut self, edge: AdjacentEdge, border: Border) -> bool {
    for _ in 0..4 {
      self.clockwise(false);
      if self.border(&edge) == border { return true; }
    }
    self.flip();
    for _ in 0..4 {
      self.clockwise(false);
      if self.border(&edge) == border { return true; }
    }
    println!("{} {:?} {:?}", self, edge, border);
    panic!("not found");
  }

  fn borders(&self) -> Vec<Border> {
    [
      AdjacentEdge::Top,
      AdjacentEdge::Right,
      AdjacentEdge::Bottom,
      AdjacentEdge::Left,
    ].iter().map(|x| self.border(x)).collect()
  }

  fn border(&self, edge: &AdjacentEdge) -> Border {
    let matrix = &self.matrix;
    let len = matrix.len();
    match edge {
      AdjacentEdge::Top => (0..len).map(|x| matrix[0][x]).collect(),
      AdjacentEdge::Bottom => (0..len).map(|x| matrix[len - 1][x]).collect(),
      AdjacentEdge::Left => (0..len).map(|y| matrix[y][0]).collect(),
      AdjacentEdge::Right => (0..len).map(|y| matrix[y][len - 1]).collect(),
    }
  }

  fn find_adjacent_border(&self, other: &Self) -> Option<Border> {
    self.borders().iter().find_map(|border| {
      other.borders().iter().find_map(|other_border| {
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

  fn find_adjacent_tile(&self, edge: AdjacentEdge, adjacent_map: &AdjacentMap) -> Option<(TileId, Border)> {
    let adjacents = adjacent_map.get(&self.id).unwrap();
    let target_border = self.border(&edge);
    adjacents.iter().find_map(|a| {
      if check_adjacent(&a.border, &target_border) {
        Some((a.tile_id, target_border.clone()))
      } else {
        None
      }
    })
  }
}

fn get_adjacent_map(tiles: &Vec<Tile>) -> AdjacentMap {
  let mut adjacent_map: AdjacentMap = HashMap::new();
  tiles.iter().for_each(|tile| {
    let entry = adjacent_map.entry(tile.id).or_insert(vec!{});
    let mut adjacent_tiles = tiles.iter().filter_map(|other| {
      if tile.id != other.id {
        match tile.find_adjacent_border(other) {
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
  adjacent_map
}

fn tweak_0x0(tile: &mut Tile, adjacent_map: &AdjacentMap) -> (TileId, Border) {
  let adjacents = adjacent_map.get(&tile.id).unwrap();
  let borders = tile.borders();
  let mut adjacent_index: Vec<usize> = adjacents
    .iter()
    .map(|a| borders.iter().position(|border| border == &a.border).unwrap())
    .collect();
  adjacent_index.sort();
  match (adjacent_index[0], adjacent_index[1]) {
    (0, 3) => {
      tile.clockwise(false);
      tile.clockwise(false);
    },
    (0, 1) => {
      tile.clockwise(false);
    },
    (1, 2) => {
      println!("ok");
    },
    (2, 3) => {
      tile.clockwise(true);
    },
    _ => panic!("not covered")
  };
  let right_border = tile.border(&AdjacentEdge::Right);
  adjacents.iter().find_map(|a| {
    if a.border == right_border {
      Some((a.tile_id, right_border.clone()))
    } else {
      let mut rev_border = a.border.clone();
      rev_border.reverse();
      if rev_border == right_border {
        Some((a.tile_id, right_border.clone())) 
      } else {
        None
      }
    }
  }).unwrap()
}

fn tweak_nxn(tile: &mut Tile, border: Border, adjacent_map: &AdjacentMap) -> Option<(TileId, Border)> {
  tile.tweak_for_arrangement(AdjacentEdge::Left, border);
  tile.find_adjacent_tile(AdjacentEdge::Right, adjacent_map)
}

fn tweak_nx0(tile: &mut Tile, border: Border, adjacent_map: &AdjacentMap) -> Option<(TileId, Border)> {
  tile.tweak_for_arrangement(AdjacentEdge::Top, border);
  tile.find_adjacent_tile(AdjacentEdge::Right, adjacent_map)
}

fn collect_tiles(
  tile_map: &mut TileMap,
  corners: &Vec<TileId>,
  adjacent_map: &AdjacentMap
) -> TileImage {
  let mut image: TileImage = vec!{vec!{}};
  let mut current = corners[0];
  let mut row = 0;
  let mut is_new_row = false;
  let mut count = 0;
  let mut border: Border = vec!{};
  let total = tile_map.len();
  while count < total {
    let tile = tile_map.get_mut(&current).unwrap();
    image[row].push(current);
    count += 1;
    // 0 x 0
    if row == 0 && count == 1 {
      let (next_id, next_border) = tweak_0x0(tile, adjacent_map);
      current = next_id;
      border = next_border;
    } else if is_new_row == true {
      // n x 0
      is_new_row = false;
      let (next_id, next_border) = tweak_nx0(tile, border.clone(), adjacent_map).unwrap();
      current = next_id;
      border = next_border;
    } else {
      match tweak_nxn(tile, border.clone(), adjacent_map) {
        // n x n
        Some((next_id, next_border)) => {
          current = next_id;
          border = next_border;
        },
        None => {
          let row_start_tile = tile_map.get_mut(&image[row][0]).unwrap();
          match row_start_tile.find_adjacent_tile(AdjacentEdge::Bottom, adjacent_map) {
            Some((next_id, next_border)) => {
              // new line
              row += 1;
              image.push(vec!{});
              is_new_row = true;
              current = next_id;
              border = next_border;
            },
            None => {
              // last one
              println!("last one");
            }
          };
        }
      }
    }
  }
  
  image
}

fn main() {
  let data = get_group_str_from_file(&vec!{"aoc2020", "data", "20.txt"});
  let tiles: Vec<Tile> = data.iter().map(|g| Tile::from_texts(g)).collect();
  let mut tile_map: TileMap = tiles.iter().map(|t| (t.id, t.clone())).collect();
  let adjacent_map = get_adjacent_map(&tiles);
  let corner_adjacents = adjacent_map.iter().filter(|(_key, value)| &value.len() == &2).collect::<Vec<_>>();
  let corners: Vec<TileId> = corner_adjacents.iter()
    .map(|(&key, _value)| key).collect();
  println!("Part 1: {}", &corners.iter().fold(1, |acc, v| acc * v));

  println!("Corners: {:?}", corners);
  let image = collect_tiles(&mut tile_map, &corners, &adjacent_map);
  println!("{:?}", image);
}
