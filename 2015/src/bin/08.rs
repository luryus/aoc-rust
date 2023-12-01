use std::io;
use regex;

fn get_escaped_len(s: &str) -> usize {
    let r = regex::Regex::new(r#"\\([\\"]|x[a-f0-9]{2})"#).unwrap();
    r.replace_all(s, "a").len() - 2
}

fn encode(s: &str) -> usize {
    s.replace(r"\", r"\\")
        .replace("\"", "\\\"")
        .len() + 2
}

fn main() -> io::Result<()> {
    let input = aoc2015::read_stdin_lines()?;

    let escaped_len_1 = input.iter().map(|l| get_escaped_len(l.trim())).sum::<usize>();
    let unescaped_len_1: usize = input.iter().map(|l| l.trim().len()).sum();
    println!("Part 1 {}", unescaped_len_1 - escaped_len_1);

    let escaped_len_2 : usize = input.iter().map(|l| encode(l.trim())).sum();

    println!("part 2 {}", escaped_len_2 - unescaped_len_1);

    Ok(())
}