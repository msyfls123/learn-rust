#![feature(linked_list_cursors)]
use std::collections::{LinkedList, HashMap, BTreeMap};

use advent_of_code::get_group_str_from_file;
use itertools::Itertools;

type Polymer = LinkedList<char>;

type PolymerPairMap = HashMap<(char, char), usize>;

type PairInsertions = HashMap<(char, char), char>;

fn get_polymer_template(line: &str) -> Polymer {
  let mut polymer = LinkedList::new();
  line.chars().for_each(|c| {
    polymer.push_back(c);
  });
  polymer
}

fn get_polymer_pair_map(line: &str) -> PolymerPairMap {
  let mut map = HashMap::new();
  let length = line.len();
  line.char_indices().for_each(|(index, c)| {
    let next = line.chars().nth(index + 1);
    if next.is_some() {
      let pair = (c, next.unwrap());
      *map.entry(pair).or_insert(0) += 1;
    }
  });
  map
}

fn get_pair_insertions(lines: &Vec<String>) -> PairInsertions {
  let mut pair_insertions = HashMap::new();
  lines.iter().for_each(|l| {
    let (elements, item) = l.split(" -> ").collect_tuple().unwrap();
    let ele_chars = elements.chars().collect_tuple().unwrap();
    let item_char = item.chars().next().unwrap();
    pair_insertions.insert(ele_chars, item_char);
  });
  pair_insertions
}

fn step(polymer: &mut Polymer, pair_insertions: &PairInsertions) {
  let mut cursor = polymer.cursor_front_mut();
  let mut current = cursor.as_cursor().current();
  let mut next = cursor.as_cursor().peek_next();
  while current.is_some() && next.is_some() {
    let ele_chars = (current.unwrap().to_owned(), next.unwrap().to_owned());
    match pair_insertions.get(&ele_chars) {
      Some(item) => {
        cursor.insert_after(item.to_owned());
        cursor.move_next();
        cursor.move_next();
      },
      None => {
        cursor.move_next();
      }
    };
    current = cursor.as_cursor().current();
    next = cursor.as_cursor().peek_next();
  }
}

#[test]
fn test_step() {
  let mut polymer = get_polymer_template("NBCCNBBBCBHCB");
  let pair_insertions = get_pair_insertions(&r#"CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C"#.lines().into_iter().map(|t| t.trim().to_string()).collect());
  step(&mut polymer, &pair_insertions);
  let polymer_str = polymer.iter().join("");
  assert_eq!(&polymer_str, "NBBBCNCCNBBNBNBBCHBHHBCHB");
}

fn find_great_diff(polymer: &Polymer) -> usize {
  let mut alphabet_count = BTreeMap::new();
  for c in polymer.iter() {
    *alphabet_count.entry(c).or_insert(0) += 1;
  }
  let max = alphabet_count.values().max().unwrap();
  let min = alphabet_count.values().min().unwrap();
  max - min
}

fn pair_insert(map: &PolymerPairMap, rules: &PairInsertions) -> PolymerPairMap {
  let mut new_map = HashMap::new();
  map.iter().for_each(|(&pair, count)| {
    match rules.get(&pair) {
      Some(&item) => {
        *new_map.entry((pair.0, item)).or_insert(0) += count;
        *new_map.entry((item, pair.1)).or_insert(0) += count;
      },
      None => {
        *new_map.entry(pair).or_insert(0) += count;
      }
    }
  });
  new_map
}

fn calc_great_diff(pair_map: &PolymerPairMap, template: &str) -> usize {
  let mut char_map = HashMap::new();
  let first_char = template.chars().next().unwrap();
  let last_char = template.chars().last().unwrap();
  *char_map.entry(first_char).or_insert(0) += 1;
  *char_map.entry(last_char).or_insert(0) += 1;
  pair_map.iter().for_each(|(pair, count)| {
    *char_map.entry(pair.0).or_insert(0) += count;
    *char_map.entry(pair.1).or_insert(0) += count;
  });

  char_map.iter_mut().for_each(|(_, count)| *count /= 2);
  let max = char_map.values().max().unwrap();
  let min = char_map.values().min().unwrap();
  max - min
}

#[test]
fn test_calc_great_diff() {
  let template = "NNCB";
  let mut pair_map = get_polymer_pair_map(template);
  let pair_insertions = get_pair_insertions(&r#"CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C"#.lines().into_iter().map(|t| t.trim().to_string()).collect());
    for _ in 0..10 {
      pair_map = pair_insert(&mut pair_map, &pair_insertions);
    }
    assert_eq!(calc_great_diff(&pair_map, template), 1588);
}

fn main() {
  let group = get_group_str_from_file(&vec!{"aoc2021", "data", "14.txt"});
  let mut polymer = get_polymer_template(&group[0][0]);
  let pair_insertions = get_pair_insertions(&group[1]);
  for _ in 0..10 {
    step(&mut polymer, &pair_insertions)
  }
  let great_diff = find_great_diff(&polymer);
  println!("Part 1: {}", great_diff);

  let template = &group[0][0];
  let mut pair_map = get_polymer_pair_map(&template);
  for _ in 0..40 {
    pair_map = pair_insert(&mut pair_map, &pair_insertions);
  }
  println!("Part 2: {}", calc_great_diff(&pair_map, &template));
}
