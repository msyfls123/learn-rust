#![feature(linked_list_cursors)]
use std::collections::{LinkedList, HashMap, BTreeMap};

use advent_of_code::get_group_str_from_file;
use itertools::Itertools;

type Polymer = LinkedList<char>;

type PairInsertions = HashMap<(char, char), char>;

fn get_polymer_template(line: &str) -> Polymer {
  let mut polymer = LinkedList::new();
  line.chars().for_each(|c| {
    polymer.push_back(c);
  });
  polymer
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

fn main() {
  let group = get_group_str_from_file(&vec!{"aoc2021", "data", "14.txt"});
  let mut polymer = get_polymer_template(&group[0][0]);
  let pair_insertions = get_pair_insertions(&group[1]);
  for _ in 0..10 {
    step(&mut polymer, &pair_insertions)
  }
  let great_diff = find_great_diff(&polymer);
  println!("Part 1: {}", great_diff);
}
