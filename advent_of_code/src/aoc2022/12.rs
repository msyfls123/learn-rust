use std::{cmp::Ordering, collections::{HashMap, BinaryHeap}};

use advent_of_code::{get_str_array_from_file, algorithm::get_siblings};

type Position = (usize, usize);
type HeightMap = HashMap<Position, char>;
type DistanceMap = HashMap<Position, usize>;

#[derive(Debug, Eq, PartialEq)]
struct Point {
    steps: Vec<Position>,
    position: Position,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.len().cmp(&self.steps.len())
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_height_map<T>(lines: &Vec<T>)-> HeightMap
where T: Into<String> + Clone  {
    let mut map = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        let str: String = line.clone().into();
        for (x, char) in str.char_indices() {
            map.insert((x + 1, y + 1), char);
        }
    }
    map
}

fn find_start(map: &HeightMap) -> Position {
    map.iter().find_map(|(pos, &val)| {
        if val == 'S' {
            Some(pos)
        } else {
            None
        }
    }).unwrap().to_owned()
}

fn shortest_path(map: &HeightMap, size: (usize, usize)) -> Option<usize> {
    let mut dist: DistanceMap = HashMap::new();
    for (&pos, _) in map {
        dist.insert(pos, usize::MAX);
    }

    let mut heap = BinaryHeap::new();
    let start = find_start(map);

    let start_entry =  dist.get_mut(&start).unwrap();
    *start_entry = 0;
    heap.push(Point{ steps: vec!{}, position: start });

    while let Some(Point { steps, position }) = heap.pop() {
        let val = if position == start {
            'a'
        } else {
            *map.get(&position).unwrap()
        };
        // println!("Current Position: {:?}", position);
        if val == 'E' {
            // println!("Found, {:?}", steps);
            return Some(steps.len())
        }

        if steps.len() > *dist.get(&position).unwrap() {
            continue;
        }

        let siblings = get_siblings(
            position.0, position.1, (1, size.0), (1, size.1)
        );

        // println!("siblings: {:?}", siblings);

        for neighborhood in siblings {
            let next_val = *map.get(&neighborhood).unwrap();
            // simply pass those higher elevation
            if next_val > val &&
                (next_val as u8) - (val as u8) > 1 {
                continue;
            }
            // check if next is end and then self is z or y
            if next_val == 'E' {
                if val != 'z' && val != 'y' {
                    continue
                }
            }

            let next = Point {
                steps: [steps.to_owned(), vec!{neighborhood}].concat(),
                position: neighborhood
            };

            if next.steps.len() < *dist.get(&next.position).unwrap() {
                let next_entry = dist.get_mut(&next.position).unwrap();
                *next_entry = next.steps.len();
                heap.push(next);
            }
        }
    }

    None
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2022", "data", "12.txt"});
    let map = parse_height_map(&data);
    let size = (data[0].len(), data.len());
    let steps = shortest_path(&map, size);
    println!("Part 1: {:?}", steps);
}
