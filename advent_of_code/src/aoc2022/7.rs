extern crate regex;
#[macro_use] extern crate lazy_static;

use std::{cell::{RefCell}, rc::Rc, fmt::Display, collections::HashMap};
use advent_of_code::get_str_array_from_file;
use regex::Regex;

/*
 * File System
 */

type INode = Rc<RefCell<Node>>;

type SizeMap = HashMap<String, usize>;

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    subs: RefCell<Vec<INode>>,
    parent: Option<INode>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
    parent: Option<INode>,
}

#[derive(Debug, Clone)]
enum Node {
    Dir(Dir),
    File(File)
}

trait HasParent {
    fn get_parent(&self) -> Option<INode>;

    fn get_depth(&self) -> usize {
        match self.get_parent() {
            Some(parent) => parent.as_ref().borrow().get_depth() + 1,
            None => 0,
        }
    }
}

impl Dir {
    fn get_node(&self, sub_name: &str) -> Option<INode> {
        self.subs.borrow().iter().find(|&s| {
            match s.borrow().to_owned() {
                Node::Dir(dir) => dir.name == sub_name,
                Node::File(file) => file.name == sub_name,
            }
        }).map(|x| Rc::clone(x))
    }

    /**
     * Holy shit..
     */
    fn get_full_path(&self) -> String {
        match &self.parent {
            Some(parent) => match *Rc::clone(&parent).borrow() {
                Node::Dir(ref par) => par.get_full_path() + "/" + &self.name,
                Node::File(_) => panic!("we should have sth wrong")
            },
            None => self.name.to_owned()
        }
    }
}

impl HasParent for Dir {
    fn get_parent(&self) -> Option<INode> {
        self.parent.as_ref().map(|p| Rc::clone(&p))
    }
}

impl Node {
    fn root() -> INode {
        Rc::new(RefCell::new(Node::Dir(Dir {
            name: "/".to_string(),
            subs: RefCell::new(vec!{}),
            parent: None,
        })))
    }

    fn create_sub_dir(this: &Rc<RefCell<Self>>, sub_name: &str) -> INode {
        let sub = Dir {
            name: sub_name.to_string(),
            subs: RefCell::new(vec!{}),
            parent: Some(Rc::clone(this)),
        };
        let sub = Rc::new(RefCell::new(Node::Dir(sub)));
        match *Rc::clone(&this).borrow() {
            Node::Dir(ref dir) => dir.subs.borrow_mut().push(Rc::clone(&sub)),
            Node::File(_) => panic!("you should not add sub to file")
        }
        
        sub
    }

    fn create_sub_node(this: &Rc<RefCell<Self>>, sub_name: &str, size: usize) -> INode {
        let file = File {
            name: sub_name.to_string(),
            size,
            parent: Some(Rc::clone(this)),
        };
        let sub = Rc::new(RefCell::new(Node::File(file)));
        match *Rc::clone(&this).borrow() {
            Node::Dir(ref dir) => dir.subs.borrow_mut().push(Rc::clone(&sub)),
            Node::File(_) => panic!("you should not add sub to file")
        }
        sub
    }

    fn get_node(&self, name: &str) -> Option<INode> {
        match self {
            Node::Dir(dir) => dir.get_node(name),
            Node::File(file) => panic!("file {} may not have sub node", file.name)
        }
    }

    fn get_size(&self) -> usize {
        match self {
            Node::Dir(dir) => dir.subs.borrow().iter().map(|sub| sub.borrow().get_size()).sum(),
            Node::File(file) => file.size,
        }
    }

    fn audit(&self, map: &mut SizeMap) -> usize {
        match self {
            Node::Dir(dir) => {
                let mut total = 0;
                for sub in &*dir.subs.borrow() {
                    let sub_size = sub.borrow().audit(map);
                    total += sub_size;
                }
                map.insert(dir.get_full_path(), total);
                total
            },
            Node::File(file) => file.size
        }
    }
}

impl HasParent for File {
    fn get_parent(&self) -> Option<INode> {
        self.parent.as_ref().map(|p| Rc::clone(&p))
    }
}

impl HasParent for Node {
    fn get_parent(&self) -> Option<INode> {
        match self {
            Node::Dir(dir) => dir.get_parent(),
            Node::File(file) => file.get_parent(),
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&"  ".repeat(self.get_depth()));
        writeln!(f, "- {} (dir)", self.name);
        for sub in &*self.subs.borrow() {
            write!(f, "{}", *sub.as_ref().borrow());
        }
        write!(f, "")
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&"  ".repeat(self.get_depth()));
        writeln!(f, "- {} (file)(size: {})", self.name, self.size);
        write!(f, "")
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Dir(dir) => write!(f, "{}", dir),
            Node::File(file) => write!(f, "{}", file),
        }
    }
}

/*
 * Parser
 */
#[derive(Debug, PartialEq, Eq)]
enum Command {
    CdRoot,
    CdParent,
    CdSub(String),
    List,
    Dir(String),
    File(String, usize),
}

fn read_line(
    line: &str
) -> Command {
    lazy_static! {
        static ref RE_FILE: Regex = Regex::new(r"(\d+)\s(\S+)").unwrap();
        static ref RE_DIR: Regex = Regex::new(r"dir\s(\S+)").unwrap();
    }
    match &line[..4] {
        "$ ls" => Command::List,
        "$ cd" => {
            let sub_name = &line[5..];
            match sub_name {
                "/" => Command::CdRoot,
                ".." => Command::CdParent,
                _ => Command::CdSub(String::from(sub_name))
            }
        },
        _ => {
            let cap_file = &RE_FILE.captures(line);
            let cap_dir = &RE_DIR.captures(line);
            if cap_file.is_some() {
                let caps = cap_file.as_ref().unwrap();
                let file_name = caps.get(2).map_or("", |m| m.as_str());
                let file_size = caps.get(1).map_or(0, |m| m.as_str().parse::<usize>().unwrap());
                Command::File(file_name.to_string(), file_size)
            } else if cap_dir.is_some() {
                let dir_name = cap_dir.as_ref().unwrap()[1].to_string();
                Command::Dir(dir_name)
            } else {
                panic!("it is impossible: {}", line);
            }
        }
    }
}

#[test]
fn test_read_line() {
    assert_eq!(
        read_line("$ cd /"),
        Command::CdRoot,
    );
    assert_eq!(
        read_line("$ cd .."),
        Command::CdParent,
    );
    assert_eq!(
        read_line("$ cd e"),
        Command::CdSub("e".to_string()),
    );
    assert_eq!(
        read_line("dir a"),
        Command::Dir("a".to_string()),
    );
    assert_eq!(
        read_line("14848514 b.txt"),
        Command::File("b.txt".to_string(), 14848514),
    );
}

/**
 * Traverse
 */

fn traverse(text: &Vec<String>) -> INode {
    let root = Node::root();
    let mut current = Rc::clone(&root);
    use Command::*;
    for command in text.iter().map(|t| read_line(t)) {
        // println!("{:?}", command);
        match command {
            CdRoot => {
                current = Rc::clone(&root);
            },
            CdParent => {
                current = Rc::clone(&current).borrow().get_parent().unwrap();
            },
            CdSub(name) => {
                current = Rc::clone(&current).borrow().get_node(&name).unwrap();
            },
            Dir(dirname) => {
                match Rc::clone(&current).borrow().get_node(&dirname) {
                    Some(_) => {},
                    None => {
                        Node::create_sub_dir(&Rc::clone(&current), &dirname);
                    }
                }
            },
            File(fname, fsize) => {
                match Rc::clone(&current).borrow().get_node(&fname) {
                    Some(_) => {},
                    None => {
                        Node::create_sub_node(&Rc::clone(&current), &fname, fsize);
                    }
                }
            },
            List => {},
        }
    }
    root
}

fn main() {
    // let root = Node::root();
    // let sub_a = Node::create_sub_dir(&root, "sub_a");
    // Node::create_sub_dir(&root, "sub_b");
    // Node::create_sub_dir(&sub_a, "sub_sub_c");
    // Node::create_sub_node(&sub_a, "sub_sub_d", 999);
    // println!("{}", (&*root).borrow());

    let data = get_str_array_from_file(&vec!{"aoc2022", "data", "7.txt"});
    let root = traverse(&data);
    println!("{}", root.borrow());
    let mut map: SizeMap = HashMap::new();
    root.borrow().audit(&mut map);
    let small_dir_space: usize = map.values().filter(|&x| x <= &100_000).sum();
    println!("Part 1: {}", small_dir_space);
}
