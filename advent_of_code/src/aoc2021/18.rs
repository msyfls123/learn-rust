use std::{rc::Rc, cell::RefCell};

use itertools::Itertools;
use serde_json::Value;

#[derive(Debug)]
enum Node {
  Pair(Rc<RefCell<SnailFish>>),
  Number(Rc<RefCell<usize>>),
}

#[derive(Debug)]
struct SnailFish {
  left: Node,
  right: Node,
}

fn get_node(value: &Value) -> Node {
  if value.is_array() {
    let (left, right) = value.as_array().expect("to array fail")
      .iter().collect_tuple().expect("to tuple fail");
      let snailfish = SnailFish {
        left: get_node(left),
        right: get_node(right),
      };
    Node::Pair(Rc::new(RefCell::new(snailfish)))
  } else if value.is_u64() {
    let num = value.as_u64().expect("to u64 fail") as usize;
    Node::Number(Rc::new(RefCell::new(num)))
  } else {
    panic!("should never go here")
  }
}

fn parse_snailfish(text: &str) -> Node {
  let list: Value = serde_json::from_str(&text).expect("serde fail");
  get_node(&list)
}

enum WalkAction {
  Continue,
  Break,
  Explode(usize),
}

type Lucky = Option<Rc<RefCell<usize>>>;

fn walk(node: &Node, action: &WalkAction, depth: usize, lucky: Lucky) -> (WalkAction, Lucky) {
  let mut lucky = lucky;
  match action {  
    WalkAction::Explode(e_num) => {
      match node {
        Node::Pair(snailfish) => {
          (_, lucky) = walk(&snailfish.borrow().left, action, depth + 1, lucky);
          (_, lucky) = walk(&snailfish.borrow().right, action, depth + 1, lucky);
          (WalkAction::Explode(*e_num), lucky)
        },
        Node::Number(num) => {
          let mut num_mut = num.borrow_mut();
          if *num_mut == 9 {
            *num_mut += 1;
            return (WalkAction::Explode(*e_num), Some(num.clone()));
          }
          (WalkAction::Explode(*e_num), lucky)
        }
      }
    },
    _ => (WalkAction::Continue, lucky)
  }
}

fn main() {
  let snailfish = parse_snailfish("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
  println!("{:?}", snailfish);
  let (_, lucky) = walk(&snailfish, &WalkAction::Explode(2), 0, None);
  match lucky {
    Some(num) => {
      *num.borrow_mut() += 2;
    },
    None => {},
  }
  println!("{:?}", snailfish);
}