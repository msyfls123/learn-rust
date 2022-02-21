use advent_of_code::get_group_str_from_file;
use itertools::Itertools;
use std::{collections::HashMap};

type Cards = Vec<usize>;

type Deck = Vec<Cards>;

type Cache = HashMap<Deck, bool>;

#[derive(Debug)]
struct Winner {
    index: usize,
    cards: Cards,
}

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

fn round2(deck_raw: &Deck, mut game: usize) -> Winner {
  let mut deck = deck_raw.to_owned();
  let mut cache: Cache = HashMap::new();
  let mut winner_index = 0;
  let mut round = 0;
  game += 1;
  while deck.iter().filter(|cards| !cards.is_empty()).count() != 1 {
    
    if cache.get(&deck).is_some() {
      winner_index = 0;
      break;
    } else {
      cache.insert(deck.to_owned(), true);
    }
    round += 1;
    // println!("Round {} Game {}:", round, game);
    // println!("{:?}", deck);

    let mut drawn_cards = vec!{};
    for (index, cards) in deck.iter_mut().enumerate() {
        if cards.len() > 0 {
          drawn_cards.push((index, cards.remove(0)))
        }
    }
    winner_index = if drawn_cards.iter().all(|&(index, card)| deck[index].len() >= card) {
      let sub_deck = drawn_cards.iter().map(|&(index, card)| deck[index][0..card].to_owned()).collect();
      round2(&sub_deck, game).index
    } else {
      drawn_cards.iter().max_by(|x, y| x.1.cmp(&y.1)).map(|x| x.0).unwrap()
    };
    let mut sorted_cards: Cards = drawn_cards.iter().sorted_by_key(|&x| {
      if x.0 == winner_index {
        -1
      } else {
        1
      }
    }).map(|x| x.1).collect();
    deck[winner_index].append(&mut sorted_cards);
  }
  Winner {
    index: winner_index,
    cards: deck[winner_index].to_owned(),
  }
}

fn main() {
  let data = get_group_str_from_file(&vec!{"aoc2020", "data", "22.txt"});
  let deck: Deck = data.iter().map(|player| {
    player[1..].iter().map(|c| c.parse::<usize>().unwrap()).collect_vec()
  }).collect();

  let mut deck1 = deck.to_owned();
  round(&mut deck1);
  let winner = deck1.iter().find(|card| !card.is_empty()).unwrap();
  let len = winner.len();
  let score = winner.iter().enumerate().fold(0, |acc, (index, value)| {
    acc + value * (len - index)
  });
  println!("Part 1: {}", score);

  let winner = round2(&deck.to_owned(), 0);
  let len = winner.cards.len();
  let score = winner.cards.iter().enumerate().fold(0, |acc, (index, value)| {
    acc + value * (len - index)
  });
  println!("Part 2: {}", score);
}
