use itertools::Itertools;

type Matrix<T> = Vec<Vec<T>>;

pub enum FlipAxis {
  Horizontal,
  Vertical,
}

pub fn clockwise<T: Clone>(matrix: &Matrix<T>, opposite: bool) -> Matrix<T> {
  let len_x = matrix[0].len();
  let len_y = matrix.len();
  let new_len_x = len_y;
  let new_len_y = len_x;
  (0..new_len_y).map(|y| {
    (0..new_len_x).map(|x| {
      if opposite {
        // (y, x): (0, 0) -> [0, len_x - 1] -> [x, len_x - 1 -y]
        matrix[x][len_x - 1 - y].to_owned()
      } else {
        // (y, x): (0, 0) -> [len_y - 1, 0] -> [len_y - 1 - x, y]
        matrix[len_y - 1 - x][y].to_owned()
      }
    }).collect()
  }).collect_vec()
}

pub fn flip<T: Clone>(matrix: &Matrix<T>, axis: FlipAxis) -> Matrix<T>{
  let len_x = matrix[0].len();
  let len_y = matrix.len();
  (0..len_y).map(|y| {
    (0..len_x).map(|x| {
      match axis {
        FlipAxis::Horizontal => matrix[len_y - 1 - y][x].to_owned(),
        FlipAxis::Vertical => matrix[y][len_x - 1 - x].to_owned(),
      }
    }).collect_vec()
  }).collect_vec()
}

#[cfg(test)]
mod matrix {
  use super::*;

  #[test]
  fn test_clockwise() {
    let origin = vec!{
      vec!{1, 2, 3},
      vec!{4, 5, 6},
    };
    let target_no_opposite = vec!{
      vec!{4, 1},
      vec!{5, 2},
      vec!{6, 3},
    };
    let target_opposite = vec!{
      vec!{3, 6},
      vec!{2, 5},
      vec!{1, 4},
    };
    assert_eq!(
      target_no_opposite,
      clockwise(&origin, false)
    );
    assert_eq!(
      target_opposite,
      clockwise(&origin, true)
    );
  }

  #[test]
  fn test_flip() {
    let origin = vec!{
      vec!{1, 2, 3},
      vec!{4, 5, 6},
      vec!{7, 8, 9},
    };
    let target_horizontal = vec!{
      vec!{7, 8, 9},
      vec!{4, 5, 6},
      vec!{1, 2, 3},
    };
    let target_vertical = vec!{
      vec!{3, 2, 1},
      vec!{6, 5, 4},
      vec!{9, 8, 7},
    }; 
    assert_eq!(
      target_horizontal,
      flip(&origin, FlipAxis::Horizontal)
    );
    assert_eq!(
      target_vertical,
      flip(&origin, FlipAxis::Vertical)
    );
  }
}

pub type Point<T> = (T, T);

pub struct Range<T> {
  pub min: Point<T>,
  pub max: Point<T>,
}

pub fn calc_range_of_points<T: PartialOrd + Ord + Copy>(points: &Vec<Point<T>>) -> Range<T> {
  let mut min_x = None;
  let mut max_x = None;
  let mut min_y = None;
  let mut max_y = None;
  for (x, y) in points {
    if min_x.is_none() || x < &min_x.unwrap() {
      min_x = Some(x.to_owned());
    }
    if max_x.is_none() || x > &max_x.unwrap() {
      max_x = Some(x.to_owned());
    }
    if min_y.is_none() || y < &min_y.unwrap() {
      min_y = Some(y.to_owned());
    }
    if max_y.is_none() || y > &max_y.unwrap() {
      max_y = Some(y.to_owned());
    }
  }
  Range {
    min: (min_x.unwrap(), min_y.unwrap()),
    max: (max_x.unwrap(), max_y.unwrap()),
  }
}

#[test]
fn test_calc_range_of_points() {
  let points = vec!{
    (1, 2),
    (3, 4),
    (5, 6),
  };
  let range = calc_range_of_points(&points);
  assert_eq!((1, 2), range.min);
  assert_eq!((5, 6), range.max);
}
