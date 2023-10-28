#![allow(dead_code, unused_imports)]
use aoc::{get_input, report};
use itertools::Itertools;

#[derive(Copy, Clone)]
struct Range {
    start: i32,
    end: i32,
}

// How does rust not have a type like this?
impl Range {
    fn new(start: i32, end: i32) -> Self {
        Self { start, end }
    }
    fn size(&self) -> i32 {
        &self.end - &self.start
    }
}

fn main() {
    let input = get_input("04");

    let a = input
        .clone()
        .lines()
        .map(parse_pair)
        .filter(|(a, b)| &a.start <= &b.start && &a.end >= &b.end)
        .collect::<Vec<_>>()
        .len();

    let b = input
        .clone()
        .lines()
        .map(parse_pair)
        .filter(|(a, b)| {
            (&a.start <= &b.start && &a.end >= &b.start) || (&a.start <= &b.end && &a.end >= &b.end)
        })
        .collect::<Vec<_>>()
        .len();

    report(&a, &b);
}

fn parse_pair(line: &str) -> (Range, Range) {
    let pairs = &line
        .split(",")
        .map(parse_range)
        .sorted_by(|a, b| Ord::cmp(&b.size(), &a.size()))
        .collect::<Vec<Range>>();

    (pairs[0], pairs[1])
}

fn parse_range(r: &str) -> Range {
    let parts = r
        .split("-")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    Range::new(parts[0], parts[1])
}
