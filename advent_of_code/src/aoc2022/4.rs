#[macro_use] extern crate lazy_static;
extern crate regex;

use advent_of_code::get_str_array_from_file;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct Pair {
    section1: (usize, usize),
    section2: (usize, usize),
}

impl Pair {
    fn is_contained(&self) -> bool {
        let (s1, s2) = self.section1;
        let (s3, s4) = self.section2;
        (s1 <= s3 && s2 >= s4) || (s1 >= s3 && s2 <= s4)
    }
}

fn get_pair(text: String) -> Pair {
    lazy_static! {
        static ref RE_PAIR: Regex = Regex::new(r"(?P<s1>\d+)-(?P<s2>\d+),(?P<s3>\d+)-(?P<s4>\d+)").unwrap();
    }
    let captures = RE_PAIR.captures(&text).unwrap();
    let s1 = captures.name("s1").unwrap().as_str().parse().unwrap();
    let s2 = captures.name("s2").unwrap().as_str().parse().unwrap();
    let s3 = captures.name("s3").unwrap().as_str().parse().unwrap();
    let s4 = captures.name("s4").unwrap().as_str().parse().unwrap();
    Pair { section1: (s1, s2), section2: (s3, s4) }
}

#[test]
fn test_get_pair() {
    let pair = get_pair(String::from("2-8,3-7"));
    assert_eq!(pair, Pair { section1: (2, 8), section2: (3, 7)});
    assert_eq!(pair.is_contained(), true);
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2022", "data", "4.txt"});
    let pairs: Vec<Pair> = data.iter().map(|line| get_pair(line.to_string())).collect();
    let contained_pairs = pairs.iter().filter(|p| p.is_contained()).count();
    println!("Part 1: {}", contained_pairs);
}
