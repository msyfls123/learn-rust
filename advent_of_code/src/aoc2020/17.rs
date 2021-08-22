use std::collections::HashMap;

use advent_of_code::get_str_array_from_file;
use itertools::Itertools;

type Position = (isize, isize, isize);

type Universe = HashMap<Position, bool>;

fn get_neighbours(point: &Position) -> Vec<Position> {
    let (px, py, pz) = point.clone();
    (-1..=1)
        .flat_map(|x| (-1..=1).flat_map(move |y| (-1..=1).map(move |z| (px + x, py + y, pz + z))))
        .collect_vec()
}

fn get_neighbours_active_count(universe: &Universe, point: &Position) -> usize {
    get_neighbours(point)
        .iter()
        .filter(|&p| {
            if point == p {
                false
            } else {
                match universe.get(p) {
                    Some(&active) => active,
                    None => false,
                }
            }
        })
        .count()
}

fn get_active_points(universe: &Universe) -> Vec<Position> {
    universe
        .iter()
        .filter_map(|(&key, &value)| if value == true { Some(key) } else { None })
        .collect::<Vec<Position>>()
}

fn change_state(universe: &Universe, point: &Position) -> bool {
    let neighbour_active_cnt = get_neighbours_active_count(universe, point);
    // println!("cnt {}", neighbour_active_cnt);
    let should_active_remain_inactive = [2usize, 3usize].contains(&neighbour_active_cnt);
    let should_inactive_become_active = neighbour_active_cnt == 3;
    match universe.get(&point) {
        Some(&active) => {
            if active {
                should_active_remain_inactive
            } else {
                should_inactive_become_active
            }
        }
        None => should_inactive_become_active,
    }
}

fn main() {
    let state: Vec<Vec<bool>> = get_str_array_from_file(&vec!["aoc2020", "data", "17.txt"])
        .iter()
        .map(|line| line.chars().map(|x| x == '#').collect_vec())
        .collect();
    let mut universe: Universe = HashMap::new();
    state.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &v)| {
            universe.insert((x as isize, y as isize, 0), v);
        })
    });

    // Part 1
    let mut total = 6;
    while total > 0 {
        let active_points = get_active_points(&universe);
        let mut new_universe: Universe = HashMap::new();
        active_points.iter().for_each(|p| {
            get_neighbours(p).iter().for_each(|&point| {
                new_universe.insert(point, change_state(&universe, &point));
            })
        });
        universe = new_universe;
        total -= 1;
    }

    println!("Part 1: {}", get_active_points(&universe).len());
}
