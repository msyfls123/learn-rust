pub fn get_siblings(
    x: usize,
    y: usize,
    x_range: (usize, usize),
    y_range: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut siblings = vec!{};
    if x > x_range.0 {
        siblings.push((x - 1, y))
    }
    if x < x_range.1 {
        siblings.push((x + 1, y))
    }
    if y > y_range.0 {
        siblings.push((x, y - 1))
    }
    if y < y_range.1 {
        siblings.push((x, y + 1))
    }
    
    siblings
}

#[test]
fn test_get_siblings() {
    assert_eq!(
        get_siblings(0, 2, (0, 5), (0, 5)),
        vec!{(1, 2), (0, 1), (0, 3)}
    );
}
