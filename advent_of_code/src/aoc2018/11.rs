use std::collections::HashMap;

static PUZZLE_INPUT: i32 = 9995;
static LENGTH: usize = 300;

fn calc_power_level(x: i32, y: i32, grid_serial_number: i32) -> i32 {
  let rack_id = x + 10;
  let mut power_level = rack_id * y;
  power_level += grid_serial_number;
  power_level *= rack_id;
  power_level = (power_level / 100) % 10;
  power_level - 5
}

fn main() {
  let grid: Vec<Vec<i32>> = (1..=LENGTH).map(|y| {
    (1..=LENGTH).map(|x| {
      calc_power_level(x as i32, y as i32, PUZZLE_INPUT)
    }).collect::<Vec<i32>>()
  }).collect();
  let result: (Option<(usize, usize)>, i32) = (0..usize::pow(LENGTH, 2)).fold((None, 0), |acc, index| {
    let x = index % LENGTH;
    let y = index / LENGTH;
    if x < LENGTH - 2 && y < LENGTH - 2 {
      let power: i32 = (x..=x + 2).map(|ix| {
        (y..=y + 2).map(|iy| {
          grid[iy][ix]
        }).sum::<i32>()
      }).sum();
      if power > acc.1 {
        return (Some((x + 1, y + 1)), power);
      }
    }
    acc
  });
  println!("Part 1: the X,Y,size identifier of the square with the largest total power is {:?}.", result.0.unwrap());

  let mut sum_table: HashMap<(usize, usize), i32> = HashMap::new();

  (0..LENGTH).for_each(|y| {
    (0..LENGTH).for_each(|x| {
      let sum: i32;
      if x == 0 && y == 0 {
        sum = grid[0][0];
      } else if x == 0 {
        sum = grid[y][0] + sum_table.get(&(y - 1, 0)).unwrap();
      } else if y == 0 {
        sum = grid[0][x] + sum_table.get(&(0, x - 1)).unwrap();
      } else {
        sum = grid[y][x]
          - sum_table.get(&(y - 1, x - 1)).unwrap()
          + sum_table.get(&(y, x - 1)).unwrap()
          + sum_table.get(&(y - 1, x)).unwrap();
      }
      sum_table.entry((y, x)).or_insert(sum);
    })
  });

  // Really slow...

  let result: (Option<(usize, usize, usize)>, i32) = (0..usize::pow(LENGTH, 2)).fold((None, 0), |mut acc, index| {
    let x = index % LENGTH;
    let y = index / LENGTH;
    let max_size = usize::min(LENGTH - 1 - x, LENGTH - 1 - y);
    (0..=max_size).for_each(|size| {
      let power = sum_table.get(&(y + size, x + size)).unwrap()
        + sum_table.get(&(y, x)).unwrap()
        - sum_table.get(&(y, x + size)).unwrap()
        - sum_table.get(&(y + size, x)).unwrap();
      if power > acc.1 {
        acc = (Some((x + 2, y + 2, size)), power);
      };
    });
    acc
  });
  println!("Part 2: the X,Y,size identifier of the square with the largest total power is {:?}.", result.0.unwrap());
}
