// 1. 设定俩 Rule struct Two & Three
// 2. 按行读取文件装入 Rule 列表
// 3. 设定判断相同 pattern 的 trait（周边一圈数字正向反向循环找相等）
// 4. 设定初始 pattern
// 5. 根据 pattern 被 2 或 3 的整除结果 n，将其变成 n * n 个二维数组
// 6. 每个二维数组通过匹配 Rule 转化成对应的 pattern，拼接成 result 赋值给 pattern
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
type Pattern = Vec<Vec<bool>>;

trait Match {
    type Item;
    fn compare(&self, item: Self::Item) -> bool;
}

fn compare_bool_vectors(vec1: &Vec<bool>, vec2: &Vec<bool>) -> bool {
    let len: isize = vec1.iter().count().try_into().unwrap(); 
    let mut result = false;
    let step = len / 4;
    for j in [-1, 1].iter() {
        result |= (0..len).step_by(step as usize).any(|i| {
            vec1.iter().enumerate().all(|(k, &v)| {
                let index = (((k as isize) * j + (i as isize) + len) % len) as usize;
                v == vec2[index]
            })
        });
    };
    result
}

fn vec_to_2d_array(vector: &Vec<Vec<bool>>) -> TwoDimension {
    let mut array: TwoDimension = [[F; 2]; 2];
    for (row1, row2) in array.iter_mut().zip(vector.iter()) {
        for (column1, column2) in row1.iter_mut().zip(row2.iter()) {
            *column1 = *column2;
        };
    };
    array
}

fn vec_to_3d_array(vector: &Vec<Vec<bool>>) -> ThreeDimension {
    let mut array: ThreeDimension = [[F; 3]; 3];
    for (row1, row2) in array.iter_mut().zip(vector.iter()) {
        for (column1, column2) in row1.iter_mut().zip(row2.iter()) {
            *column1 = *column2;
        };
    };
    array
}

fn vec_to_4d_array(vector: &Vec<Vec<bool>>) -> FourDimension {
    let mut array: FourDimension = [[F; 4]; 4];
    for (row1, row2) in array.iter_mut().zip(vector.iter()) {
        for (column1, column2) in row1.iter_mut().zip(row2.iter()) {
            *column1 = *column2;
        };
    };
    array
}

#[derive(Debug)]
struct Two {
    pattern: TwoDimension,
    result: ThreeDimension,
}

impl Two {
    fn new(item: &Vec<Vec<Vec<bool>>>) -> Self {
        let pattern = vec_to_2d_array(&item[0]);
        let result = vec_to_3d_array(&item[1]);
        Two {
            pattern,
            result,
        }
    }
    fn print(&self) -> String {
        let mut str = String::new();
        self.pattern.iter().for_each(|row| {
            row.iter().for_each(|&x| {
                if x {
                    str.push_str("#");
                } else {
                    str.push_str(".");
                }
            });
            str.push_str("\n");
        });
        str
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
        let pattern = vec_to_3d_array(&item[0]);
        let result = vec_to_4d_array(&item[1]);
        Three {
            pattern,
            result,
        }
    }

    fn print(&self) -> String {
        let mut str = String::new();
        self.pattern.iter().for_each(|row| {
            row.iter().for_each(|&x| {
                if x {
                    str.push_str("#");
                } else {
                    str.push_str(".");
                }
            });
            str.push_str("\n");
        });
        str
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

fn slice_to_vec(pattern: &Pattern, row: usize, column: usize, step: usize) -> Pattern {
    let mut vector: Pattern = vec!{};
    for i in row..row+step {
        // for j in column..column+step {
        //     vector[i - row].push(pattern[i][j])
        // }
        vector.push(pattern[i][column..column + step].to_vec().clone());
    }
    vector
}

fn p_to_s(pattern: &Pattern) -> String {
    let mut str = String::new();
    pattern.iter().for_each(|row| {
        row.iter().for_each(|&x| {
            if x {
                str.push_str("#");
            } else {
                str.push_str(".");
            }
        });
        str.push_str("\n");
    });
    str
}

fn transform(pattern: &Vec<Vec<bool>>, rules: &Vec<Rule>) -> Vec<Vec<bool>> {
    let len = pattern.len();
    let mut result: Pattern;
    match len % 2 {
        0 => {
            let from = 2;
            let to = 3;
            let size = len / 2;
            let available_rules: Vec<&Two> = rules.iter().filter_map(|r| {
                match r {
                    Rule::Two(expr) => Some(expr),
                    _ => None,
                }
            }).collect();
            let mut pattern_group: Vec<Vec<Pattern>> = (0..size).map(|_| vec!{}).collect();
            for i in 0..size {
                for j in 0..size {
                    pattern_group[i].push(
                        slice_to_vec(pattern, i * from, j * from, from)
                    )
                }
            };
            result = (0..size * to).map(|_| vec!{}).collect();
            pattern_group.iter().enumerate().for_each(|(i, p_row)| {
                p_row.iter().for_each(|p| {
                    available_rules.iter().any(|r| {
                        if r.compare(vec_to_2d_array(p)) {
                            // println!("p:\n{}\nrule:\n{}", p_to_s(p), r.print());
                            for k in 0..to {
                                let mut single_result = r.result[k].to_vec().clone();
                                result[i * to + k].append(&mut single_result);
                            }
                            return true
                        };
                        false
                    });
                })
            })
        },
        1 => {
            let from = 3;
            let to = 4;
            let size = len / 3;
            let available_rules: Vec<&Three> = rules.iter().filter_map(|r| {
                match r {
                    Rule::Three(expr) => Some(expr),
                    _ => None,
                }
            }).collect();
            let mut pattern_group: Vec<Vec<Pattern>> = (0..size).map(|_| vec!{}).collect();
            for i in 0..size {
                for j in 0..size {
                    pattern_group[i].push(
                        slice_to_vec(pattern, i * from, j * from, from)
                    )
                }
            };
            result = (0..size * to).map(|_| vec!{}).collect();
            pattern_group.iter().enumerate().for_each(|(i, p_row)| {
                p_row.iter().for_each(|p| {
                    available_rules.iter().any(|r| {
                        if r.compare(vec_to_3d_array(p)) {
                            for k in 0..to {
                                let mut single_result = r.result[k].to_vec().clone();
                                result[i * to + k].append(&mut single_result);
                            }
                            return true
                        };
                        false
                    });
                })
            })
        },
        _ => {
            result = vec!{};
        },
    };
    result
}
fn sum(pattern: &Pattern) -> usize {
    pattern.iter().map(|row| {
        row.iter().map(|&v| {
            if v {
                1
            } else {
                0
            }
        }).sum::<usize>()
    }).sum()
}

fn main() {
    let mut pattern = vec![
        vec![F, T, F],
        vec![F, F, T],
        vec![T, T, T]
    ];

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
    let count = 18;
    (0..count).for_each(|index| {
        pattern = transform(&pattern, &rules);
        println!("Round {}:\n", index + 1);
        if index == 4 {
        let total = sum(&pattern);
            println!("Part 1: {}", total);
        }
    });
    let total = sum(&pattern);
    println!("Part 2: {}", total);
}
