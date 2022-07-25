use std::collections::HashMap;

use advent_of_code::get_str_array_from_file;
use itertools::Itertools;

type CommonBitMap = HashMap<usize, isize>;

fn find_rating(
    bit_lists: &Vec<Vec<usize>>,
    index: usize,
    is_most: bool
) -> Vec<Vec<usize>> {
    if bit_lists.len() == 1 {
        return bit_lists.to_owned()
    }
    let zeroes = bit_lists.iter().filter_map(|bits| if bits[index] == 0 {
        Some(bits.to_owned())
    } else { None }).collect_vec();
    let ones = bit_lists.iter().filter_map(|bits| if bits[index] == 1 {
        Some(bits.to_owned())
    } else { None }).collect_vec();
    let target = if is_most {
        if ones.len() >= zeroes.len() {
            ones
        } else {
            zeroes
        }
    } else {
        if zeroes.len() <= ones.len() {
            zeroes
        } else {
            ones
        }
    };
    find_rating(&target.to_owned(), index + 1, is_most)
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2021", "data", "3.txt"});
    let numbers  = data.iter().map(|line| {
        line.split("").filter(|&x| x != "").map(|c| c.parse::<usize>().unwrap()).collect_vec()
    }).collect_vec();
    let mut common_map: CommonBitMap = HashMap::new();
    for nums in numbers.clone() {
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

    let oxygen_rating = find_rating(&numbers.clone(), 0, true);
    let co2_rating = find_rating(&numbers.clone(), 0, false);
    let oxygen_num = isize::from_str_radix(&oxygen_rating.first().unwrap().iter().join(""), 2).unwrap();
    let co2_num = isize::from_str_radix(&co2_rating.first().unwrap().iter().join(""), 2).unwrap();
    println!("Part 2: {}", oxygen_num * co2_num);
}
