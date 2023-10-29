#![allow(dead_code, unused_imports)]
use aoc::{get_input, report};
use itertools::Itertools;
use std::collections::HashMap;

// Since this puzzle could use a state machine, and I'm doing this to learn rust not learn algos
// I again chose to build this like a real program instead of code-golfing it
// It was difficult, but it forced me to contend with borrow-checker and lifetimes for the first time

#[derive(Debug)]
struct Dir<'a> {
    name: String,
    files: HashMap<String, File>,
    dirs: HashMap<String, &'a Dir<'a>>,
}

impl<'a> Dir<'a> {
    fn new(name: String) -> Self {
        Self {
            name: name,
            files: HashMap::new(),
            dirs: HashMap::new(),
        }
    }

    fn get_size(&self) -> u32 {
        self.files.values().map(|f| f.size).sum::<u32>()
            + self.dirs.values().map(|d| d.get_size()).sum::<u32>()
    }

    fn add_dir(&'a mut self, dir: &'a Dir<'a>) {
        self.dirs.insert(dir.name.to_owned(), dir);
    }

    fn add_file(&mut self, file: File) {
        self.files.insert(file.name.to_owned(), file);
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

enum LineRead {
    MoveUp,
    MoveInto(String),
    MoveRoot,
    File(File),
    NoOp,
}

impl LineRead {
    fn parse(line: &str) -> Self {
        if line == "$ cd /" {
            Self::MoveRoot
        } else if line == "$ cd .." {
            Self::MoveUp
        } else if line.starts_with("$ cd ") {
            Self::MoveInto(line.replace("$ cd ", ""))
        } else if line.is_empty() || line.starts_with("dir ") || line.starts_with("ls") {
            Self::NoOp
        } else if line.len() > 1 && line.chars().nth(0).unwrap().is_numeric() {
            let (size, name) = line.split(" ").collect_tuple().unwrap();
            Self::File(File {
                name: name.to_owned(),
                size: size.parse::<u32>().unwrap(),
            })
        } else {
            unreachable!("no lineread type")
        }
    }
}

#[derive(Debug)]
struct FileSystem<'a> {
    dirs: HashMap<String, Dir<'a>>,
}

// Ownership for the directories must live somewhere accessible by the line-reader
// If it lived in main() using a Vec we would lose dirs when moving to root
// If it lived in the chain of dirs it would require walking the tree to get mut refs
// So it lives in this struct, which keeps them in an easily-accessible hash-map
impl<'a> FileSystem<'a> {
    fn new() -> Self {
        let root = Dir::new("/".to_owned());
        let mut dirs = HashMap::new();
        dirs.insert(root.name.to_owned(), root);
        Self { dirs }
    }

    fn get_root(&self) -> &'a Dir {
        self.dirs.get("/").unwrap()
    }

    fn create_dir(&mut self, parent: &'a mut Dir<'a>, dir_name: String) -> &'a Dir {
        let dir: Dir<'a> = Dir::new(dir_name.to_owned());
        self.dirs.insert(dir_name.to_owned(), dir);
        let dir_out = &self.dirs.get(dir_name.as_str()).unwrap();
        parent.add_dir(&dir_out);
        &dir_out
    }

    // fn create_file(&mut self, )
}

fn main() {
    let input = get_input("07");

    // for each line
    //      parse into type
    //      match
    //          move_up =>

    let mut fs = FileSystem::new();
    let mut dir_stack = vec![fs.get_root()];

    for line in input.lines() {
        let line_read = LineRead::parse(line);

        match line_read {
            LineRead::MoveUp => {
                let _ = dir_stack.pop();
            }
            LineRead::MoveInto(dir) => {
                let parent = &mut dir_stack.last().unwrap();
                let dir = fs.create_dir(parent, dir);
                dir_stack.push(dir);
            }
            LineRead::MoveRoot => {
                dir_stack.clear();
                dir_stack.push(fs.get_root());
            }
            LineRead::NoOp => (),
            LineRead::File(file) => {
                let cd = &mut dir_stack.last().unwrap();
                cd.add_file(file);
            }
        }
    }

    println!("{:?}", fs);

    let a = 1;
    let b = 2;

    report(&a, &b);

    // uncomment once you have correct to support refactoring
    // assert_eq!(a, 1);
    // assert_eq!(b, 2);
}
