use std::collections::HashMap;

fn main() {

    let mut array = vec![1, 2, 3, 4, 5, 3, 6, 1, 2, 3, 1, 2, 3];
    array.push(3);

    let average = array.iter().fold(0, |sum, x| sum + x) as f64 / array.len() as f64;
    let mean = (array.iter()
        .fold(0.0, |sum, &x| {
            sum + (x as f64 - average) * (x as f64 - average)
        }) / array.len() as f64).sqrt();
    let mut sorted: Vec<_> = array.iter().cloned().collect();
    sorted.sort();
    let mut heap = HashMap::new();
    for item in &array {
        let count = heap.entry(item).or_insert(0);
        *count += 1
    };
    let mode = heap.iter().fold((0, 0), |(max_value, max_count), (value, count)| {
        if *count > max_count {
            (**value, *count)
        } else {
            (max_value, max_count)
        }
    });


    println!("average: {:.2}", average);
    println!("mean: {:.2}", mean);
    println!("{:?} middle is {}", sorted, &sorted[sorted.len() / 2]);
    println!("mode: {}", mode.0);
}