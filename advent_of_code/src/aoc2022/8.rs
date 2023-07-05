use std::collections::HashSet;

use advent_of_code::get_str_array_from_file;

type Grid = Vec<Vec<usize>>;
type VisibleSet = HashSet<(usize, usize)>;

fn find_visible_line<'a, T>(list: T) -> Vec<usize>
where T: Iterator<Item=&'a usize> {
    let mut tallest = None;
    list.enumerate().fold(vec!{}, |mut visibles, (i, v)| { 

        if tallest.is_some() {
            let height = tallest.unwrap();
            if v > height {
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

fn find_visible_scenic_line<'a, T>(list: T, tree: usize) -> usize
where T: Iterator<Item=&'a usize> + ToOwned {
    let mut count = 0;
    for curr in list {
        count += 1;
        if curr >= &tree {
            break;
        }
    }
    count
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

fn scenic_score(grid: &Grid, tree: (usize, usize)) -> usize {
    let (x, y) = tree;
    let row = &grid[y];
    let column: Vec<usize> = grid.iter().map(|row| row[x]).collect();
    let tree = grid[y][x];
    let up = find_visible_scenic_line(column[0..y].iter().rev(), tree);
    let bottom = find_visible_scenic_line(column[y+1..].iter(), tree);
    let left = find_visible_scenic_line(row[0..x].iter().rev(), tree);
    let right = find_visible_scenic_line(row[x+1..].iter(), tree);
    up * bottom * left * right
}

#[test]
fn test_scenic_score() {
    let grid = vec!{
        vec![3,0,3,7,3],
        vec![2,5,5,1,2],
        vec![6,5,3,3,2],
        vec![3,3,5,4,9],
        vec![3,5,3,9,0],
    };
    assert_eq!(scenic_score(&grid, (2, 1)), 4);
    assert_eq!(scenic_score(&grid, (2, 3)), 8);
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2022", "data", "8.txt"});
    let grid = data.iter().map(|row| {
        row.chars().map(|v| v.to_string().parse::<usize>().unwrap()).collect()
    }).collect();
    let visibles = find_visible_in_grid(&grid);
    println!("Part 1: {}", visibles.len());

    let mut highest_score = 0;
    let row_count = grid.len();
    let column_count = grid[0].len();
    for x in 0..column_count {
        for y in 0..row_count {
            if visibles.contains(&(x, y)) {
                let score = scenic_score(&grid, (x, y));
                highest_score = highest_score.max(score);
            }
        }
    }
    println!("Part 2: {}", highest_score);
}
