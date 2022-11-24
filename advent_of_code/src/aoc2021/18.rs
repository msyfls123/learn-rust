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
  left: RefCell<Node>,
  right: RefCell<Node>,
}

#[derive(Copy, Clone, PartialEq)]
enum WalkAction {
  Continue,
  Break,
  Add(usize),
  Explode(usize, usize),
}

type Lucky = Option<Rc<RefCell<usize>>>;

fn get_node(value: &Value) -> RefCell<Node> {
  if value.is_array() {
    let (left, right) = value.as_array().expect("to array fail")
      .iter().collect_tuple().expect("to tuple fail");
      let snailfish = SnailFish {
        left: get_node(left),
        right: get_node(right),
      };
    RefCell::new(
        Node::Pair(Rc::new(RefCell::new(snailfish)))
    )
  } else if value.is_u64() {
    let num = value.as_u64().expect("to u64 fail") as usize;
    RefCell::new(
        Node::Number(Rc::new(RefCell::new(num)))
    )
  } else {
    panic!("should never go here")
  }
}

fn parse_snailfish(text: &str) -> RefCell<Node> {
  let list: Value = serde_json::from_str(&text).expect("serde fail");
  get_node(&list)
}

fn get_pair_num(snailfish: &Rc<RefCell<SnailFish>>) -> (usize, usize) {
  let left = (match &*snailfish.clone().borrow().left.borrow() {
    Node::Number(num) => num,
    _ => panic!("{:?} get left value error", snailfish),
  }).borrow().to_owned();
  let right = (match &*snailfish.clone().borrow().right.borrow() {
    Node::Number(num) => num,
    _ => panic!("{:?} get right value error", snailfish),
  }).borrow().to_owned();
  (left, right)
}

fn get_factor(num: usize, is_down: bool) -> usize {
  let factor = (num as f64) / 2.0;
  if is_down {
    factor.floor() as usize
  } else {
    factor.ceil() as usize
  }
}

#[test]
fn test_get_factor() {
  assert_eq!(get_factor(11, true), 5);
  assert_eq!(get_factor(11, false), 6);
}

fn walk(node: &RefCell<Node>, r_action: &WalkAction, depth: usize, r_lucky: Lucky) -> (WalkAction, Lucky) {
  let mut lucky = r_lucky.clone();
  let mut action = r_action.clone();

  let walk_pair = |snailfish: &Rc<RefCell<SnailFish>>| {
    (action, lucky) = walk(&snailfish.borrow().left, &action, depth + 1, lucky);
    if action == WalkAction::Break {
      return (action, lucky);
    }
    return walk(&snailfish.borrow().right, &action, depth + 1, lucky);
  };
  let result = match r_action {  
    WalkAction::Add(e_num) => {
      match &*node.borrow() {
        // find next right num to add
        Node::Pair(snailfish) => {
          walk_pair(&snailfish)
        },
        // explode to right
        Node::Number(num) => {
          *num.borrow_mut() += e_num;
          (WalkAction::Break, r_lucky)
        }
      }
    },
    WalkAction::Continue => {
      match &*node.borrow() {
        Node::Pair(snailfish) => {
          if depth + 1 == 5 {
            let (left, right) = get_pair_num(&snailfish);
            
            (WalkAction::Explode(left, right), r_lucky)
          } else {
            walk_pair(&snailfish)
          }
        },
        Node::Number(num) => {
          (WalkAction::Continue, Some(num.clone()))
        }
      }
    },
    WalkAction::Break => (WalkAction::Break, None),
    _ => panic!("unreachable action"),
  };
  match result {
    (WalkAction::Explode(left, right), l) => {
      match l {
        Some(some_lucky) => {
          *some_lucky.borrow_mut() += left;
        },
        _ => {},
      }
      *node.borrow_mut() = Node::Number(Rc::new(RefCell::new(0)));
      (WalkAction::Add(right), None)
    },
    (a, l) => (a, l),
  }
}

fn main() {
  let snailfish = parse_snailfish("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
  println!("{:?}\n\n", snailfish);
  walk(&snailfish, &WalkAction::Continue, 0, None);
  println!("{:?}", snailfish);
}