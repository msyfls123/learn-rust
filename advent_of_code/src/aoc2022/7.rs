use std::{cell::{RefCell}, rc::Rc, fmt::Display};

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    subs: Vec<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
    parent: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug, Clone)]
enum Node {
    Dir(Dir),
    File(File)
}

trait HasParent {
    fn get_parent(&self) -> Option<&Rc<RefCell<Node>>>;

    fn get_depth(&self) -> usize {
        match self.get_parent() {
            Some(parent) => parent.as_ref().borrow().get_depth() + 1,
            None => 0,
        }
    }
}

impl Dir {
    fn get_node(&self, sub_name: &str) -> Option<Rc<RefCell<Node>>> {
        self.subs.iter().find(|&s| {
            match s.borrow().to_owned() {
                Node::Dir(dir) => dir.name == sub_name,
                Node::File(file) => file.name == sub_name,
            }
        }).map(|x| x.to_owned())
    }

    
}

impl HasParent for Dir {
    fn get_parent(&self) -> Option<&Rc<RefCell<Node>>> {
        self.parent.as_ref()
    }
}

impl Node {
    fn create_sub_dir(this: &Rc<RefCell<Self>>, sub_name: &str) -> Rc<RefCell<Node>> {
        let sub = Dir {
            name: sub_name.to_string(),
            subs: vec!{},
            parent: Some(Rc::clone(this)),
        };
        let sub = Rc::new(RefCell::new(Node::Dir(sub)));
        match *this.borrow_mut() {
            Node::Dir(ref mut dir) => dir.subs.push(Rc::clone(&sub)),
            Node::File(_) => panic!("you should not add sub to file")
        }
        sub
    }

    fn create_sub_node(this: &Rc<RefCell<Self>>, sub_name: &str, size: usize) -> Rc<RefCell<Node>> {
        let file = File {
            name: sub_name.to_string(),
            size,
            parent: Some(Rc::clone(this)),
        };
        let sub = Rc::new(RefCell::new(Node::File(file)));
        match *this.borrow_mut() {
            Node::Dir(ref mut dir) => dir.subs.push(Rc::clone(&sub)),
            Node::File(_) => panic!("you should not add sub to file")
        }
        sub
    }
}

impl HasParent for File {
    fn get_parent(&self) -> Option<&Rc<RefCell<Node>>> {
        self.parent.as_ref()
    }
}

impl HasParent for Node {
    fn get_parent(&self) -> Option<&Rc<RefCell<Node>>> {
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
        for sub in &self.subs {
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

fn main() {
    let root = Rc::new(RefCell::new(Node::Dir(Dir {
        name: "root".to_string(),
        subs: vec!{},
        parent: None,
    })));

    let sub_a = Node::create_sub_dir(&root, "sub_a");
    Node::create_sub_dir(&root, "sub_b");
    Node::create_sub_dir(&sub_a, "sub_sub_c");
    Node::create_sub_node(&sub_a, "sub_sub_d", 999);
    println!("{}", (&*root).borrow());
}
