#![allow(dead_code, unused_imports)]
use aoc::{get_input, report};
use itertools::Itertools;

fn main() {
    let input = get_input("03");
    let lines = input.lines();

    let a: usize = lines
        .clone()
        .map(|line| find_overlap(bifurcate(line)))
        .map(|c| value(c))
        .sum();

    let b: usize = lines
        .clone()
        .chunks(3)
        .into_iter()
        .map(find_overlap)
        .map(|c| value(c))
        .sum();

    report(&a, &b);

    assert_eq!(a, 8240);
    assert_eq!(b, 2587);
}

fn bifurcate(line: &str) -> [&str; 2] {
    let (left, right) = line.split_at(line.len() / 2);
    [left, right]
}

fn find_overlap<'a>(vals: impl IntoIterator<Item = &'a str>) -> char {
    let overlap = vals
        .into_iter()
        .map(|s| s.to_string())
        .reduce(|left, right| {
            left.chars()
                .filter(|c| right.chars().contains(&c))
                .collect::<String>()
        });
    overlap.unwrap().chars().next().unwrap()
}

fn value(c: char) -> usize {
    match c {
        'a'..='z' => c as usize - b'a' as usize + 1,
        'A'..='Z' => c as usize - b'A' as usize + 27,
        _ => unreachable!(),
    }
}
