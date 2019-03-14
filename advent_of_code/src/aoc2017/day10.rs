pub fn round(
    list: &mut Vec<i64>,
    lengths: &[usize],
    mut index: usize,
    mut skip_size: usize
) -> (usize, usize) {
    let data_length = list.len();
    let mut sliced_list: Vec<i64> = Vec::new();
    // let mut index = 0;
    // let mut skip_size = 0;
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
    (index, skip_size)
}

fn main() {
    let mut data_list: Vec<i64> = (0..256).collect();
    let raw_list = vec![147,37,249,1,31,2,226,0,161,71,254,243,183,255,30,70];
    round(&mut data_list, &raw_list[..], 0, 0);
    println!("Resolve 1: {}", data_list[0] * data_list[1]);

    let mut data_list: Vec<i64> = (0..256).collect();
    let raw_string = "147,37,249,1,31,2,226,0,161,71,254,243,183,255,30,70";
    let mut acsii_lengths: Vec<usize> = String::from(raw_string).chars().map(|x| x as usize).collect();
    let mut index = 0;
    let mut skip_size = 0;
    let extra = vec![17, 31, 73, 47, 23];
    acsii_lengths.extend(&extra);
    for _ in 0..64 {
        let (a, b) = round(&mut data_list, &acsii_lengths[..], index, skip_size);
        index = a;
        skip_size = b;
    };
    let result_vec: Vec<String> = data_list.chunks(16).map(|list| {
        format!("{:02x}", list.iter().fold(0, |acc, x| acc ^ x))
    }).collect();
    println!("Resolve 2: {}", result_vec.join(""));
}