use std::io::{Read, self};
use std::iter::Iterator;

pub fn read_stdin_to_string() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_owned())
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