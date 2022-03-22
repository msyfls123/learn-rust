use std::collections::HashMap;

use advent_of_code::get_str_array_from_file;

type Position = (isize, isize);

type Tiles = HashMap<Position, bool>;

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
}
