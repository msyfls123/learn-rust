use std::collections::HashMap;

use advent_of_code::get_str_array_from_file;
use itertools::Itertools;

type Position = Vec<isize>;

type Universe = HashMap<Position, bool>;

fn get_neighbours(point: &Position) -> Vec<Position> {
    let dimension = point.len();
    let point_value = point[dimension - 1].clone();
    if dimension > 1 {
        get_neighbours(&point[0..dimension - 1].to_vec())
            .iter()
            .flat_map(|list| {
                (-1..=1).map(move |v| {
                    let mut new_list = list.clone();
                    new_list.push(point_value + v);
                    new_list
                })
            })
            .collect_vec()
    } else {
        (-1..=1).map(|v| vec![point_value + v]).collect_vec()
    }
}

#[test]
fn three_dimension() {
    assert_eq!(
        [
            [2, 3, 4],
            [2, 3, 5],
            [2, 3, 6],
            [2, 4, 4],
            [2, 4, 5],
            [2, 4, 6],
            [2, 5, 4],
            [2, 5, 5],
            [2, 5, 6],
            [3, 3, 4],
            [3, 3, 5],
            [3, 3, 6],
            [3, 4, 4],
            [3, 4, 5],
            [3, 4, 6],
            [3, 5, 4],
            [3, 5, 5],
            [3, 5, 6],
            [4, 3, 4],
            [4, 3, 5],
            [4, 3, 6],
            [4, 4, 4],
            [4, 4, 5],
            [4, 4, 6],
            [4, 5, 4],
            [4, 5, 5],
            [4, 5, 6]
        ]
        .iter()
        .map(|v| v.to_vec())
        .collect_vec(),
        get_neighbours(&vec! {3, 4, 5})
    )
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
        .filter_map(|(key, &value)| {
            if value == true {
                Some(key.clone())
            } else {
                None
            }
        })
        .collect::<Vec<Position>>()
}

fn change_state(universe: &Universe, point: &Position) -> bool {
    let neighbour_active_cnt = get_neighbours_active_count(universe, point);
    let should_active_remain_inactive = [2usize, 3usize].contains(&neighbour_active_cnt);
    let should_inactive_become_active = neighbour_active_cnt == 3;
    match universe.get(&point[..]) {
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

fn get_result(state: &Vec<Vec<bool>>, dimension: usize) -> usize {
    let mut universe: Universe = HashMap::new();
    state.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, &v)| {
            universe.insert(
                [vec![x as isize, y as isize], vec![0; dimension - 2]].concat(),
                v,
            );
        })
    });

    let mut total = 6;
    while total > 0 {
        let active_points = get_active_points(&universe);
        let mut new_universe: Universe = HashMap::new();
        active_points.iter().for_each(|p| {
            get_neighbours(p).iter().for_each(|point| {
                new_universe.insert(point.clone(), change_state(&universe, &point));
            })
        });
        println!("Round {}: {}", 6 - total, new_universe.len());
        universe = new_universe;
        total -= 1;
    }
    get_active_points(&universe).len()
}

fn main() {
    let state: Vec<Vec<bool>> = get_str_array_from_file(&vec!["aoc2020", "data", "17.txt"])
        .iter()
        .map(|line| line.chars().map(|x| x == '#').collect_vec())
        .collect();

    println!("Part 1: {}", get_result(&state, 3));
    println!("Part 2: {}", get_result(&state, 4));
}
