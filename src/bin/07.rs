#![allow(dead_code, unused_imports)]
use aoc::{get_input, report};
use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap, env::join_paths, rc::Rc};

// Since this puzzle could use a state machine, and I'm doing this to learn rust not learn algos
// I again chose to build this like a real program instead of code-golfing it
// It was difficult, but it forced me to contend with borrow-checker and lifetimes for the first time

#[derive(Debug)]
struct Dir {
    name: String,
    path: String,
    files: RefCell<HashMap<String, File>>,
    dirs: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn new(name: String, parent_path: Option<&str>) -> Self {
        Self {
            name: name.clone(),
            path: match parent_path {
                Some(p) => [p, &name].join("/").to_owned(),
                _ => name,
            },
            files: RefCell::new(HashMap::new()),
            dirs: RefCell::new(HashMap::new()),
        }
    }

    fn get_size(&self) -> u32 {
        self.files.borrow().values().map(|f| f.size).sum::<u32>()
            + self
                .dirs
                .borrow()
                .values()
                .map(|d| d.get_size())
                .sum::<u32>()
    }

    fn add_dir(&self, dir: Rc<Dir>) {
        self.dirs.borrow_mut().insert(dir.name.to_owned(), dir);
    }

    fn add_file(&self, file: File) {
        self.files.borrow_mut().insert(file.name.to_owned(), file);
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
        } else if line.is_empty() || line.starts_with("dir ") || line.starts_with("$ ls") {
            Self::NoOp
        } else if line.len() > 1 && line.chars().nth(0).unwrap().is_numeric() {
            let (size, name) = line.split(" ").collect_tuple().unwrap();
            Self::File(File {
                name: name.to_owned(),
                size: size.parse::<u32>().unwrap(),
            })
        } else {
            println!("lr parse {}", line);
            unreachable!("no lineread type")
        }
    }
}

#[derive(Debug)]
struct FileSystem {
    dirs: RefCell<HashMap<String, Rc<Dir>>>,
}

// Ownership for the directories must live somewhere accessible by the line-reader
// If it lived in main() using a Vec we would lose dirs when moving to root
// If it lived in the chain of dirs it would require walking the tree to get mut refs
// So it lives in this struct, which keeps them in an easily-accessible hash-map
impl FileSystem {
    fn new() -> Self {
        let root = Rc::new(Dir::new("/".to_owned(), None));
        let dirs = RefCell::new(HashMap::new());
        dirs.borrow_mut().insert(root.path.to_owned(), root);
        Self { dirs }
    }

    fn get_root(&self) -> Rc<Dir> {
        self.dirs.borrow().get("/").unwrap().clone()
    }

    fn get_dir(&self, path: &str) -> Rc<Dir> {
        self.dirs.borrow_mut().get(path).unwrap().clone()
    }

    fn create_dir(&self, parent: Rc<Dir>, dir_name: String) -> Rc<Dir> {
        let dir = Rc::new(Dir::new(dir_name.to_owned(), Some(&parent.path)));
        self.dirs
            .borrow_mut()
            .insert(dir.path.to_owned(), dir.clone());
        parent.add_dir(dir.clone());
        dir.clone()
    }

    fn dirs_vec(&self) -> Vec<Rc<Dir>> {
        self.dirs.borrow().values().map(|d| d.clone()).collect()
    }

    // fn create_file(&mut self, )
}

fn main() {
    let input = get_input("07");

    // for each line
    //      parse into type
    //      match
    //          move_up =>

    let fs = FileSystem::new();
    let mut dir_stack = vec![fs.get_root().path.clone()];

    for line in input.lines() {
        let line_read = LineRead::parse(line);

        match line_read {
            LineRead::MoveUp => {
                let _ = dir_stack.pop();
            }
            LineRead::MoveInto(dir_name) => {
                let parent_path = dir_stack.last().unwrap();
                let parent = fs.get_dir(parent_path);
                let dir = fs.create_dir(parent, dir_name);
                dir_stack.push(dir.path.clone());
            }
            LineRead::MoveRoot => {
                dir_stack.clear();
                dir_stack.push(fs.get_root().path.clone());
            }
            LineRead::NoOp => (),
            LineRead::File(file) => {
                let cd = &mut fs.get_dir(dir_stack.last().unwrap());
                cd.add_file(file);
            }
        }
    }

    // println!(
    //     "{:?}",
    //     fs.dirs_vec()
    //         .iter()
    //         .map(|d| (d.name.to_owned(), d.get_size()))
    //         .collect::<Vec<(String, u32)>>()
    // );

    let total_size = 70000000;
    let total_used = fs.get_root().get_size();
    let unused = total_size - total_used;
    let update_size = 30000000;
    let needed_size = update_size - unused;

    let a: u32 = fs
        .dirs_vec()
        .iter()
        .map(|d| d.get_size().clone())
        .filter(|d| d <= &(100_000 as u32))
        .sum();
    let b = fs
        .dirs_vec()
        .iter()
        .map(|d| d.get_size().clone())
        .filter(|d| d >= &needed_size)
        .sorted()
        .rev()
        .last()
        .unwrap();

    report(&a, &b);

    // uncomment once you have correct to support refactoring
    // assert_eq!(a, 1);
    // assert_eq!(b, 2);
}
