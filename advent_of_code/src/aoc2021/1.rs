
use advent_of_code::get_str_array_from_file;

fn get_increase_sum(nums: &Vec<usize>) -> usize {
    nums.iter().scan(None, |prev, x| {
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
    }).filter(|x| *x).count()
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2021", "data", "1.txt"});
    let nums: Vec<usize> = data.iter().map(|x| x.parse().unwrap()).collect();
    let increases_sum = get_increase_sum(&nums);
    println!("Part 1: {}", increases_sum);

    let sliding_windows: Vec<usize> = (0..nums.len() - 2).map(|x| {
        nums[x] + nums[x+1] + nums[x+2]
    }).collect();

    let increases_sum = get_increase_sum(&sliding_windows);
    println!("Part 2: {}", increases_sum);
}
