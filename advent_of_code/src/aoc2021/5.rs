#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;

use advent_of_code::get_str_array_from_file;
use itertools::Itertools;
use regex::Regex;

type Diagram = HashMap<(isize, isize), usize>;

#[derive(Debug)]
struct Line {
  start: (isize, isize),
  end: (isize, isize),
  is_orthogonal: bool,
  step: (isize, isize),
}

fn get_step(start: isize, end: isize) -> isize {
  if start == end {
    return 0
  }
  (end - start) / (end - start).abs()
}

fn get_line(text: &str) -> Line {
  lazy_static! {
    static ref RE_LINE: Regex = Regex::new(r"(?P<start_x>\d+),(?P<start_y>\d+)\s+->\s+(?P<end_x>\d+),(?P<end_y>\d+)").unwrap();
  }
  let [start_x, start_y, end_x, end_y] = [
    "start_x",
    "start_y",
    "end_x",
    "end_y",
  ].map(|name| RE_LINE.captures(text).unwrap().name(name).unwrap().as_str().parse::<isize>().unwrap());

  let step = (get_step(start_x, end_x), get_step(start_y, end_y));
  Line {
    start: (start_x, start_y),
    end: (end_x, end_y),
    step,
    is_orthogonal: step.0 == 0 || step.1 == 0,
  }
}

fn get_diagram(lines: &Vec<Line>, include_diagoal: bool) -> Diagram {
  let mut diagram: Diagram = HashMap::new();
  lines.iter().filter(|l| include_diagoal || l.is_orthogonal).for_each(|l| {
    (0..)
      .map(|count| (l.start.0 + l.step.0 * count, l.start.1 + l.step.1 * count))
      .take_while(|point| {
        // next point
        point != &(l.end.0 + l.step.0, l.end.1 + l.step.1)
      })
      .for_each(|point| {
        let entry = diagram.entry(point).or_insert(0);
        *entry += 1;
      });
  });
  diagram
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2021", "data", "5.txt"});
  let lines = data.iter().map(|t| get_line(t)).collect_vec();
  // println!("{:?}", lines);
  let orthogonal_diagram = get_diagram(&lines, false);
  let overlapping_points_count = orthogonal_diagram.values().filter(|&v| v > &1).count();
  println!("Part 1: {}", overlapping_points_count);
  let full_diagram = get_diagram(&lines, true);
  let overlapping_points_count = full_diagram.values().filter(|&v| v > &1).count();
  println!("Part 2: {}", overlapping_points_count);
}
