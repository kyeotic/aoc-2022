#![allow(dead_code, unused_imports)]
use aoc::{get_input, report};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MATCH_MOVE: Regex = Regex::new(r"(\d+).+?(\d+).+?(\d+)").unwrap();
}

fn main() {
    let input = get_input("05").to_owned();

    let a = Warehouse::parse(&input).sort(&input.clone()).get_tops();
    let b = Warehouse::parse(&input)
        .sort_9001(&input.clone())
        .get_tops();

    report(&a, &b);

    assert_eq!(a, String::from("JRVNHHCSJ"));
    assert_eq!(b, String::from("GNFBSBJLH"));
}

#[derive(Debug, Clone)]
struct Warehouse {
    stacks: Vec<Vec<char>>,
}

struct MoveOp(u32, u32, u32);

impl Warehouse {
    fn parse(lines: &str) -> Self {
        let stacks = lines
            .lines()
            // collect all the stack lines
            .fold_while(Vec::<&str>::new(), |mut vec, l| {
                if l.starts_with(" 1") {
                    Done(vec)
                } else {
                    vec.push(l);
                    Continue(vec)
                }
            })
            .into_inner()
            .iter()
            // reverse so that we build up stacks from the bottom
            .rev()
            .fold(Vec::<Vec<char>>::new(), |mut vec, l| {
                // 4 chars gives us each cell
                for (i, g) in l.chars().chunks(4).into_iter().enumerate() {
                    if vec.len() < i + 1 {
                        vec.push(Vec::<char>::new())
                    }
                    let c = g.into_iter().find(|c| c.is_alphanumeric());
                    if let Some(c) = c {
                        vec[i].push(c);
                    }
                }
                vec
            });

        Self { stacks }
    }

    fn sort(&mut self, op: &str) -> &mut Self {
        let moves = parse_moves(op);

        for MoveOp(n, start, end) in moves {
            for _ in 0..n {
                let item = self.stacks[start as usize - 1].pop().unwrap();
                let _ = &self.stacks[end as usize - 1].push(item);
            }
        }
        self
    }

    fn sort_9001(&mut self, op: &str) -> &mut Self {
        let moves = parse_moves(op);

        for MoveOp(n, start, end) in moves {
            let start_stack = &mut self.stacks[start as usize - 1];
            let tail = start_stack.split_off(start_stack.len() - n as usize);
            self.stacks[end as usize - 1].extend(tail);
        }
        self
    }

    fn get_tops(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.last().unwrap())
            .collect::<String>()
    }
}

fn parse_moves(operation: &str) -> Vec<MoveOp> {
    operation
        .lines()
        .filter(|l| l.contains("move"))
        // get just the numbers
        .map(extract_move)
        .collect_vec()
}

fn extract_move(line: &str) -> MoveOp {
    let (_, [n, start, end]) = MATCH_MOVE.captures(line).unwrap().extract();
    return MoveOp(
        n.parse::<u32>().unwrap(),
        start.parse::<u32>().unwrap(),
        end.parse::<u32>().unwrap(),
    );
}
