// 1. 设定俩 strategy struct Two & Three
// 2. 按行读取文件装入 strategy 列表
// 3. 设定判断相同 pattern 的 trait（周边一圈数字正向反向循环找相等）
// 4. 设定初始 pattern
// 5. 根据 pattern 被 2 的整除结果 n，将其变成 n * n 个二维数组
// 6. 每个二维数组通过匹配 strategy 转化成对应的 pattern
// 7. 重复 5，6 步 5 次得到最终 pattern

use std::convert::TryInto;
use advent_of_code::get_str_array_from_file;

static T: bool = true;
static F: bool = false;

static TWO_INDEX: [[usize; 2]; 4] = [[0, 0], [0, 1], [1, 1], [1, 0]];
static THREE_INDEX: [[usize; 2]; 8] = [
    [0, 0],
    [0, 1],
    [0, 2],
    [1, 2],
    [2, 2],
    [2, 1],
    [2, 0],
    [1, 0],
];

type TwoDimension = [[bool; 2]; 2];
type ThreeDimension = [[bool; 3]; 3];
type FourDimension = [[bool; 4]; 4];

trait Match {
    type Item;
    fn compare(&self, item: Self::Item) -> bool;
}

fn compare_bool_vectors(vec1: &Vec<bool>, vec2: &Vec<bool>) -> bool {
    let len: isize = vec1.iter().count().try_into().unwrap(); 
    let mut result = false;
    for j in [-1, 1].iter() {
        for i in 0..len {
            result |= vec1.iter().enumerate().all(|(k, &v)| {
                let index = (((k as isize) * j + (i as isize) + len) % len) as usize;
                v == vec2[index]
            });
            if result {
                return result;
            }
        };
    };
    result
}

#[derive(Debug)]
struct Two {
    pattern: TwoDimension,
    result: ThreeDimension,
}

impl Two {
    fn new(item: &Vec<Vec<Vec<bool>>>) -> Self {
        let mut pattern: TwoDimension = [[F; 2]; 2];
        let mut result: ThreeDimension = [[F; 3]; 3];
        for (row1, row2) in pattern.iter_mut().zip(item[0].iter()) {
            for (column1, column2) in row1.iter_mut().zip(row2.iter()) {
                *column1 = *column2;
            };
        };
        for (row1, row2) in result.iter_mut().zip(item[1].iter()) {
            for (column1, column2) in row1.iter_mut().zip(row2.iter()) {
                *column1 = *column2;
            };
        };
        Two {
            pattern,
            result,
        }
    }
}

impl Match for Two {
    type Item = TwoDimension;
    fn compare(&self, item: Self::Item) -> bool {
        let circle: Vec<bool> = TWO_INDEX.iter().map(|&v| self.pattern[v[0]][v[1]]).collect();
        let circle2: Vec<bool> = TWO_INDEX.iter().map(|&v| item[v[0]][v[1]]).collect();
        compare_bool_vectors(&circle, &circle2)
    }
}

#[derive(Debug)]
struct Three {
    pattern: ThreeDimension,
    result: FourDimension,
}

impl Three {
    fn new(item: &Vec<Vec<Vec<bool>>>) -> Self {
        let mut pattern: ThreeDimension = [[F; 3]; 3];
        let mut result: FourDimension = [[F; 4]; 4];
        for (row1, row2) in pattern.iter_mut().zip(item[0].iter()) {
            for (column1, column2) in row1.iter_mut().zip(row2.iter()) {
                *column1 = *column2;
            };
        };
        for (row1, row2) in result.iter_mut().zip(item[1].iter()) {
            for (column1, column2) in row1.iter_mut().zip(row2.iter()) {
                *column1 = *column2;
            };
        };
        Three {
            pattern,
            result,
        }
    }
}

impl Match for Three {
    type Item = ThreeDimension;
    fn compare(&self, item: Self::Item) -> bool {
        if self.pattern[1][1] != item[1][1] {
            false
        } else {
            let circle: Vec<bool> = THREE_INDEX.iter().map(|&v| self.pattern[v[0]][v[1]]).collect();
            let circle2: Vec<bool> = THREE_INDEX.iter().map(|&v| item[v[0]][v[1]]).collect();
            compare_bool_vectors(&circle, &circle2)
        }
    }
}

#[derive(Debug)]
enum Rule {
    Two(Two),
    Three(Three),
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
    let cop2 = [[F, T], [T, T]];

    let three = Three {
        pattern: [
            [T, F, T],
            [F, T, F],
            [T, T, T],
        ],
        result: [
            [T, T, T, F],
            [F, F, T, T],
            [T, T, T, F],
            [F, F, T, T],
        ]
    };
    let cop3 = [
        [T, T, T],
        [F, F, F],
        [T, F, T],
    ];
    println!("Are we matched: {:?}", two.compare(cop2));
    println!("Are we matched: {:?}", three.compare(cop3));

    let array: Vec<Vec<Vec<Vec<bool>>>> =
      get_str_array_from_file(&vec!{"aoc2017", "day21_data.txt"})
        .iter()
        .map(|x| {
            x.split("=>").map(|v| {
                v.trim().split("/").map(|i| {
                    i.split("").filter(|s| !s.is_empty()).map(|j| {
                        j == "#"
                    }).collect()
                }).collect()
            }).collect()
        }).collect();
    let rules: Vec<Rule> = array.iter().map(|x| {
        match x[0].len() {
            2 => Rule::Two(Two::new(x)),
            3 => Rule::Three(Three::new(x)),
            _ => panic!()
        }
    }).collect();
    println!("{:?}", rules);
}
