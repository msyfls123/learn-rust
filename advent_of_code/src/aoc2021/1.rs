
use advent_of_code::get_str_array_from_file;

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2021", "data", "1.txt"});
    let nums: Vec<usize> = data.iter().map(|x| x.parse().unwrap()).collect();
    let increases_sum = nums.iter().scan(None, |prev, x| {
        match *prev {
            Some(prev_num) => {
                let result = x > prev_num;
                *prev = Some(x);
                Some(result)
            },
            None => {
                *prev = Some(x);
                Some(false)
            }
        }
    }).filter(|x| *x).count();
    println!("Part 1: {}", increases_sum);
}
