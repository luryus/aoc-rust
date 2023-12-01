use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{char, i32, newline},
    combinator::{all_consuming, map},
    multi::{count, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};
use std::io;

#[derive(PartialEq, Eq, Clone)]
enum Item {
    Number(i32),
    List(Vec<Item>),
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Item::Number(s), Item::Number(o)) => s.cmp(o),
            (s @ Item::List(_), o @ Item::Number(_)) => s.cmp(&Item::List(vec![o.clone()])),
            (s @ Item::Number(_), o @ Item::List(_)) => Item::List(vec![s.clone()]).cmp(o),
            (Item::List(s), Item::List(o)) => s
                .iter()
                .zip_longest(o.iter())
                .filter_map(|p| match p {
                    itertools::EitherOrBoth::Both(l, r) => l.partial_cmp(r),
                    itertools::EitherOrBoth::Left(_) => Some(std::cmp::Ordering::Greater),
                    itertools::EitherOrBoth::Right(_) => Some(std::cmp::Ordering::Less),
                })
                .find(|c| !c.is_eq())
                .unwrap_or(std::cmp::Ordering::Equal),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn item(input: &str) -> IResult<&str, Item> {
    alt((item_number, item_list))(input)
}

fn item_list(input: &str) -> IResult<&str, Item> {
    map(
        delimited(char('['), separated_list0(char(','), item), char(']')),
        Item::List,
    )(input)
}

fn item_number(input: &str) -> IResult<&str, Item> {
    map(i32, Item::Number)(input)
}

fn item_pair(input: &str) -> IResult<&str, (Item, Item)> {
    separated_pair(item_list, newline, item_list)(input)
}

fn parse_input(input: &str) -> Vec<(Item, Item)> {
    all_consuming(separated_list1(count(newline, 2), item_pair))(input.trim_end())
        .unwrap()
        .1
}

fn part1(input: &[(Item, Item)]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| l <= r)
        .map(|(i, _p)| i + 1)
        .sum()
}

fn part2(input: Vec<(Item, Item)>) -> usize {
    let div1 = Item::List(vec![Item::List(vec![Item::Number(2)])]);
    let div2 = Item::List(vec![Item::List(vec![Item::Number(6)])]);

    let mut input = input
        .into_iter()
        .flat_map(|(l, r)| [l, r].into_iter())
        .collect_vec();

    input.push(div1.clone());
    input.push(div2.clone());

    input.sort();

    let div1_idx = input.iter().position(|i| i == &div1).unwrap() + 1;
    let div2_idx = input.iter().position(|i| i == &div2).unwrap() + 1;

    div1_idx * div2_idx
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_string()?;
    let input = parse_input(&input);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = std::fs::read_to_string(aoclib::get_test_input_file!(13)).unwrap();
        let input = parse_input(&input);

        let p1 = part1(&input);
        assert_eq!(p1, 5905);

        let p2 = part2(input);
        assert_eq!(p2, 21691);
    }
}
