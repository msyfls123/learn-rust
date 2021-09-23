extern crate regex;
#[macro_use] extern crate lazy_static;

use advent_of_code::{get_group_str_from_file};
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashMap, hash::Hash};

type RuleMap = HashMap<usize, RuleRaw>;
type RuleLetters = Vec<String>;
type CachedRuleLettersMap = HashMap<usize, RuleLetters>;

#[derive(Debug)]
enum RuleRaw {
  Pairs(Vec<Vec<usize>>),
  Letter(char)
}

fn get_rule(text: &str) -> (usize, RuleRaw) {
  lazy_static! {
    static ref RE_PAIRS: Regex = Regex::new(r#"^(?P<index>\d+):\s(?P<pairs>.*)"#).unwrap();
    static ref RE_LETTER: Regex = Regex::new(r#"(?P<index>\d+):\s"(?P<letter>[ab])""#).unwrap();
  }
  match RE_LETTER.captures(text) {
    Some(cap) => {
      let index = cap.name("index").unwrap().as_str().parse().unwrap();
      let letter = cap.name("letter").unwrap().as_str().chars().nth(0).unwrap();
      return (index, RuleRaw::Letter(letter))
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
      (index, RuleRaw::Pairs(pairs))
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
    &RuleRaw::Letter(c) => {
      vec!{c.to_string()}
    },
    RuleRaw::Pairs(pairs) => {
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

fn part1(rules: &Vec<String>, msgs: &Vec<String>) {
  let mut rule_map: RuleMap = HashMap::new();
  rules.iter().for_each(|line| {
    let (index, rule) = get_rule(line);
    rule_map.insert(index, rule);
  });
  // println!("{:?}", rule_map);
  let cached: Rc<RefCell<CachedRuleLettersMap>> = Rc::new(RefCell::new(HashMap::new()));
  let rule_letters = get_rule_letters(&rule_map, cached.clone(), 0);
  let matched_message_count = msgs.iter().filter(|m| rule_letters.contains(m)).count();
  println!("Part 1: {:?}", matched_message_count);
}

/// copied from https://github.com/timvisee/advent-of-code-2020/blob/master/day19b/src/main.rs
enum Rule {
  Lit(u8),
  Seq(Vec<usize>),
  SeqOr(Vec<usize>, Vec<usize>),
}

fn matches_42(msg: &[u8], rules: &[Rule]) -> bool {
  (0..)
      .try_fold(msg, |msg, depth| match matches(msg, 42, rules) {
          Some(msg) if matches_31(depth, msg, rules) => Err(true),
          Some(msg) => Ok(msg),
          None => Err(false),
      })
      .err()
      .unwrap()
}

fn matches_31(depth: usize, msg: &[u8], rules: &[Rule]) -> bool {
  (0..depth)
      .try_fold(msg, |msg, _| match matches(msg, 31, rules) {
          Some(msg) if msg.is_empty() => Err(true),
          Some(msg) => Ok(msg),
          None => Err(false),
      })
      .err()
      .unwrap_or(false)
}

fn matches<'a>(msg: &'a [u8], rule: usize, rules: &[Rule]) -> Option<&'a [u8]> {
  match &rules[rule] {
      Rule::Lit(_) if msg.is_empty() => None,
      Rule::Lit(c) if &msg[0] == c => Some(&msg[1..]),
      Rule::Lit(_) => None,
      Rule::Seq(a) => a.into_iter().try_fold(msg, |m, &r| matches(m, r, rules)),
      Rule::SeqOr(a, b) => a
          .into_iter()
          .try_fold(msg, |m, &r| matches(m, r, rules))
          .or_else(|| b.into_iter().try_fold(msg, |m, &r| matches(m, r, rules))),
  }
}

fn part2(rules: &Vec<String>, msgs: &Vec<String>) {
  let mut rules: Vec<(usize, Rule)> = rules
        .iter()
        .map(|l| {
            let (n, rule) = l.split_once(": ").unwrap();
            (
                n.parse().unwrap(),
                if rule.starts_with('"') {
                    Rule::Lit(rule.chars().nth(1).unwrap() as u8)
                } else {
                    let parts: Vec<_> = rule.splitn(2, '|').collect();
                    let a = parts[0]
                        .split_terminator(' ')
                        .filter(|n| !n.is_empty())
                        .map(|n| n.parse().unwrap())
                        .collect();

                    if parts.len() == 1 {
                        Rule::Seq(a)
                    } else {
                        let b = parts[1]
                            .split_terminator(' ')
                            .filter(|n| !n.is_empty())
                            .map(|n| n.parse().unwrap())
                            .collect();
                        Rule::SeqOr(a, b)
                    }
                },
            )
        })
        .collect();
    rules.sort_unstable_by_key(|r| r.0);
    let rules: Vec<_> = rules.into_iter().map(|r| r.1).collect();

    // rules[8] = Rule::SeqOr(vec![42], vec![42, 8]);
    // rules[11] = Rule::SeqOr(vec![42, 31], vec![42, 11, 31]);

    println!(
        "Part 2: {}",
        msgs.iter()
            .filter(|msg| matches_42(msg.as_bytes(), &rules))
            .count(),
    );
}

/// copied end

fn main() {
  let data = get_group_str_from_file(&vec!{"aoc2020", "data", "19.txt"});
  part1(&data[0], &data[1]);
  part2(&data[0], &data[1]);
}
