use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, io};

fn part1(patterns: &[String], rows: &[String]) -> usize {
    let patterns_re = patterns.iter().join("|");
    let re = Regex::new(&format!("^({patterns_re})*$")).unwrap();
    rows.iter().filter(|l| re.is_match(l)).count()
}

fn part2(patterns: &[String], rows: &[String]) -> usize {
    let mut cache: HashMap<&str, usize> = HashMap::new();

    return rows
        .iter()
        .map(|r| {
            let res = patterns
                .iter()
                .map(|p| visit(patterns, &mut cache, r, p))
                .sum::<usize>();
            res
        })
        .sum();

    fn visit<'a>(
        patterns: &'a [String],
        cache: &mut HashMap<&'a str, usize>,
        row: &'a str,
        pat: &'a str,
    ) -> usize {
        //println!("{row} {pat}");
        if pat.len() > row.len() || !row.starts_with(pat) {
            return 0;
        }

        if row == pat {
            return 1;
        }

        let row = &row[pat.len()..];

        if let Some(c) = cache.get(row) {
            return *c;
        }


        let res =patterns
            .iter()
            .map(|p| visit(patterns, cache, row, p))
            .sum();
        cache.insert(row, res);
        res
    }
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_string()?;
    let (patterns, rows) = parse_input(input);

    let p1 = part1(&patterns, &rows);
    println!("Part 1: {}", p1);

    let p2 = part2(&patterns, &rows);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(input: String) -> (Vec<String>, Vec<String>) {
    let (patterns, rows) = aoclib::split_to_tuple2(&input, "\n\n").unwrap();

    let patterns = patterns.split(", ").map_into().collect();

    (patterns, rows.lines().map_into().collect())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = std::fs::read_to_string(aoclib::get_test_input_file!(19)).unwrap();
        let (patterns, rows) = parse_input(input);

        let p1 = part1(&patterns, &rows);
        assert_eq!(p1, 333);

        let p2 = part2(&patterns, &rows);
        assert_eq!(p2, 678536865274732);
    }
}
