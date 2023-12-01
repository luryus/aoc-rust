use itertools::Itertools;
use std::io;

fn part1(input: &Vec<String>) -> usize {
    let t0: usize = input[0].parse().unwrap();
    let ids: Vec<usize> = aoc2020::read_ints_from_string(&input[1]);

    let (id, w) = ids
        .into_iter()
        .map(|id| (id, id - t0 % id))
        .min_by_key(|(_, w)| *w)
        .unwrap();

    id * w
}

fn part2(input: &Vec<String>) -> usize {
    let input = input[1]
        .split(",")
        .map(|x| x.parse::<usize>())
        .enumerate()
        .filter_map(|(i, x)| x.ok().map(|y| (i, y)));

    input
        .fold((1, 1), |(min, step), (skip, interval)| {
            let (a, b) = (min..)
                .step_by(step)
                .filter(|x| (x + skip) % interval == 0)
                .take(2)
                .collect_tuple()
                .unwrap();
            (a, b - a)
        })
        .0
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
