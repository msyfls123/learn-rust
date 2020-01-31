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

pub fn get_knot_hash(raw_string: &str) -> u128 {
    let mut data_list: Vec<i64> = (0..256).collect();
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
    data_list.chunks(16).fold(
        0u128,
        |mut result, x| {
            result <<= 8;
            result |= x.iter().fold(0, |acc, &v| acc ^ v as u8) as u128;
            result
        }
    )
}

pub fn main() {
    let mut data_list: Vec<i64> = (0..256).collect();
    let raw_list = vec![147,37,249,1,31,2,226,0,161,71,254,243,183,255,30,70];
    round(&mut data_list, &raw_list[..], 0, 0);
    println!("Resolve 1: {}", data_list[0] * data_list[1]);

    let result2 = get_knot_hash("147,37,249,1,31,2,226,0,161,71,254,243,183,255,30,70");
    println!("Resolve 2: {:032x}", result2);
}