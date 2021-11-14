use itertools::Itertools;

type Matrix<T> = Vec<Vec<T>>;

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
}
