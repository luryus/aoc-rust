use std::io;

fn part1(input: &Vec<String>) -> Option<usize> {
    trees(input, 3, 1)
}

fn trees(input: &Vec<String>, dx: usize, dy: usize)  -> Option<usize> {
    let mut x = dx;
    let mut trees = 0;
    let w = input.get(0)?.len();
        
    for y in (dy..input.len()).step_by(dy)  {
        if input[y].chars().nth(x)? == '#' {
            trees += 1;
        }

        x = (x + dx) % w;
    }

    Some(trees)
}

fn part2(input: &Vec<String>) -> Option<usize> {
    let res = trees(input, 1, 1)? *
        trees(input, 3, 1)? *
        trees(input, 5, 1)? *
        trees(input, 7, 1)? *
        trees(input, 1, 2)?;

    Some(res)
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1.unwrap());

    let p2 = part2(&input);
    println!("Part 2: {}", p2.unwrap());

    Ok(())
}