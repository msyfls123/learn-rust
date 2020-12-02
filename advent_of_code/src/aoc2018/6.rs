use advent_of_code;
use std::collections::HashMap;

type Point = [i32; 2];

fn get_manhattan_distance(p1: Point, p2: Point) -> i32 {
  (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
}

fn main() {
  let array = advent_of_code::get_str_array_from_file(&vec!{"aoc2018", "data", "6.txt"});
  let points: Vec<Point> = array.iter().map(|text| {
    let point: Vec<i32> = text.split(", ").map(|x| x.parse::<i32>().unwrap()).collect();
    [point[0], point[1]]
  }).collect();
  let points_total = points.len();

  let [left, right, top, bottom] = points.iter().fold([i32::MAX, i32::MIN, i32::MAX, i32::MIN], |[left, right, top, bottom], point| {
    [
      left.min(point[0]),
      right.max(point[0]),
      top.min(point[1]),
      bottom.max(point[1]),
    ]
  });

  let mut locations: Vec<usize> = vec!{};
  let mut edge_points: Vec<usize> = vec!{};

  (left..right + 1).for_each(|x| {
    (top..bottom + 1).for_each(|y| {
      let nearest_distance = points.iter().map(|&p| get_manhattan_distance(p, [x, y])).min().unwrap();
      let nearest_points: Vec<usize> = points.iter().enumerate().filter_map(|(index, &p)| {
        if get_manhattan_distance(p, [x, y]) == nearest_distance {
          Some(index)
        } else {
          None
        }
      }).collect();
      let location = if nearest_points.len() == 1 {
        nearest_points[0]
      } else {
        points_total
      };
      if [left, right].contains(&x) || [top, bottom].contains(&y) {
        edge_points.push(location);
      }
      locations.push(location);
    })
  });
  edge_points.sort();
  edge_points.dedup();

  let finite_areas: Vec<usize> = locations.iter().filter(|x| {
    !edge_points.contains(x)
  }).map(|x| *x).collect();

  let largest_area_map: HashMap<usize, usize>  = finite_areas.iter().fold(HashMap::new(), |mut acc, &area| {
    let entry = acc.entry(area).or_insert(0);
    *entry += 1;
    acc
  });
  let largest_area = largest_area_map.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
  println!("Part 1: largest area size is {}.", largest_area.1)
}
