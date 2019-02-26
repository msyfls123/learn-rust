use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn get_array(path: &Path) -> Vec<String> {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let array: Vec<String> = s.split("").filter_map(|x| {
        if x == "" {
            None
        } else {
            Some(x.to_string())
        }
    }).collect();
    array
}


pub fn resolve_1() {
    let path = Path::new("advent_of_code").join("src").join("aoc2017").join("day9_data.txt");
    let array = get_array(&path);
    let mut canceled = false;
    let mut inside_garbage = false;
    let mut bracket = 0;
    let mut total = 0;
    for item in array {
        match &item[..] {
            "{" => {
                if !inside_garbage {
                    bracket += 1;
                } else if canceled {
                    canceled = false;
                }
            },
            "}" => {
                if !inside_garbage {
                    total += bracket;
                    bracket -= 1;
                } else if canceled {
                    canceled = false;
                }
            },
            "<" => {
                if !inside_garbage {
                    inside_garbage = true;
                } else if canceled {
                    canceled = false;
                }
            },
            ">" => {
                if !canceled && inside_garbage {
                    inside_garbage = false;
                } else if inside_garbage {
                    canceled = false;
                }
            },
            "!" => {
                if !canceled && inside_garbage {
                    canceled = true;
                } else if inside_garbage {
                    canceled = false;
                }
            },
            _ => {
                if inside_garbage && canceled {
                    canceled = false;
                }
            }
        }
    };
    println!("{}", total);
}

fn main() {
    resolve_1()
}