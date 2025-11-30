use regex::Regex;
use std::io;
use lazy_static::lazy_static;

lazy_static! {
    static ref VOWEL_REGEX: Regex = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    static ref PROHIBITED_REGEX: Regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();
}

fn has_double_character(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(1))
        .find(|(l, r)| l == r)
        .is_some()
}

fn has_character_repeat_with_one_between(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(2))
        .find(|(l, r)| l == r)
        .is_some()
}

fn has_repeating_pair(s: &str) -> bool {
    for i in 0..(s.len()-2) {
        let pair = &s[i..i+2];
        let tail = &s[i+2..];

        if tail.contains(pair) {
            return true
        }
    }

    false
}

fn is_nice1(s: &str) -> bool {
    let vowel_valid = VOWEL_REGEX.find(s).is_some();
    let prohibited_valid = PROHIBITED_REGEX.find(s).is_none();
    let double_valid = has_double_character(s);

    vowel_valid && prohibited_valid && double_valid
}

fn is_nice2(s: &str) -> bool {
    has_character_repeat_with_one_between(s) &&
        has_repeating_pair(s)
}

fn run_part1(input: &Vec<String>) {
    let c = input.iter().filter(|s| is_nice1(s.as_str())).count();
    println!("Part 1: {}", c);
}

fn run_part2(input: &Vec<String>) {
    let c = input.iter().filter(|s| is_nice2(s.as_str())).count();
    println!("Part 2: {}", c);
}

fn main() -> io::Result<()> {
    let input = aoc2015::read_stdin_lines()?;

    run_part1(&input);

    run_part2(&input);

    Ok(())
}