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
