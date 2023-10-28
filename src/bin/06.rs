#![allow(dead_code, unused_imports)]
use aoc::{get_input, report};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn main() {
    let input = get_input("06");

    let a = find_marker(&input, 4);
    let b = find_marker(&input, 14);

    report(&a, &b);

    // uncomment once you have correct to support refactoring
    assert_eq!(a, 1702);
    assert_eq!(b, 3559);
}

fn find_marker(input: &str, len: usize) -> usize {
    input
        .chars()
        .enumerate()
        .find(|(i, _)| {
            *i > len - 1usize
                && input[i - (len - 1)..=*i]
                    .chars()
                    .unique()
                    .collect_vec()
                    .len()
                    == len
        })
        .unwrap()
        .0
        + 1
}
