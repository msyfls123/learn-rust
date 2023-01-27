use std::{collections::HashMap, fmt::Debug};

use advent_of_code::get_str_array_from_file;

type Sea = HashMap<(usize, usize), bool>;

fn parse_sea<T>(lines: &Vec<T>) -> (Sea, (usize, usize))
where
T: Into<String> + Clone + Debug,
{
    let mut map = HashMap::new();
    let limit_y = lines.len();
    let mut limit_x = 0;
    for (y, line) in lines.iter().enumerate() {
        let line: String = line.clone().into();
        limit_x = line.len();
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                map.insert((x, y), char == 'v');
            }
        }
    }
    (map, (limit_x, limit_y))
}

fn step(
    sea: &Sea,
    limit_x: usize,
    limit_y: usize,
) -> (Sea, usize) {
    let mut new_sea = HashMap::new();
    let mut moves = 0;
    let downs: Vec<(usize, usize)> = sea.iter().filter_map(|(&pos, &is_down)| if is_down { Some(pos) } else { None }).collect();
    let rights: Vec<(usize, usize)> = sea.iter().filter_map(|(&pos, &is_down)| if !is_down { Some(pos) } else { None }).collect();
    for &pos in rights.iter() {
        let next_pos = ((pos.0 + 1) % limit_x, pos.1);
        if sea.contains_key(&next_pos) {
            new_sea.insert(pos, false);
        } else {
            moves += 1;
            new_sea.insert(next_pos, false);

        }
    }
    for &pos in downs.iter() {
        let next_pos = (pos.0, (pos.1 + 1) % limit_y);
        let is_empty = !new_sea.contains_key(&next_pos)
            && sea.get(&next_pos) != Some(&true);
        if is_empty {
            moves += 1;
            new_sea.insert(next_pos, true);
        } else {
            new_sea.insert(pos, true);
        }
    }
    (new_sea, moves)
}

#[test]
fn test_parse_sea() {
    let lines: Vec<&str> = r#"...>...
.......
......>
v.....>
......>
.......
..vvv.."#.lines().collect();
    let (map, _) = parse_sea(&lines);
    let down_count = map.values().filter(|&&v| v).count();
    let right_count = map.values().filter(|&&v| !v).count();
    assert_eq!(down_count, 4);
    assert_eq!(right_count, 4);
}

fn print_sea(sea: &Sea, limit_x: usize, limit_y: usize) {
    for y in 0..limit_y {
        for x in 0..limit_x {
            match sea.get(&(x, y)) {
                Some(&is_down) => {
                    if is_down {
                        print!("v")
                    } else {
                        print!(">")
                    }
                },
                None => print!(".")
            }
        }
        println!("");
    }
    println!("\n\n")
}

fn find_last_move(sea: &Sea, limit_x: usize, limit_y: usize) -> usize {
    let mut index = 0;
    let mut sea = sea.clone();
    let mut count = usize::MAX;
    
    while count != 0 {
        // print_sea(&sea, limit_x, limit_y);
        (sea, count) = step(&sea, limit_x, limit_y);
        index += 1;
    }
    index
}

fn main() {
    let lines = get_str_array_from_file(&vec!{"aoc2021", "data", "25.txt"});
    let (sea, (limit_x, limit_y)) = parse_sea(&lines);
    // println!("{} {}", limit_x, limit_y);
    let result = find_last_move(&sea, limit_x, limit_y);
    println!("Part 1: {}", result);
}
