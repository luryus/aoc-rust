use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::io;

lazy_static! {
    static ref PFIELDS: HashSet<&'static str> =
        ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
            .iter()
            .copied()
            .collect();
}

fn validate1(p: &str) -> bool {
    let missing = PFIELDS
        .iter()
        .copied()
        .filter(|f| !p.contains(*f))
        .collect::<Vec<_>>();

    match missing.len() {
        0 => true,
        1 if missing[0] == "cid" => true,
        _ => false,
    }
}

fn validate_num(s: &str, lo: i32, hi: i32) -> bool {
    let n = s.parse::<i32>().unwrap();
    n >= lo && n <= hi
}

fn validate2(p: &str) -> bool {
    let parts = p.split_whitespace();

    let mut remaining = PFIELDS.clone();

    for part in parts {
        let (left, right) = aoc2020::split_to_tuple2(part, ":").unwrap();
        assert!(remaining.remove(left));

        let res = if left == "byr" {
            validate_num(right, 1920, 2002)
        } else if left == "iyr" {
            validate_num(right, 2010, 2020)
        } else if left == "eyr" {
            validate_num(right, 2020, 2030)
        } else if left == "hgt" {
            let n: i32 = aoc2020::read_ints_from_string(right)[0];
            if right.ends_with("cm") && (n < 150 || n > 193) {
                false
            } else if right.ends_with("in") && (n < 59 || n > 76) {
                false
            } else if !(right.ends_with("cm") || right.ends_with("in")) {
                false
            } else {
                true
            }
        } else if left == "hcl" {
            lazy_static! {
                static ref COLOR_RE: Regex = Regex::new(r"#[a-f0-9]{6}").unwrap();
            }
            COLOR_RE.is_match(right)
        } else if left == "ecl" {
            right == "amb"
                || right == "blu"
                || right == "brn"
                || right == "gry"
                || right == "grn"
                || right == "hzl"
                || right == "oth"
        } else if left == "pid" {
            right.len() == 9 && right.chars().all(|x| x.is_ascii_digit())
        } else {
            true
        };

        if !res {
            return false;
        }
    }

    if remaining.len() > 1 {
        false
    } else if remaining.len() == 1 && *remaining.iter().next().unwrap() != "cid" {
        false
    } else {
        true
    }
}

fn run(input: &Vec<&str>, validator: fn(&str) -> bool) -> usize {
    input.into_iter().filter(|&&s| validator(s)).count()
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_to_string()?;
    let passports = input.split("\n\n").collect();

    let p1 = run(&passports, validate1);
    println!("Part 1: {}", p1);

    let p2 = run(&passports, validate2);
    println!("Part 2: {}", p2);

    Ok(())
}
