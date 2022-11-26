use std::{rc::Rc, cell::RefCell, fmt::Display};

use advent_of_code::get_str_array_from_file;
use itertools::Itertools;
use serde_json::Value;

#[derive(Debug)]
enum Node {
  Pair(Rc<RefCell<SnailFish>>),
  Number(Rc<RefCell<usize>>),
}

impl Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Self::Number(num) => {
          write!(f, "{}", num.borrow())
        },
        Self::Pair(snailfish) => {
          write!(f, "[{},{}]", *snailfish.borrow().left.borrow(), *snailfish.borrow().right.borrow())
        }
      }
  }
}

#[derive(Debug)]
struct SnailFish {
  left: RefCell<Node>,
  right: RefCell<Node>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum WalkAction {
  Continue,
  Break,
  Add(usize),
  Explode(usize, usize),
  Split(usize),
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
  let v = RefCell::new(5);
  let v_v = v.borrow().to_owned();
  *v.borrow_mut() += v_v;
  println!("{}", v.borrow());
}

fn walk(node: &RefCell<Node>, r_action: &WalkAction, depth: usize, r_lucky: Lucky, prefer_split: bool) -> (WalkAction, Lucky) {
  let mut lucky = r_lucky.clone();
  let mut action = r_action.clone();

  let walk_pair = |snailfish: &Rc<RefCell<SnailFish>>| {
    (action, lucky) = walk(&snailfish.borrow().left, &action, depth + 1, lucky, prefer_split);
    if action == WalkAction::Break {
      return (action, None);
    }
    return walk(&snailfish.borrow().right, &action, depth + 1, lucky, prefer_split);
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
          (WalkAction::Break, None)
        }
      }
    },
    WalkAction::Continue => {
      match &*node.borrow() {
        Node::Pair(snailfish) => {
          // meet explode threshold
          if depth + 1 >= 5 {
            let (left, right) = get_pair_num(&snailfish);
            
            (WalkAction::Explode(left, right), r_lucky)
          } else {
            walk_pair(&snailfish)
          }
        },
        Node::Number(num) => {
          let number = *num.borrow();
          // scan explode pair first, ignore split
          if number >= 10 && !prefer_split {
            (WalkAction::Split(number), None)
          } else {
            (WalkAction::Continue, Some(num.clone()))
          }
        }
      }
    },
    WalkAction::Break => (WalkAction::Break, None),
    a => panic!("unreachable action{:?}: {:?}", a, node),
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
    (WalkAction::Split(num), _) => {
      let left = get_factor(num, true);
      let right = get_factor(num, false);
      *node.borrow_mut() = parse_snailfish(&format!("[{},{}]", left, right)).into_inner();
      (WalkAction::Break, None)
    },
    (a, l) => (a, l),
  }
}

fn reduce(node: &RefCell<Node>) {
  let mut action = WalkAction::Break;
  while action != WalkAction::Continue {
    (action, _) = walk(&node, &WalkAction::Continue, 0, None, true);
    if action == WalkAction::Continue {
      (action, _) = walk(&node, &WalkAction::Continue, 0, None, false);
    }
    // println!("{}", node.borrow());
  }
}

fn add(left: RefCell<Node>, right: RefCell<Node>) -> RefCell<Node> {
  let node = RefCell::new(Node::Pair(
    Rc::new(RefCell::new(SnailFish { left, right }))
  ));
  reduce(&node);
  node
}

fn sum_list<T>(list: &Vec<T>) -> RefCell<Node>
where
  T: Into<String> + Clone,
{
  let mut node = parse_snailfish(&list.get(0).unwrap().clone().into());
  for text in &list[1..] {
      node = add(node, parse_snailfish(&text.clone().into()));
  }
  node
}

fn calc_magnitude(node: &RefCell<Node>) -> usize {
  match &*node.borrow() {
    Node::Number(num) => num.borrow().to_owned(),
    Node::Pair(snailfish) => {
      3 * calc_magnitude(&snailfish.borrow().left) +
      2 * calc_magnitude(&snailfish.borrow().right)
    },
  }
}

#[test]
fn test_calc_magnitude() {
  assert_eq!(calc_magnitude(&parse_snailfish("[[1,2],[[3,4],5]]")), 143);
  assert_eq!(calc_magnitude(&parse_snailfish("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")), 1384);
  assert_eq!(calc_magnitude(&parse_snailfish("[[[[1,1],[2,2]],[3,3]],[4,4]]")), 445);
  assert_eq!(calc_magnitude(&parse_snailfish("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")), 3488);
}

fn calc_largest_magnitude(list: &Vec<String>) -> usize{
  list.iter().enumerate().map(|(i, text_a)| {
    [&list[0..i], &list[i+1..]].concat().iter().map(|text_b| calc_magnitude(&sum_list(&vec!{text_a, text_b}))).max().unwrap()
  }).max().unwrap()
}

#[test]
fn test_calc_largest_magnitude() {
  let lines = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#.lines().into_iter().map(|t| t.trim().to_string()).collect();
  assert_eq!(calc_largest_magnitude(&lines), 3993);
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2021", "data", "18.txt"});
  let node = sum_list(&data);
  let magnitude = calc_magnitude(&node);
  println!("Part 1: {}", magnitude);

  let largest_magnitude = calc_largest_magnitude(&data);
  println!("Part 2: {}", largest_magnitude);
}