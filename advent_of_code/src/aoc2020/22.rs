use advent_of_code::get_group_str_from_file;
use itertools::Itertools;

type Cards = Vec<usize>;

type Deck = Vec<Cards>;

fn round(deck: &mut Deck) {
  while deck.iter().filter(|cards| !cards.is_empty()).count() != 1 {
      let mut drawn_cards = vec!{};
      for (index, cards) in deck.iter_mut().enumerate() {
          if cards.len() > 0 {
            drawn_cards.push((index, cards.remove(0)))
          }
      }
      let sorted_cards: Vec<(usize, usize)> = drawn_cards.iter()
        .sorted_by(|x, y| y.1.cmp(&x.1))
        .map(|x| x.to_owned())
        .collect();
      let mut append_cards = sorted_cards.iter().map(|x| x.1).collect_vec();
      let winner = sorted_cards[0].0;
      deck[winner].append(&mut append_cards);
  }
}

fn main() {
  let data = get_group_str_from_file(&vec!{"aoc2020", "data", "22.txt"});
  let mut deck: Deck = data.iter().map(|player| {
    player[1..].iter().map(|c| c.parse::<usize>().unwrap()).collect_vec()
  }).collect();
  round(&mut deck);
  let winner = deck.iter().find(|card| !card.is_empty()).unwrap();
  let len = winner.len();
  let score = winner.iter().enumerate().fold(0, |acc, (index, value)| {
    acc + value * (len - index)
  });
  println!("Part 1: {}", score);
}
