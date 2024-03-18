use std::{collections::{HashSet}, iter::FromIterator};

use advent_of_code::{geometry::{calc_range_of_points, Range}, get_str_array_from_file};
use itertools::Itertools;

type Position = (usize, usize);
type PointSet = HashSet<Position>;

const START_POINT: Position = (500, 0);

fn text_to_line (text: &str) -> Vec<Position> {
    let segments = text.split(" -> ");
    segments.map(|segment| {
       let (x, y) = segment.split(",").collect_tuple().unwrap();
       (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }).collect()
}

#[test]
fn test_text_to_line() {
    let text = "1,2 -> 3,4";
    let expected = vec![(1, 2), (3, 4)];
    assert_eq!(text_to_line(text), expected);
}

fn sort_x_y(x: usize, y: usize) -> (usize, usize) {
    if x < y {
        (x, y)
    } else {
        (y, x)
    }
}

fn path_to_set(text: &str) -> PointSet {
    let line = text_to_line(text);
    let mut set = HashSet::new();
    let mut prev = None;
    for point in line {
        if prev.is_none() {
            prev = Some(point);
        } else {
            let (x1, y1) = prev.unwrap();
            let (x2, y2) = point;
            if x1 == x2 {
                let (start, end) = sort_x_y(y1, y2);
                for y in start..=end {
                    set.insert((x1, y));
                }
            } else {
                let (start, end) = sort_x_y(x1, x2);
                for x in start..=end {
                    set.insert((x, y1));
                }
            }
            prev = Some(point);
        }
    }
    set
}

#[test]
fn test_path_to_set() {
    let text = "498,4 -> 498,6 -> 496,6";
    assert_eq!(path_to_set(text).len(), 5);
    let text = "503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(path_to_set(text).len(), 15);
}

fn build_map(texts: &Vec<String>) -> PointSet {
    let mut map = HashSet::new();
    for line in texts {
        map.extend(path_to_set(line));
    }
    map
}

fn fall(map: &mut PointSet, range: &Range<usize>) -> bool {
    let mut sand = START_POINT;
    while sand.1 <= range.max.1 {
        let down = (sand.0, sand.1 + 1);
        let down_left = (sand.0 - 1, sand.1 + 1);
        let down_right = (sand.0 + 1, sand.1 + 1);
        if !map.contains(&down) {
            sand = down;
            continue;
        } else if !map.contains(&down_left) {
            sand = down_left;
            continue;
        } else if !map.contains(&down_right) {
            sand = down_right;
            continue;
        } else {
            map.insert(sand);
            return false;
        }
    }
    return true;
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2022", "data", "14.txt"});
    let mut map = build_map(&data);
    let range = calc_range_of_points(&map.clone().into_iter().collect());
    let rest_sand_count = (0..).take_while(|_| {
        !fall(&mut map, &range)
    }).count();
    println!("Part 1: {}", rest_sand_count);
}
