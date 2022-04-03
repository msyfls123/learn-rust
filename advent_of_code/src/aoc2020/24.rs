use std::{collections::HashMap};

use advent_of_code::get_str_array_from_file;

type Position = (isize, isize);
type Tiles = HashMap<Position, bool>;
type AdjacentMap = HashMap<Position, usize>;

const ADJACENT_LIST: [Position; 6] = [
  (2, 0),
  (-2, 0),
  (1, 1),
  (-1, 1),
  (1, -1),
  (-1, -1),
];

fn get_pos(text: &str, pos: Position) -> Position {
  match text.len() {
    0 => pos,
    1 => match text {
      "e" => get_pos(&text[1..], (pos.0 + 2, pos.1)),
      "w" => get_pos(&text[1..], (pos.0 - 2, pos.1)),
      _ => panic!("should not go here3"),
    },
    _ => match &text[0..2] {
      "ne" => get_pos(&text[2..], (pos.0 + 1, pos.1 + 1)),
      "nw" => get_pos(&text[2..], (pos.0 - 1, pos.1 + 1)),
      "se" => get_pos(&text[2..], (pos.0 + 1, pos.1 - 1)),
      "sw" => get_pos(&text[2..], (pos.0 - 1, pos.1 - 1)),
      "en" | "ee" | "ew" | "es" => get_pos(&text[1..], (pos.0 + 2, pos.1)),
      "wn" | "we" | "ww" | "ws" => get_pos(&text[1..], (pos.0 - 2, pos.1)),
      _ => panic!("should not go here1"),
    },
  }
}

fn get_adjacent_map(tiles: &Tiles) -> AdjacentMap {
  let mut map: AdjacentMap = HashMap::new();
  tiles.iter().filter(|(_p, &v)| v).for_each(|(r_pos, _v)| {
    ADJACENT_LIST.iter().for_each(|d_pos| {
      let pos = (r_pos.0 + d_pos.0, r_pos.1 + d_pos.1);
      let entry = map.entry(pos).or_insert(0);
      *entry += 1;
    });
  });
  map
}

fn flip(tiles: &mut Tiles) {
  let mut adjacent_map = get_adjacent_map(tiles);
  tiles.iter().filter(|(_p, &v)| v).for_each(|(&pos, _v)| {
    adjacent_map.entry(pos).or_insert(0);
  });
  adjacent_map.iter().for_each(|(&pos, &count)| {
    let entry = tiles.entry(pos).or_insert(false);
    if *entry {
      if count == 0 || count > 2 {
        *entry = false;
      }
    } else {
      if count == 2 {
        *entry = true;
      }
    }
  })
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "24.txt"});
  let mut tiles: Tiles = HashMap::new();
  data.iter().for_each(|text| {
    let pos = get_pos(text, (0, 0));
    let entry = tiles.entry(pos).or_insert(false);
    *entry = !*entry;
  });
  let black_tiles = tiles.values().filter(|&v| *v).count();
  println!("Part 1: {}", black_tiles);

  for _ in 1..=100 {
    flip(&mut tiles);
  }
  let black_tiles = tiles.values().filter(|&v| *v).count();
  println!("Part 2: {}", black_tiles);
}
