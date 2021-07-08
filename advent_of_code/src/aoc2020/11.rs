use advent_of_code::get_str_array_from_file;
use num;

type Layout = Vec<Vec<String>>;

const DIRECTIONS: [(isize, isize); 8] = [
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, -1),
  (0, 1),
  (1, -1),
  (1, 0),
  (1, 1),
];

fn clamp(num: usize, min: usize, max: usize, diff: isize) -> usize {
  if diff < 0 {
    match num.checked_sub(1) {
      None => 0,
      Some(v) => num::clamp(v, min, max),
    }
  } else {
    num::clamp(num + (diff as usize), min, max)
  }
}

fn check_point_in_area(
  point: (isize, isize),
  area: (isize, isize, isize, isize),
) -> bool {
  let (y, x) = point;
  let (left_x, top_y, right_x, bottom_y) = area;
  x >= left_x
    && y >= top_y
    && x <= right_x
    && y <= bottom_y
}

fn get_num_of_visible_occupied_seats(
  layout: &Layout,
  seat: (usize, usize),
) -> usize {
  let area = (
    0isize,
    0isize,
    (layout[0].len() - 1) as isize,
    (layout.len() - 1) as isize,
  );
  DIRECTIONS.iter().filter(|(dir_y, dir_x)| {
    (1..).find_map(|step| {
      let new_y = (seat.0 as isize) + step * dir_y;
      let new_x = (seat.1 as isize) + step * dir_x;
      if check_point_in_area((new_y, new_x), area) {
        match &layout[new_y as usize][new_x as usize][..] {
          "L" => Some(false),
          "#" => Some(true),
          _ => None,
        }
      } else {
        Some(false)
      }
    }).unwrap()
  }).count()
}

fn get_num_of_occupied_seats_adjacent(
  layout: &Layout,
  seat: (usize, usize),
  log: bool,
) -> usize {
  let (seat_y, seat_x) = seat;
  let y_len = layout.len();
  let x_len = layout[0].len();
  let y_min = clamp(seat_y, 0, y_len - 1, -1);
  let y_max = clamp(seat_y, 0, y_len - 1, 1);
  let x_min = clamp(seat_x, 0, x_len - 1, -1);
  let x_max = clamp(seat_x, 0, x_len - 1, 1);
  (y_min..=y_max).flat_map(|y| {
    let res = (x_min..=x_max).map(|x| {
      if x != seat_x || y != seat_y {
        layout[y][x] == "#"
      } else {
        false
      }
    }).collect::<Vec<bool>>();
    if log {
      println!("{}, {} - {}, {}", y, x_min, x_max, res.iter().map(|&x| if x { "#" } else { "." }).collect::<Vec<&str>>().join(""));
    }
    res
  }).filter(|&x| x == true).count()
}

fn round(layout: &Layout) -> (Layout, usize) {
  let mut new_layout = layout.clone();
  let mut changed_count = 0usize;
  layout.iter().enumerate().for_each(|(y, line)| {
    line.iter().enumerate().for_each(|(x, seat)| {
      match &seat[..] {
        "#" => {
          if get_num_of_occupied_seats_adjacent(layout, (y, x), false) >= 4 {
            changed_count += 1;
            new_layout[y][x] = String::from("L")
          };
        },
        "L" => {
          if get_num_of_occupied_seats_adjacent(layout, (y, x), false) == 0 {
            changed_count += 1;
            new_layout[y][x] = String::from("#")
          }
        },
        _ => {}
      }
    });
  });
  (new_layout, changed_count)
}

fn visible_round(layout: &Layout) -> (Layout, usize) {
  let mut new_layout = layout.clone();
  let mut changed_count = 0usize;
  layout.iter().enumerate().for_each(|(y, line)| {
    line.iter().enumerate().for_each(|(x, seat)| {
      match &seat[..] {
        "#" => {
          if get_num_of_visible_occupied_seats(layout, (y, x)) >= 5 {
            changed_count += 1;
            new_layout[y][x] = String::from("L")
          };
        },
        "L" => {
          if get_num_of_visible_occupied_seats(layout, (y, x)) == 0 {
            changed_count += 1;
            new_layout[y][x] = String::from("#")
          }
        },
        _ => {}
      }
    });
  });
  (new_layout, changed_count)
}

fn print(layout: &Layout) {
  for line in layout {
    println!("{}", line.join(""));
  }
}

fn main() {
  let layout: Layout = get_str_array_from_file(&vec!{"aoc2020", "data", "11.txt"})
    .iter().map(|line| line.split("").filter(|&x| x != "").map(|x| x.to_owned()).collect()).collect();
  let stabilized_layout = (0..).scan((layout.clone(), 1), |state, i| {
    let result = round(&state.0);
    *state = result.clone();
    Some(result)
  }).find(|x| x.1 == 0).unwrap().0;
  let occupied_seat_count: usize = stabilized_layout.iter().map(|line| line.iter().filter(|&x| x == "#").count()).sum();
  println!("Part 1: {}", occupied_seat_count);
  let stabilized_layout = (0..).scan((layout, 1), |state, i| {
    let result = visible_round(&state.0);
    *state = result.clone();
    Some(result)
  }).find(|x| x.1 == 0).unwrap().0;
  let occupied_seat_count: usize = stabilized_layout.iter().map(|line| line.iter().filter(|&x| x == "#").count()).sum();
  println!("Part 2: {}", occupied_seat_count);
}
