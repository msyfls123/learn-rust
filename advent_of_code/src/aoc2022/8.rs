use std::collections::HashSet;

use advent_of_code::get_str_array_from_file;
use itertools::Itertools;

type Grid = Vec<Vec<usize>>;
type VisibleSet = HashSet<(usize, usize)>;

fn find_visible_line<'a, T>(list: T) -> Vec<usize>
where T: Iterator<Item=&'a usize> {
    let mut tallest = None;
    list.enumerate().fold(vec!{}, |mut visibles, (i, v)| { 

        if tallest.is_some() {
            if v > tallest.unwrap() {
                tallest = Some(v);
                visibles.push(i);
            }
        } else {
            tallest = Some(v);
            visibles.push(i);
        }
        visibles
    })
}

#[test]
fn test_find_visible_line() {
    let iter = vec!{1,3,2,4};
    assert_eq!(find_visible_line(iter.iter()), vec!{0,1,3});
    assert_eq!(find_visible_line(iter.iter().rev()), vec!{0});
}

fn find_visible_in_grid(grid: &Grid) -> VisibleSet {
    let mut set = HashSet::new();
    let row_count = grid.len();
    let column_count = grid[0].len();
    for y in 0..row_count {
        let row = &grid[y];
        let left_tallest = find_visible_line(row.iter());
        let right_tallest = find_visible_line(row.iter().rev()).iter().map(|v| column_count - 1 - v).collect();
        for x in [left_tallest, right_tallest].concat() {
            set.insert((x, y));
        }
    }
    for x in 0..column_count {
        let column: Vec<usize> = grid.iter().map(|row| row[x]).collect();
        let top_tallest = find_visible_line(column.iter());
        let bottom_tallest = find_visible_line(column.iter().rev()).iter().map(|v| row_count - 1 - v).collect();
        for y in [top_tallest, bottom_tallest].concat() {
            set.insert((x, y));
        }
    }
    set
}

#[test]
fn test_find_visible_in_grid() {
    let grid = vec!{
        vec![3,0,3,7,3],
        vec![2,5,5,1,2],
        vec![6,5,3,3,2],
        vec![3,3,5,4,9],
        vec![3,5,3,9,0],
    };
    assert_eq!(find_visible_in_grid(&grid).len(), 21);
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2022", "data", "8.txt"});
    let grid = data.iter().map(|row| {
        row.chars().map(|v| v.to_string().parse::<usize>().unwrap()).collect()
    }).collect();
    let visibles = find_visible_in_grid(&grid);
    println!("Part 1: {}", visibles.len());
}
