use itertools::Itertools;
use std::collections::HashMap;
use std::io::{stdin, BufRead};
use std::path::PathBuf;

struct Session {
    pwd: PathBuf,
    dirs: HashMap<PathBuf, u32>,
}

impl Session {
    fn new() -> Self {
        Self {
            pwd: PathBuf::new(),
            dirs: HashMap::new(),
        }
    }

    fn cd(&mut self, path: String) {
        match path.as_str() {
            ".." => {
                self.pwd.pop();
            }
            _ => self.pwd.push(path),
        };
    }

    fn add_file(&mut self, size: u32) {
        let mut dir = Some(self.pwd.as_path());
        while let Some(path) = dir {
            *self.dirs.entry(path.to_path_buf()).or_default() += size;
            dir = path.parent();
        }
    }
}

#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

impl From<&str> for Command {
    fn from(str: &str) -> Self {
        match &str[0..2] {
            "cd" => Self::Cd(str[3..].to_string()),
            "ls" => Self::Ls,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Folder {
    name: String,
    // folders: HashMap<String, Folder>,
    files: Vec<File>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            files: vec![],
            // folders: HashMap::new()
        }
    }
}

#[derive(Debug)]
struct File(u32);

#[derive(Debug)]
enum Item {
    Folder(Folder),
    File(u32),
}

impl From<&str> for Item {
    fn from(str: &str) -> Self {
        if str.starts_with("dir") {
            Item::Folder(Folder::new(str.split_once(" ").unwrap().1.to_string()))
        } else {
            // Item::File(File(str.split_once(" ").unwrap().0.parse().unwrap()))
            Item::File(str.split_once(" ").unwrap().0.parse().unwrap())
        }
    }
}

pub fn run() {
    let input = stdin().lock().lines().map(|line| line.unwrap()).join("\n");

    let commands = input
        .split("$ ")
        .skip(1) // skip empty split at start
        .map(|cmd| cmd.split_once('\n').unwrap_or((cmd, "")))
        .map(|(cmd, out)| (Command::from(cmd), out.trim()))
        .collect::<Vec<_>>();

    let mut session = Session::new();

    for (cmd, out) in commands {
        match cmd {
            Command::Cd(path) => session.cd(path),
            Command::Ls => out
                .split("\n")
                .map(Item::from)
                .for_each(|entry| match entry {
                    Item::Folder(_) => {}
                    Item::File(size) => session.add_file(size),
                }),
        }
    }

    const MAX_DIR_SIZE: u32 = 100_000;

    const TOTAL_SPACE: u32 = 70_000_000;
    const REQUIRED_FREE: u32 = 30_000_000;

    let free_space = TOTAL_SPACE - session.dirs.get(PathBuf::from("/").as_path()).unwrap();
    let to_free = REQUIRED_FREE - free_space;

    let res = session
        .dirs
        .values()
        .filter(|size| **size >= to_free)
        .min()
        .unwrap();

    println!("{res}");
}
