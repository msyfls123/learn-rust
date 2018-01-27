extern crate learn;
// use learn::url;
// use learn::print;
// use learn::thread;

fn main() {
    // url::run();
    // print::run();
    // thread::process();
    let iter = "123 123 321".split_whitespace();
    let data = iter.map(|x| x.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    println!("{:?}", data);
}