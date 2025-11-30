use itertools::{Itertools, MinMaxResult};
use regex;
use std::collections::{BTreeMap, BTreeSet};
use std::io;

fn get_escaped_len(s: &str) -> usize {
    let r = regex::Regex::new(r#"\\([\\"]|x[a-f0-9]{2})"#).unwrap();
    r.replace_all(s, "a").len() - 2
}

fn encode(s: &str) -> usize {
    s.replace(r"\", r"\\").replace("\"", "\\\"").len() + 2
}

fn parse_line(l: &str) -> Option<(String, String, usize)> {
    let (a, b) = aoc2015::split_to_tuple2(l, " to ")?;
    let (c, d) = aoc2015::split_to_tuple2(b, " = ")?;

    return Some((
        a.to_owned(),
        c.to_owned(),
        aoc2015::read_ints_from_string(d)[0],
    ));
}

fn parse_input<'a>(v: Vec<String>) -> Option<(Vec<String>, BTreeMap<(String, String), usize>)> {
    let mut distances = BTreeMap::new();
    let mut all_cities = Vec::new();

    for row in v {
        let (a, b, dist) = parse_line(&row)?;
        if !all_cities.contains(&a) {
            all_cities.push(a.clone());
        }
        if !all_cities.contains(&b) {
            all_cities.push(b.clone());
        }
        distances.insert((a, b), dist);
    }

    Some((all_cities, distances))
}

fn main() -> io::Result<()> {
    let input = aoc2015::read_stdin_lines()?;
    let (all_cities, input) = parse_input(input).unwrap();

    let (p1, p2) = find_minmax_route(&input, &all_cities);

    println!("Part 1 {}", p1);
    println!("Part 2 {}", p2);

    Ok(())
}

fn find_minmax_route(input: &BTreeMap<(String, String), usize>, all_cities: &Vec<String>) -> (usize, usize) {
    let index_distances = input
        .into_iter()
        .map(|((a, b), dist)| {
            (
                (
                    all_cities.iter().position(|x| x == a).unwrap(),
                    all_cities.iter().position(|x| x == b).unwrap(),
                ),
                dist,
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mmr = (0..all_cities.len())
        .permutations(all_cities.len())
        .map(|p| {
            p.into_iter()
                .tuple_windows()
                .map(|(a, b)| *index_distances.get(&(a, b)).or_else(|| index_distances.get(&(b, a))).unwrap())
                .sum()
        })
        .minmax();

    match mmr {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => panic!()
    }
}
