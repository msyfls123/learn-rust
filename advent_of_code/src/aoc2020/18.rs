#![feature(destructuring_assignment)]
use advent_of_code::get_str_array_from_file;
use itertools::Itertools;

enum Operator {  
  Plus,
  Multiple,
}

fn operate(num1: usize, num2: usize, operator: &Operator) -> usize {
  match operator {
    Operator::Multiple => num1 * num2,
    Operator::Plus => num1 + num2,
  }
}

fn traverse(expression: &Vec<String>, mut index: usize) -> (usize, usize) {
  let len = expression.len();
  let mut result = 0;
  let mut operator = Operator::Plus;
  'cur: while index < len {
    match &expression[index][..] {
      "*" => {
        index += 1;
        operator = Operator::Multiple;
      },
      "+" => {
        index += 1;
        operator = Operator::Plus;
      },
      ")" => {
        index += 1;
        break 'cur;
      },
      "(" => {
        index += 1;
        let sub_result = traverse(&expression, index);
        index = sub_result.1;
        result = operate(result, sub_result.0, &operator);
      }
      x => {
        match x.parse::<usize>() {
          Ok(num) => {
            result = operate(result, num, &operator);
            index += 1;
          },
          Err(e) => eprintln!("{:?}", e)
        }
      }
    }
  }
  (result, index)
}

enum Value {  
  // Plus,
  Multiple,
  Num(usize),
}

fn traverse2(expression: &Vec<String>, mut index: usize) -> (usize, usize) {
  let len = expression.len();
  let mut items = vec!{};

  'cur: while index < len {
    match &expression[index][..] {
      ")" => {
        index += 1;
        break 'cur;
      },
      "(" => {
        index += 1;
        let sub_result = traverse2(&expression, index);
        index = sub_result.1;
        items.push(Value::Num(sub_result.0));
      },
      "+" => {
        index += 1;
        // items.push(Value::Plus);
      },
      "*" => {
        index += 1;
        items.push(Value::Multiple);
      }
      x => {
        match x.parse::<usize>() {
          Ok(num) => {
            index += 1;
            items.push(Value::Num(num));
          },
          Err(e) => eprintln!("{:?}", e)
        }
      }
    }
  }
  let result = items
    .into_iter()
    .group_by(|v| match v {
      Value::Multiple => false,
      Value::Num(_) => true,
    })
    .into_iter()
    .filter_map(|(ismatch, group)| {
      if ismatch {
        Some(group.into_iter().map(|x| match x {
          Value::Multiple => 0,
          Value::Num(x) => x,
        }).sum())
      } else {
        None
      }
    })
    .fold(1, |acc, v: usize| acc * v);
  (result, index)
}

fn get_expression(text: &str) -> Vec<String> {
  let text = text.replace(" ", "");
  text.split("").filter_map(|x| if x == "" { None } else { Some(x.to_string()) }).collect()
}

fn main() {
  let data = get_str_array_from_file(&vec!{"aoc2020", "data", "18.txt"});
  let expressions: Vec<Vec<String>> = data.iter().map(|l| get_expression(l)).collect();
  let sum_of_results: usize = expressions.iter().map(|exp| traverse(exp, 0).0).sum();
  println!("Part 1: {}", sum_of_results);
  let sum_of_results: usize = expressions.iter().map(|exp| traverse2(exp, 0).0).sum();
  println!("Part 2: {}", sum_of_results);
}
