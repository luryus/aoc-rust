use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, io};

lazy_static! {
    static ref DIGIT_REGEX: Regex =
        Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
}
lazy_static! {
    static ref DIGIT_WORDS: HashMap<&'static str, u32> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .into_iter()
    .collect();
}

fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|l| l.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<_>>())
        .map(|l| l.first().unwrap() * 10 + l.last().unwrap())
        .sum()
}

fn find_first_last_digit(s: &str) -> (u32, u32) {
    let first = DIGIT_REGEX.find(s).unwrap().as_str();
    let last = (0..s.len())
        .rev()
        .find_map(|i| DIGIT_REGEX.find_at(s, i))
        .unwrap()
        .as_str();

    (
        DIGIT_WORDS
            .get(first)
            .copied()
            .unwrap_or_else(|| first.parse().unwrap()),
        DIGIT_WORDS
            .get(last)
            .copied()
            .unwrap_or_else(|| last.parse().unwrap()),
    )
}

fn part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|l| find_first_last_digit(l))
        .map(|(l, r)| l * 10 + r)
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(1)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 56049);

        let p2 = part2(&input);
        assert_eq!(p2, 54530);
    }
}
