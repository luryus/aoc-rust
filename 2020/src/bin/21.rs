use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    io,
};
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Ingredient {
    name: String,
    allergen: Option<String>,
}

#[derive(Clone, Debug)]
struct Food {
    ingredients: Vec<Ingredient>,
    known_allergens: Vec<String>,
}

fn set_ing_allergen(foods: &mut Vec<Food>, ing_name: &str, allergen: &String) {
    for food in foods.iter_mut() {
        for i in food.ingredients.iter_mut() {
            if i.name == ing_name {
                i.allergen = Some(allergen.clone());
            }
        }
    }
}

fn resolve(mut input: Vec<Food>) -> (Vec<Food>, Vec<String>) {
    let mut allergen_counts = HashMap::new();

    input
        .iter()
        .flat_map(|f| f.known_allergens.iter())
        .for_each(|alg| {
            let c = allergen_counts.entry(alg.clone()).or_insert(0);
            *c += 1;
        });

    let mut remaining_allergens = allergen_counts
        .iter()
        .filter(|(_, c)| **c > 1)
        .map(|(k, _)| k.clone())
        .collect_vec();

    loop {
        let mut resolved = Vec::new();
        for allergen in &remaining_allergens {
            let candidates = input
                .iter()
                .filter(|f| f.known_allergens.contains(allergen))
                .map(|f| {
                    f.ingredients
                        .iter()
                        .filter(|&i| i.allergen.is_none())
                        .cloned()
                        .collect::<HashSet<_>>()
                })
                .fold1(|acc, ings| acc.intersection(&ings).cloned().collect::<HashSet<_>>());
            if let Some(candidates) = candidates {
                if let Ok(ing) = candidates.into_iter().exactly_one() {
                    resolved.push(allergen.clone());
                    set_ing_allergen(&mut input, &ing.name, allergen);
                }
            }
        }

        if resolved.is_empty() {
            break;
        }
        
        remaining_allergens.retain(|al| !resolved.contains(&al));
    }

    // Assign allergens which appear once and only have one possible ingredient
    for (al, &c) in &allergen_counts {
        if c > 1 {
            continue;
        }

        // find the food where this is
        let food = input.iter().filter(|f| f.known_allergens.contains(al)).next().unwrap();
        if let Ok(cand) = food.ingredients.iter().filter(|ing| ing.allergen.is_none()).map(|i| i.name.clone()).exactly_one() {
            set_ing_allergen(&mut input, &cand, al);
        }
    }

    (input, remaining_allergens)
}

fn part1(input: &Vec<Food>, remaining_allergens: &Vec<String>) -> usize {
    input.iter()
        .filter(|f| !f.known_allergens.iter().any(|al| remaining_allergens.contains(al)))
        .flat_map(|f| f.ingredients.iter())
        .filter(|ing| ing.allergen.is_none())
        .count()
}

fn part2(input: Vec<Food>) -> String {
    input.into_iter()
        .flat_map(|f| f.ingredients.into_iter())
        .filter(|i| i.allergen.is_some())
        .sorted_by_key(|i| i.allergen.clone())
        .map(|i| i.name)
        .unique()
        .join(",")
}

fn parse_input() -> io::Result<Vec<Food>> {
    let input = aoc2020::read_stdin_lines()?;
    Ok(input
        .into_iter()
        .map(|l| {
            let (ings, allergens) =
                aoc2020::split_to_tuple2(&l.trim_end_matches(')'), " (contains ").unwrap();
            let ings = ings
                .trim()
                .split_whitespace()
                .map(|n| Ingredient {
                    name: n.to_string(),
                    allergen: None,
                })
                .collect_vec();
            let known_allergens = allergens.trim().split(", ").map(String::from).collect_vec();

            Food {
                ingredients: ings,
                known_allergens,
            }
        })
        .collect_vec())
}

fn main() -> io::Result<()> {
    let input = parse_input()?;
    let (input, remaining) = resolve(input);

    let p1 = part1(&input, &remaining);
    println!("Part 1: {}", p1);

    let p2 = part2(input);
    println!("Part 2: {}", p2);

    Ok(())
}
