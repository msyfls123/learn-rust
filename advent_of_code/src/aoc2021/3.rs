use std::collections::HashMap;

use advent_of_code::get_str_array_from_file;
use itertools::Itertools;

type CommonBitMap = HashMap<usize, isize>;

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2021", "data", "3.txt"});
    let numbers  = data.iter().map(|line| {
        line.split("").filter(|&x| x != "").map(|c| c.parse::<usize>().unwrap()).collect_vec()
    }).collect_vec();
    let mut common_map: CommonBitMap = HashMap::new();
    for nums in numbers {
        for  (index, &num) in nums.iter().enumerate() {
            let entry = common_map.entry(index).or_insert(0);
            if num == 0 {
                *entry -= 1;
            } else {
                *entry += 1;
            }
        }
    }
    let most_common = common_map.iter().sorted_by_key(|x| x.0).map(|x| {
        if x.1 > &0 { 1 } else { 0 }
    }).collect_vec();
    let least_common = most_common.iter().map(|x| 1 - x).collect_vec();
    let most_num = isize::from_str_radix(&most_common.iter().join(""), 2).unwrap();
    let least_num = isize::from_str_radix(&least_common.iter().join(""), 2).unwrap();
    let power_comsumption = most_num * least_num;
    println!("Part 1: {}", power_comsumption);
}