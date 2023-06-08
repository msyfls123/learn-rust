use std::{cell::RefCell, rc::Rc, fmt::Display};

struct Dir {
    name: String,
    subs: Vec<Rc<RefCell<Dir>>>,
    parent: Option<Rc<RefCell<Dir>>>,
}

impl Dir {
    fn add_sub(&mut self, sub: &Rc<RefCell<Dir>>) {
        self.subs.push(Rc::clone(sub));
    }

    fn create_sub(this: &Rc<RefCell<Self>>, sub_name: &str) -> Rc<RefCell<Dir>> {
        let sub = Rc::new(RefCell::new(Dir {
            name: sub_name.to_string(),
            subs: vec!{},
            parent: Some(Rc::clone(this)),
        }));
        this.as_ref().borrow_mut().subs.push(Rc::clone(&sub));
        sub

    }

    fn get_sub(&self, sub_name: &str) -> Option<&Rc<RefCell<Dir>>> {
        self.subs.iter().find(|s| s.as_ref().borrow().name == sub_name)
    }

    fn get_depth(&self) -> usize {
        match &self.parent {
            Some(parent) => parent.as_ref().borrow().get_depth() + 1,
            None => 0,
        }
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&"  ".repeat(self.get_depth()));
        writeln!(f, "- {} (dir)", self.name);
        for sub in &self.subs {
            write!(f, "{}", sub.as_ref().borrow());
        }
        write!(f, "")
    }
}

fn main() {
    let root = Rc::new(RefCell::new(Dir {
        name: "root".to_string(),
        subs: vec!{},
        parent: None,
    }));

    let sub_a = Dir::create_sub(&root, "sub_a");
    Dir::create_sub(&root, "sub_b");
    Dir::create_sub(&sub_a, "sub_sub_c");
    println!("{}", root.as_ref().borrow());
}
