// 1. 设定俩 strategy struct Two & Three
// 2. 按行读取文件装入 strategy 列表
// 3. 设定判断相同 pattern 的 trait（周边一圈数字正向反向循环找相等）
// 4. 设定初始 pattern
// 5. 根据 pattern 被 2 的整除结果 n，将其变成 n * n 个二维数组
// 6. 每个二维数组通过匹配 strategy 转化成对应的 pattern
// 7. 重复 5，6 步 5 次得到最终 pattern

use std::convert::TryInto;

trait Match {
    type Item;
    fn compare(&self, item: Self::Item) -> bool;
}

static T: bool = true;
static F: bool = false;
static TWO_INDEX: [[usize; 2]; 4] = [[0, 0], [0, 1], [1, 1], [1, 0]];

type TwoDimension = [[bool; 2]; 2];

#[derive(Debug)]
struct Two {
    pattern: TwoDimension,
    result: [[bool; 3]; 3],
}

impl Match for Two {
  type Item = TwoDimension;
  fn compare(&self, item: Self::Item) -> bool {
    let circle: Vec<bool> = TWO_INDEX.iter().map(|&v| self.pattern[v[0]][v[1]]).collect();
    let circle2: Vec<bool> = TWO_INDEX.iter().map(|&v| item[v[0]][v[1]]).collect();
    let len: isize = circle.iter().count().try_into().unwrap();
    let mut result = false;
    for j in [-1, 1].iter() {
        for i in 0..len {
            result |= circle.iter().enumerate().all(|(k, &v)| {
                let index = (((k as isize) * j + (i as isize) + len) % len) as usize;
                v == circle2[index]
            });
        };
    };
    result
  }
}

fn main() {
    let two = Two {
        pattern: [[T, F], [T, T]],
        result: [
            [T, F, T],
            [T, F, F],
            [F, T, T],
        ],
    };
    let cop = [[F, T], [T, T]];
    println!("Are we matched: {:?}", two.compare(cop));
}
