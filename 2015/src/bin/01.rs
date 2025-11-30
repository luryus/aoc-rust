use aoc2015::read_stdin_to_string;
use std::io;

fn main() -> io::Result<()> {
    let buffer = read_stdin_to_string()?;
    let chars = buffer.chars();

    let movements = chars.map(|c| {
        match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        }
    }).collect::<Vec<i32>>();

    println!("{}", movements.iter().sum::<i32>());

    let mut level = 0;
    for (i, m) in movements.iter().enumerate() {
        level += m;
        if level < 0 {
            println!("{}", i + 1);
            break;
        }
    }

    Ok(())
}