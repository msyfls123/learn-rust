extern crate regex;
#[macro_use] extern crate lazy_static;

use advent_of_code::get_str_array_from_file;
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashMap, hash::Hash};

type RuleMap = HashMap<usize, Rule>;
type RuleLetters = Vec<String>;
type CachedRuleLettersMap = HashMap<usize, RuleLetters>;

#[derive(Debug)]
enum Rule {
  Pairs(Vec<Vec<usize>>),
  Letter(char)
}

fn get_rule(text: &str) -> (usize, Rule) {
  lazy_static! {
    static ref RE_PAIRS: Regex = Regex::new(r#"^(?P<index>\d+):\s(?P<pairs>.*)"#).unwrap();
    static ref RE_LETTER: Regex = Regex::new(r#"(?P<index>\d+):\s"(?P<letter>[ab])""#).unwrap();
  }
  match RE_LETTER.captures(text) {
    Some(cap) => {
      let index = cap.name("index").unwrap().as_str().parse().unwrap();
      let letter = cap.name("letter").unwrap().as_str().chars().nth(0).unwrap();
      return (index, Rule::Letter(letter))
    },
    None => {}
  };
  match RE_PAIRS.captures(text) {
    Some(cap) => {
      let index = cap.name("index").unwrap().as_str().parse().unwrap();
      let pairs_text = cap.name("pairs").unwrap().as_str();
      let pairs = pairs_text.split("|").into_iter().map(|pair| {
        pair.split(" ").into_iter().filter_map(|i| {
          match i {
            "" => None,
            n => Some(n.parse::<usize>().unwrap())
          }
        }).collect()
      }).collect();
      (index, Rule::Pairs(pairs))
    },
    None => panic!("noop")
  }
}

fn get_rule_letters(
  map: &RuleMap,
  cached: Rc<RefCell<CachedRuleLettersMap>>,
  index: usize,
) -> RuleLetters {
  match cached.borrow().get(&index) {
    Some(letters) => {
      return letters.to_owned();
    },
    None => {}
  };
  let rule = map.get(&index).unwrap();
  match rule {
    &Rule::Letter(c) => {
      vec!{c.to_string()}
    },
    Rule::Pairs(pairs) => {
      let letters: Vec<String> = pairs.iter().flat_map(|pair| {
        pair.iter().fold(vec!{String::from("")}, |acc: Vec<String>, &i| {
          let sub_letters = get_rule_letters(map, cached.clone(), i);
          sub_letters.iter().flat_map(|str| {
            acc.iter().map(|origin_str| {
              let mut new_str = origin_str.to_owned();
              new_str.push_str(str);
              new_str
            }).collect::<Vec<String>>()
          }).collect()
        })
      }).collect();
      cached.borrow_mut().insert(index, letters.to_owned());
      letters
    }
  }
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "19.txt"});
  let mut rule_map: RuleMap = HashMap::new();
  data.iter().for_each(|line| {
    let (index, rule) = get_rule(line);
    rule_map.insert(index, rule);
  });
  println!("{:?}", rule_map);
  let cached: Rc<RefCell<CachedRuleLettersMap>> = Rc::new(RefCell::new(HashMap::new()));
  let rule_letters = get_rule_letters(&rule_map, cached.clone(), 0);
  println!("cached {:?}", cached.clone());
}
