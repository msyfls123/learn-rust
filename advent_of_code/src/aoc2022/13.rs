use advent_of_code::get_group_str_from_file;
use serde_json::{self, Value};
use core::cmp::Ordering;

fn from_str(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

fn compare(val1: &Value, val2: &Value) -> Ordering {
    let isVal1Int = val1.is_number();
    let isVal2Int: bool = val2.is_number();
    let isVal1List: bool = val1.is_array();
    let isVal2List: bool = val2.is_array();

    if isVal1Int && isVal2Int {
        return val1.as_i64().unwrap().cmp(&val2.as_i64().unwrap());
    }

    if isVal1List && isVal2List {
        let list1 = val1.as_array().unwrap();
        let list2 = val2.as_array().unwrap();

        for (i, v1) in list1.iter().enumerate() {
            if list2.len() <= i {
                return Ordering::Greater;
            }
            let ord = compare(v1, &list2[i]);
            if ord != Ordering::Equal {
                return ord
            }
        }
        if list2.len() > list1.len() {
            return Ordering::Less;
        }
        return Ordering::Equal;
    }

    if isVal1Int {
        compare(&Value::Array(vec!{val1.to_owned()}), val2)
    } else {
        compare(val1, &Value::Array(vec!{val2.to_owned()}))
    }
}

fn cmp_str(str1: &str, str2: &str) -> Ordering {
    compare(&from_str(str1), &from_str(str2))
}

#[test]
fn test_cmp_str() {
    assert_eq!(cmp_str("[1,1,3,1,1]", "[1,1,5,1,1]"), Ordering::Less);
    assert_eq!(cmp_str("[[1],[2,3,4]]", "[[1],4]"), Ordering::Less);
    assert_eq!(cmp_str("[9]", "[[8,7,6]]"), Ordering::Greater);
    assert_eq!(cmp_str("[1,1,3,1,1]", "[1,1,3,1,1]"), Ordering::Equal);
    assert_eq!(cmp_str("[[[]]]", "[[]]"), Ordering::Greater);
}

fn main() {
    let data = get_group_str_from_file(&vec!{"aoc2022", "data", "13.txt"});

    let compare_results: usize = data.iter().map(|pair| {
        cmp_str(&pair[0], &pair[1])
    }).enumerate().filter_map(|(i, ord)| {
        if ord == Ordering::Less {
            Some(i + 1)
        } else {
            None
        }
    }).sum();

    println!("Part 1: {}", compare_results);
}