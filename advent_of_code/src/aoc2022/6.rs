use advent_of_code::get_str_from_file;

fn slide_window<T: Eq>(
    list: &Vec<T>,
    size: usize,
) -> Option<usize> {
    let mut start_idx = 0;
    let mut end_idx = 0;
    while end_idx <= list.len() - 1 {
        let next = &list[end_idx];
        let found_index = list[start_idx..end_idx].iter().position(|x| x == next);
        match found_index {
            Some(index) => {
                start_idx += index + 1;
            },
            None => {
                if end_idx - start_idx + 1 == size {
                    return Some(end_idx + 1);
                }
                end_idx += 1;
            }
        }
        // println!("{}: {}", start_idx, end_idx);
    }
    None
}

#[test]
fn test_slide_window() {
    assert_eq!(slide_window(&"bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect(), 4), Some(5));
    assert_eq!(slide_window(&"nppdvjthqldpwncqszvftbrmjlhg".chars().collect(), 4), Some(6));
    assert_eq!(slide_window(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect(), 4), Some(10));
    assert_eq!(slide_window(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect(), 4), Some(11));
}

#[test]
fn test_slide_window_long() {
    assert_eq!(slide_window(&"bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect(), 14), Some(23));
    assert_eq!(slide_window(&"nppdvjthqldpwncqszvftbrmjlhg".chars().collect(), 14), Some(23));
    assert_eq!(slide_window(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect(), 14), Some(29));
    assert_eq!(slide_window(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect(), 14), Some(26));
}

fn main() {
    let data = get_str_from_file(&vec!{"aoc2022", "data", "6.txt"});
    let res = slide_window(&data.chars().collect(), 4);
    println!("Part 1: {:?}", res);

    let res = slide_window(&data.chars().collect(), 14);
    println!("Part 2: {:?}", res);
}
