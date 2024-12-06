use aoclib::UnwrapOptionIterator;
use itertools::Itertools;
use std::{collections::HashMap, io};

fn toposort(page: &[u8], orders: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
    // DFS-based topological sort

    let mut res = vec![];
    let mut marks: HashMap<u8, bool> = Default::default();

    fn visit(
        node: u8,
        res: &mut Vec<u8>,
        marks: &mut HashMap<u8, bool>,
        page: &[u8],
        orders: &HashMap<u8, Vec<u8>>,
    ) {
        if marks.get(&node).is_some_and(|x| *x) {
            return;
        }

        marks.insert(node, false);
        if let Some(os) = orders.get(&node) {
            for o in os {
                if page.contains(o) {
                    visit(*o, res, marks, page, orders);
                }
            }
        }

        marks.insert(node, true);
        res.push(node);
    }

    while marks.len() < page.len() || marks.iter().any(|(_, &b)| !b) {
        let n = page.iter().find(|p| !marks.contains_key(p)).unwrap();
        visit(*n, &mut res, &mut marks, page, orders);
    }

    res.reverse();
    res
}

fn part1(orders: &HashMap<u8, Vec<u8>>, pages: &[Vec<u8>]) -> usize {
    pages
        .iter()
        .filter(|&p| &toposort(p, orders) == p)
        .map(|p| p[p.len() / 2] as usize)
        .sum()
}

fn part2(orders: &HashMap<u8, Vec<u8>>, pages: &[Vec<u8>]) -> usize {
    pages
        .iter()
        .filter_map(|p| {
            let t = toposort(p, orders);
            (&t != p).then_some(t)
        })
        .map(|p| p[p.len() / 2] as usize)
        .sum()
}

fn parse_input(input: &[String]) -> (HashMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    let (orders, pages) = input.split(|l| l.is_empty()).collect_tuple().unwrap();

    let orders: HashMap<u8, Vec<u8>> = orders
        .iter()
        .map(|l| {
            aoclib::read_ints_from_string(l, false)
                .into_iter()
                .collect_tuple()
        })
        .unwrap_options()
        .into_group_map();
    let pages = pages
        .iter()
        .map(|l| aoclib::read_ints_from_string(l, false))
        .collect();

    (orders, pages)
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let (orders, pages) = parse_input(&input);

    let p1 = part1(&orders, &pages);
    println!("Part 1: {}", p1);

    let p2 = part2(&orders, &pages);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(5)).unwrap();
        let (orders, pages) = parse_input(&input);

        let p1 = part1(&orders, &pages);
        assert_eq!(p1, 5651);

        let p2 = part2(&orders, &pages);
        assert_eq!(p2, 4743);
    }
}
