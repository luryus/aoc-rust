use std::collections::VecDeque;
use itertools::Itertools;
use std::io;
use std::collections::HashMap;
use fraction::{ToPrimitive, GenericFraction};

struct Recipe<'a> {
    target: &'a str,
    target_amt: u64,
    ings: HashMap<&'a str, u32>,
}

fn parse_recipe(l: &str) -> Option<Recipe> {
    let (ings, target) = l.split(" => ").tuples().next()?;

    let mut target_iter = target.trim().split(" ");
    let target_amt = target_iter.next().and_then(|s| s.parse().ok())?;
    let target = target_iter.next()?;

    let ings = ings
        .split(',')
        .flat_map(|s| s.split_whitespace())
        .tuples()
        .map(|(a, b)| (b, a.parse().unwrap()))
        .collect::<HashMap<&str, u32>>();
    
    Some(Recipe {
        target: target,
        target_amt: target_amt,
        ings: ings
    })
}

fn run1(recipes: &HashMap<&str, Recipe>, order: &VecDeque<&str>, fuel: u32) -> u64 {
    let mut queue = order.clone();
    let mut amounts: HashMap<&str, GenericFraction<u128>> = HashMap::new();
    amounts.insert("FUEL", fuel.into());

    while let Some(key) = queue.pop_front() {
        if key == "ORE" {
            break
        }

        let r = recipes.get(key).unwrap();
        let amt: GenericFraction<u128> =
            (*amounts.get(key).unwrap() / r.target_amt.into()).ceil() * r.target_amt.into();

        for (ing, ing_amt) in r.ings.iter() {
            let ing_amt: GenericFraction<u128> = (*ing_amt).into();
            let new_amt: GenericFraction<u128> = ing_amt * amt / r.target_amt.into();
            *amounts.entry(ing).or_insert(0.into()) += new_amt;
        }
    }

    amounts.get("ORE").unwrap().ceil().to_u64().unwrap()
}


fn run2(recipes: &HashMap<&str, Recipe>, order: &VecDeque<&str>) -> u32 {
    let mut queue = order.clone();
    let mut amounts: HashMap<&str, GenericFraction<u128>> = HashMap::new();
    amounts.insert("FUEL", 1.into());

    while let Some(key) = queue.pop_front() {
        if key == "ORE" {
            break
        }

        let r = recipes.get(key).unwrap();
        let amt: GenericFraction<u128> = amounts.remove(key).unwrap();

        for (ing, ing_amt) in r.ings.iter() {
            let ing_amt: GenericFraction<u128> = (*ing_amt).into();
            let new_amt: GenericFraction<u128> = ing_amt * (amt / r.target_amt.into());
            *amounts.entry(ing).or_insert(0.into()) += new_amt;
        }
    }

    let total_ore: GenericFraction<u128> = 1000000000000u128.into();
    let mut perfect_max = (total_ore / amounts.remove("ORE").unwrap()).floor().to_u32().unwrap();

    while run1(recipes, order, perfect_max) as u64 > 1000000000000u64 {
        perfect_max -= 1;
    }

    perfect_max
}


fn top_sort<'a>(recipes: &HashMap<&'a str, Recipe<'a>>) -> VecDeque<&'a str> {
    let mut sorted: VecDeque<&'a str> = VecDeque::new();

    fn visit<'a, 'b>(k: &'a str, sorted: &'b mut VecDeque<&'a str>, recipes: &'b HashMap<&'a str, Recipe<'a>>) {
        if sorted.contains(&k) {
            return
        }

        let rec = recipes.get(k);
        if let Some(r) = rec {
            for edge in r.ings.keys() {
                visit(edge, sorted, recipes);
            }
        }
        sorted.push_front(k);
    };

    for k in recipes.keys() {
        visit(k, &mut sorted, recipes);
    }

    sorted
}

fn main() -> io::Result<()> {

    let input = aoc2019::read_stdin_lines()?;

    let recipes = input.iter()
        .map(|l| parse_recipe(&l).unwrap())
        .map(|r| (r.target, r))
        .collect::<HashMap<&str, Recipe>>();

    let order = top_sort(&recipes);

    println!("{:?}", run1(&recipes, &order, 1));
    println!("{:?}", run2(&recipes, &order));

    Ok(())
}