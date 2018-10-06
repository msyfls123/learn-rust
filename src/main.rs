struct Fibonacci {
    a: Option<i64>,
    b: Option<i64>,
}

impl Fibonacci {
    fn new() -> Self {
        Self {
            a: Some(0),
            b: Some(1),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        match self.a.take() {
            None => None,
            Some(result) => {
                let next = self.b.and_then(|b| result.checked_add(b));
                self.a = self.b;
                self.b = next;
                Some(result)
            }
        }
    }
}

fn main() {
    println!("hello, world!");
}

#[cfg(test)]
mod test {
    use super::Fibonacci;

    #[test]
    fn it_works() {
        let expected = &[0, 1, 1, 2, 3, 5, 8, 13];
        let actual:Vec<_> = Fibonacci::new().take(8).collect();
        assert_eq!(expected, &*actual);
    }
}