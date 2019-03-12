pub fn resolve_1(list: &mut Vec<i64>, lengths: &[usize]) {
    let data_length = list.len();
    let mut sliced_list: Vec<i64> = Vec::new();
    let mut index = 0;
    let mut skip_size = 0;
    for &length in lengths {
        for i in 0..length {
            sliced_list.push(list[(index + i) % data_length]);
        };
        sliced_list.reverse();
        for i in 0..length {
            let cursor = (index + i) % data_length;
            list[cursor] = sliced_list[i];
        };
        index = (index + skip_size + length) % data_length;
        skip_size += 1;
        sliced_list.clear();
    };
}

fn main() {
    let mut data_list: Vec<i64> = (0..256).collect();
    let lengths = [147, 37, 249, 1, 31, 2, 226, 0, 161, 71, 254, 243, 183, 255, 30, 70];
    resolve_1(&mut data_list, &lengths);
    println!("{}", data_list[0] * data_list[1]);
}