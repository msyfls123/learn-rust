use std::fmt::Debug;

#[derive(Debug)]
struct Monkey {
    id: usize,
    staring_items: Vec<usize>,
    operation: Box<dyn Operation>,
    test: Box<dyn  Test>,
    true_dest: usize,
    false_dest: usize,
}

trait Operation: Fn(usize) -> usize {}

impl<F> Operation for F where F: Fn(usize) -> usize { }

impl std::fmt::Debug for dyn Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation {{...}}")
    }
}

trait Test: Fn(usize) -> bool {}

impl<F> Test for F where F: Fn(usize) -> bool { }

impl std::fmt::Debug for dyn Test {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Test {{...}}")
    }
}

fn main() {
    let monkey = Monkey {
        id: 0,
        staring_items: vec!{1,2,3},
        operation: Box::new(|x| x * 2),
        test: Box::new(|x| x % 2 == 0),
        true_dest: 1,
        false_dest: 2,
    };
    println!("{:?}", monkey);
}