use num_integer::Integer;
use std::{collections::HashMap, io};

struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

enum Dir {
    L,
    R,
}

fn part1(dirs: &[Dir], nodes: &HashMap<&str, Node>) -> usize {
    let mut curr = "AAA";
    let mut steps = 0usize;
    for d in dirs.iter().cycle() {
        steps += 1;
        let n = &nodes[curr];
        curr = match d {
            Dir::L => n.left,
            Dir::R => n.right,
        };
        if curr == "ZZZ" {
            return steps;
        }
    }

    unreachable!()
}

fn find_loop(start: &str, dirs: &[Dir], nodes: &HashMap<&str, Node>) -> (usize, usize) {
    let mut i = 0usize;
    let mut curr = start;
    let mut states: HashMap<(usize, &str), usize> = HashMap::new();
    for (di, d) in dirs.iter().enumerate().cycle() {
        i += 1;

        let n = &nodes[curr];
        curr = match d {
            Dir::L => n.left,
            Dir::R => n.right,
        };

        if let Some(prev) = states.get(&(di, curr)) {
            let ending = states.iter().find(|(k, _)| k.1.ends_with('Z')).unwrap();
            return (*ending.1, i - prev);
        } else {
            states.insert((di, curr), i);
        }
    }

    unreachable!()
}

fn part2(dirs: &[Dir], nodes: &HashMap<&str, Node>) -> usize {
    let starts = nodes
        .keys()
        .copied()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<_>>();

    let loops: Vec<_> = starts.iter().map(|id| find_loop(id, dirs, nodes)).collect();

    // The loops seem to be designed in a way such that the first Z (end) lands on the
    // loop cycle --> lcm is enough to get the final result
    // This is _not_ a general solution, but AoC is designed so that this works,
    // this is day 8 after all
    return loops
        .iter()
        .map(|(_, l)| *l)
        .reduce(|a, b| a.lcm(&b))
        .unwrap();
}

fn parse_input(input: &[String]) -> (Vec<Dir>, HashMap<&str, Node>) {
    let dirs = input[0]
        .chars()
        .map(|c| match c {
            'L' => Dir::L,
            'R' => Dir::R,
            _ => panic!("Invalid direction char"),
        })
        .collect();

    let nodes = input
        .iter()
        .skip(2)
        .map(|l| {
            let ms = aoclib::read_regex_matches_from_string(l, "[A-Z0-9]{3}");
            (
                ms[0],
                Node {
                    left: ms[1],
                    right: ms[2],
                },
            )
        })
        .collect();

    (dirs, nodes)
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let (dirs, nodes) = parse_input(&input);

    let p1 = part1(&dirs, &nodes);
    println!("Part 1: {}", p1);

    let p2 = part2(&dirs, &nodes);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(8)).unwrap();

        let (dirs, nodes) = parse_input(&input);
        let p1 = part1(&dirs, &nodes);
        assert_eq!(p1, 21251);

        let p2 = part2(&dirs, &nodes);
        assert_eq!(p2, 11678319315857);
    }
}
