use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Bag<'a> {
    parents: Vec<&'a str>,
    children: Vec<(usize, &'a str)>,
    dirty: bool
}

impl<'a> Bag<'a> {
    fn new() -> Bag<'a> {
        Bag {
            parents: Vec::new(),
            children: Vec::new(),
            dirty: false
        }
    }
}

fn containing_count(
    bc: &str,
    g: &mut HashMap<&str, Bag>,
) -> usize {
    let bag = g.get_mut(bc).unwrap();

    if bag.dirty {
        return 0;
    }

    bag.dirty = true;

    bag.parents
        .clone()
        .iter()
        .map(|p| containing_count(p, g))
        .sum::<usize>()
        + 1
}

fn part1(g: &HashMap<&str, Bag>) -> usize {
    let mut g = g.clone();
    containing_count("shiny gold", &mut g) - 1
}

fn parse_line<'a>(l: &'a str) -> Option<(&'a str, Vec<(usize, &'a str)>)> {
    lazy_static! {
        static ref REL: Regex = Regex::new(r"^(.+?) bags?").unwrap();
        static ref RER: Regex = Regex::new(r"(\d+?) (.+?) bags?").unwrap();
    }

    let (l, r) = aoc2020::split_to_tuple2(l, " contain ")?;
    let lcolor = &REL.captures(l)?.get(1)?.as_str();
    let rcolors = RER
        .captures_iter(r)
        .map(|c| (c[1].parse().unwrap(), c.get(2).unwrap().as_str()))
        .collect::<Vec<_>>();

    Some((lcolor, rcolors))
}

fn build_graph(input: &Vec<String>) -> HashMap<&str, Bag> {
    let mut m: HashMap<&str, Bag> = HashMap::new();

    input
        .iter()
        .map(|s| parse_line(s).unwrap())
        .for_each(|(l, r)| {
            let lb = m.entry(l).or_insert_with(|| Bag::new());
            for rb in &r {
                lb.children.push(*rb);
            }

            for (_, rb) in r {
                let b = m.entry(rb).or_insert_with(|| Bag::new());
                b.parents.push(l);
            }
        });
    m
}

fn bag_count(b: &Bag, g: &HashMap<&str, Bag>) -> usize {
    b.children
        .iter()
        .map(|(c, tag)| c * bag_count(&g[*tag], g))
        .sum::<usize>()
        + 1
}

fn part2(g: &HashMap<&str, Bag>) -> usize {
    bag_count(g.get("shiny gold").unwrap(), &g) - 1
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_lines()?;
    let graph = build_graph(&input);

    let p1 = part1(&graph);
    println!("Part 1: {}", p1);

    let p2 = part2(&graph);
    println!("Part 2: {}", p2);

    Ok(())
}
