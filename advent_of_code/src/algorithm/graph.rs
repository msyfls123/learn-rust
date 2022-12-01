use std::{collections::HashSet, iter::FromIterator};

use itertools::Itertools;

type Edge<T> = (T, T);

fn traverse_edges<T: Eq + Sized + Clone, P> (
    visited: &Vec<T>,
    end: &T,
    edges: &P,
) -> Vec<Vec<T>>
where
    P: IntoIterator<Item=Edge<T>> + Clone,
{
    let last = visited.last().unwrap();
    if last == end {
        return vec!{visited.to_owned()};
    }
    let edges_c = edges.clone();
    edges_c.into_iter().filter_map(|(from, to)| {
        if &from == last && !visited.contains(&to) {
            Some(to)
        } else {
            None
        }
    }).flat_map(|vertex| {
        traverse_edges::<T, P>(&[visited.to_owned(), vec!{vertex.clone()}].concat(), end, &edges)
    }).collect()
}

pub fn get_pathes<T: Eq + Sized + Clone, P> (
    start: T,
    end: T,
    edges: &P,
) -> Vec<Vec<Edge<T>>>
where
    P: IntoIterator<Item=Edge<T>> + Clone,
{
    let all_pathes = traverse_edges(&vec!{start}, &end, edges);
    all_pathes.iter().map(|vertexes| {
        let len = vertexes.len();
        (0..len - 1).map(|i| {
            vertexes[i..i + 2].iter().map(|v| v.to_owned()).collect_tuple().unwrap()
        }).collect()
    }).collect()
}

pub fn get_shortest_path<T: Eq + Sized + Clone, P> (
    start: T,
    end: T,
    edges: &P,
) -> Option<Vec<Edge<T>>>
where
    P: IntoIterator<Item=Edge<T>> + Clone,
{
    if start == end {
        return Some(vec!{})
    }
    let all_pathes = get_pathes(start, end, edges);
    all_pathes
        .iter()
        .sorted_by(|a, b| Ord::cmp(&a.len(), &b.len()))
        .map(|v| v.to_owned())
        .next()
}

#[test]
fn test_get_pathes() {
    assert_eq!(
        get_shortest_path(1, 3, &vec!{(1,2), (2,3), (2,4), (4,3), (1,3)}),
        Some(vec!{(1,3)}),
    );
    let map: HashSet<(usize, usize)> = HashSet::from_iter(vec!{
        (1,2), (2,3), (2,4), (4,3)
    }.iter().map(|x| x.to_owned()));
    let result = get_pathes(1, 3, &map);
    assert!(result.contains(&vec!{(1,2), (2,3)}));
    assert!(result.contains(&vec!{(1,2), (2,4), (4,3)}));
}
