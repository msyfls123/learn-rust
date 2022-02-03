#[macro_use] extern crate lazy_static;
extern crate regex;

use std::collections::{HashMap, HashSet};
use advent_of_code::get_str_array_from_file;
use regex::{ Regex };
use std::iter::FromIterator;

type AllergenIngredientsMap = HashMap<String, HashSet<String>>;
type Ingredients = HashSet<String>;

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn get_food(text: &str) -> Food {
    lazy_static! {
        static ref RE_FOOD: Regex = Regex::new(r"(?P<ingredients>.*)\s\(contains\s(?P<allergens>.*)\)").unwrap();
    }
    let captured = RE_FOOD.captures(text).unwrap();
    let ingredients = captured.name("ingredients").unwrap().as_str().split_whitespace().map(|v| v.to_owned()).collect();
    let allergens = captured.name("allergens").unwrap().as_str().split(", ").map(|v| v.to_owned()).collect();
    Food { ingredients, allergens }
}

fn reduce_susense_ingredients(
    suspense_map: &mut AllergenIngredientsMap,
    convinced_ingredients: &mut Ingredients,
) {
    let only_child_allergens = suspense_map.iter().filter_map(|(key, items)| {
        if items.len() == 1 {
            let first =  items.into_iter().next().unwrap();
            Some((key.to_owned(), first.to_owned()))
        } else {
            None
        }
    }).collect::<Vec<(String, String)>>();
    if only_child_allergens.is_empty() { return; }
    for (key, item) in only_child_allergens {
        suspense_map.remove(&key);
        convinced_ingredients.insert(item.to_owned());
        let allergens = suspense_map.keys().map(|v| v.to_owned()).collect::<Vec<String>>();
        allergens.iter().for_each(|allergen| {
            let mut entry = suspense_map.entry(allergen.to_owned()).or_default();
            entry.retain(|x| x != &item);
        })
    }
    reduce_susense_ingredients(suspense_map, convinced_ingredients);
}

fn combine_susense_ingredients(foods: &Vec<Food>) -> AllergenIngredientsMap {
    let mut map: AllergenIngredientsMap = HashMap::new();
    foods.iter().for_each(|food| {
        let hashset: HashSet<String> = food.ingredients.to_owned().into_iter().collect();
        food.allergens.iter().for_each(|a| {
            let entry = map.entry(a.to_string()).or_default();
            if entry.len() == 0 {
                *entry = hashset.clone();
            } else {
                *entry = entry.intersection(&hashset).map(|v| v.to_owned()).collect();
            }
        })
    });
    map
}

fn count_innocence_ingredients(
    foods: &Vec<Food>,
    guilty_ingredients: &Ingredients
) -> usize {
    foods.iter().map(|Food { ingredients, .. }| {
        ingredients.iter().filter(|&ingredient| {
            !guilty_ingredients.contains(ingredient)
        }).count()
    }).sum()
}

fn main() {
    let data = get_str_array_from_file(&vec!{"aoc2020", "data", "21.txt"});
    let foods: Vec<Food> = data.iter().map(|l| get_food(l)).collect();

    let map = combine_susense_ingredients(&foods);
    let mut convinced_ingredients = HashSet::new();
    let mut suspense_ingredient_map = map.clone();
    reduce_susense_ingredients(&mut suspense_ingredient_map, &mut convinced_ingredients);
    let all_suspense_ingredients = suspense_ingredient_map.values().flatten().map(|v| v.to_owned());
    let all_guilty_ingredients: Ingredients = HashSet::from_iter(all_suspense_ingredients.chain(convinced_ingredients));
    let innocence_ingredients_count = count_innocence_ingredients(&foods, &all_guilty_ingredients);
    println!("Part 1: {}", innocence_ingredients_count);
}