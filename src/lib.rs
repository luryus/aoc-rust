use std::io::{Read, self};
use std::iter::Iterator;
use std::str::FromStr;
use num_integer::Integer;
use regex::Regex;
use itertools::Itertools;

pub fn read_stdin_to_string() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.to_owned())
}

pub fn read_stdin_lines() -> io::Result<Vec<String>> {
    let input = read_stdin_to_string()?;
    Ok(input.lines().filter_map(|l| {
        if l.is_empty() {
            None
        } else {
            Some(l.to_owned())
        }
    }).collect())
}

pub fn read_ints_from_stdin<T: Integer + FromStr>() -> io::Result<Vec<T>> {
    let s = read_stdin_to_string()?;
    Ok(read_ints_from_string(&s))
}

pub fn read_ints_from_string<T: Integer + FromStr>(s: &str) -> Vec<T> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(s)
        .map(|m| m.as_str())
        .filter_map(|m| m.parse::<T>().ok())
        .collect::<Vec<T>>()
}

pub fn read_regex_matches_from_stdin(regex_pattern: &str) -> io::Result<Vec<String>> {
    let s = read_stdin_to_string()?;
    let matches = read_regex_matches_from_string(&s, regex_pattern);

    let res = matches.into_iter().map(|sm| sm.to_owned()).collect::<Vec<_>>();

    Ok(res)
}

pub fn read_regex_matches_from_string<'a>(s: &'a str, regex_pattern: &str) -> Vec<&'a str> {
    let re = Regex::new(regex_pattern).unwrap();
    re.find_iter(s)
        .map(|m| m.as_str())
        .collect()
}

pub fn split_to_tuple2<'a>(s: &'a str, pattern: &str) -> Option<(&'a str, &'a str)> {
    let parts = s.splitn(2, pattern);
    return parts.collect_tuple();
}

pub fn split_to_tuple3<'a>(s: &'a str, pattern: &str) -> Option<(&'a str, &'a str, &'a str)> {
    let parts = s.splitn(3, pattern);
    return parts.collect_tuple();
}

pub fn split_to_tuple4<'a>(s: &'a str, pattern: &str) -> Option<(&'a str, &'a str, &'a str, &'a str)> {
    let parts = s.splitn(4, pattern);
    return parts.collect_tuple();
}


#[test]
fn test_read_ints_from_string() {
    let s = "a123b22 123x02\n123";
    let res: Vec<i32> = read_ints_from_string(s);

    assert_eq!(5, res.len());
    assert_eq!(vec![123i32, 22, 123, 02, 123], res);
}

#[test]
fn test_read_regex_matches_from_string() {
    let s = "0.12,1.23,4.2\n111.1,.,111.,.23";
    let re = r"\d+\.\d+";

    let m = read_regex_matches_from_string(s, re);
    assert_eq!(vec!["0.12", "1.23", "4.2", "111.1"], m);
}
