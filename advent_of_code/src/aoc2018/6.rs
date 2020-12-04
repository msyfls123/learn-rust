use advent_of_code;
use std::collections::HashMap;

type Point = [i32; 2];

fn get_manhattan_distance(p1: Point, p2: Point) -> i32 {
  (p1[0] - p2[0]).abs() + (p1[1] - p2[1]).abs()
}

fn get_largest_area(
  points: &Vec<Point>,
  points_total: usize,
  &[left, right, top, bottom]: &[i32; 4]
) -> (usize, usize) {
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
  (*largest_area.0, *largest_area.1)
}

fn get_nearest_area_size(
  points: &Vec<Point>,
  &[left, right, top, bottom]: &[i32; 4],
  limit: i32
) -> usize {
  let top_line: Vec<Point> = (left..right).map(|x| [x, top]).collect();
  let right_line: Vec<Point> = (top..bottom).map(|y| [right, y]).collect();
  let bottom_line: Vec<Point> = (left + 1..=right).map(|x| [x, bottom]).collect();
  let left_line: Vec<Point> = (top + 1..=bottom).map(|y| [left, y]).collect();
  let all_lines = [top_line, right_line, bottom_line, left_line].concat();
  let points_within_region = all_lines.iter().filter(|&current| {
    let result = points.iter().fold(0, |acc, &point| {
      acc + get_manhattan_distance(*current, point)
    }) < limit;
    result
  }).count();
  if points_within_region > 0 {
    get_nearest_area_size(points, &[left - 1, right + 1, top - 1, bottom + 1], limit) + points_within_region
  } else {
    points_within_region
  }
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

  let largest_area = get_largest_area(
    &points,
    points_total,
    &[left, right, top, bottom],
  );

  println!("Part 1: largest area size is {}.", largest_area.1);

  let mid_x = (left + right) / 2;
  let mid_y = (top + bottom) / 2;

  let nearest_area_size = get_nearest_area_size(
    &points,
    &[mid_x, mid_x + 1, mid_y, mid_y + 1],
    10000,
  );

  println!("Part 2: the size of the region of nearest area is {}.", nearest_area_size);
}
