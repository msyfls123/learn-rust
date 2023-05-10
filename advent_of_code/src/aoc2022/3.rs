use advent_of_code::get_str_array_from_file;

fn find_intersection_char<T: Into<String> + Clone>(str1: T, str2: T) -> Option<char> {
    let binding = str1.clone().into();
    let mut str1 = binding.chars();
    str1.find(|&a| {
        let binding = str2.clone().into();
        let mut str2 = binding.chars();
        str2.any(|b| a == b)
    })
}

#[test]
fn test_find_intersection_char() {
    let res = find_intersection_char(
        "vJrwpWtwJgWr",
        "hcsFMMfFFhFp",
    );
    assert_eq!(res, Some('p'));
}

fn find_same_item_per_rucksack<T: Into<String>>(str: T) -> Option<char> {
    let str: String = str.into();
    let len = str.len();
    let (str1, str2) = str.split_at(len / 2);
    find_intersection_char(str1, str2)
}

#[test]
fn test_find_same_item_per_rucksack() {
    assert_eq!(find_same_item_per_rucksack("vJrwpWtwJgWrhcsFMMfFFhFp"), Some('p'));
    assert_eq!(find_same_item_per_rucksack("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"), Some('L'));
    assert_eq!(find_same_item_per_rucksack("PmmdzqPrVvPwwTWBwg"), Some('P'));
}

fn priority<T: Into<String>>(str: T) -> Option<usize> {
    let res = find_same_item_per_rucksack(str);
    res.map(|v| {
        let num = v as u8 as usize;
        if num > 96 {
            num % 96
        } else {
            num % 64 + 26
        }
    })
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2022", "data", "3.txt"});
    let priorities = data.iter().map(|l| priority(l));

    let sum: usize = priorities.map(|x| x.unwrap_or_default()).sum();
    println!("Part 1: {}", sum);
}
