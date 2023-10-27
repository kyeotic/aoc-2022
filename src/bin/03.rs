#![allow(dead_code, unused_imports)]
use aoc::{get_input, report};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::BTreeMap, ops::Index};

lazy_static! {
    static ref MAP: BTreeMap<char, i32> = {
        let mut map: BTreeMap<char, i32> = BTreeMap::new();

        for (i, x) in ('a'..='z').enumerate() {
            map.insert(x, i as i32 + 1);
        }

        for (i, x) in ('A'..='Z').enumerate() {
            map.insert(x, i as i32 + 27);
        }

        map
    };
}

fn main() {
    let input = get_input("03");
    let lines = input.lines();

    let a: i32 = lines
        .clone()
        .map(|line| find_overlap(bifurcate(line)))
        .map(|c| get_priority(&c))
        .sum();

    let b: i32 = lines
        .clone()
        .chunks(3)
        .into_iter()
        .map(find_overlap)
        .map(|c| get_priority(&c))
        .sum();

    report(a, b);
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

fn get_priority(c: &char) -> i32 {
    *MAP.get(&c).unwrap()
}
