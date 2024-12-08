use aoclib::coord2::Coord2;
use itertools::Itertools;
use ndarray::Array2;
use std::{
    collections::{HashMap, HashSet},
    io,
};

type Bounds = (usize, usize);

fn part1(dim: Bounds, nodes: &HashMap<char, Vec<Coord2<usize>>>) -> usize {
    let mut antinodes: HashSet<Coord2<usize>> = Default::default();

    for n in nodes.values() {
        let new_antinodes = n
            .iter()
            .tuple_combinations()
            .flat_map(|(&a, &b)| {
                let d = b.signed_sub(a);

                [
                    b.checked_add_with_upper(d, dim),
                    a.checked_add_with_upper(-d, dim),
                ]
            })
            .flatten();
        antinodes.extend(new_antinodes);
    }

    antinodes.len()
}

fn part2(dim: Bounds, nodes: &HashMap<char, Vec<Coord2<usize>>>) -> usize {
    let mut antinodes: HashSet<Coord2<usize>> = Default::default();

    for n in nodes.values() {
        let new_antinodes = n.iter().tuple_combinations().flat_map(|(&a, &b)| {
            let d = b.signed_sub(a);
            std::iter::successors(Some(b), move |bb| bb.checked_add_with_upper(d, dim)).chain(
                std::iter::successors(Some(a), move |aa| aa.checked_add_with_upper(-d, dim)),
            )
        });
        antinodes.extend(new_antinodes);
    }

    antinodes.len()
}

fn parse_input(input: Array2<char>) -> (Bounds, HashMap<char, Vec<Coord2<usize>>>) {
    let dim = input.dim();
    let nodes = input
        .indexed_iter()
        .filter_map(|(coord, char)| match char {
            '.' => None,
            c => Some((*c, coord.into())),
        })
        .into_group_map();

    (dim, nodes)
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_char_matrix()?;
    let (dim, nodes) = parse_input(input);

    let p1 = part1(dim, &nodes);
    println!("Part 1: {}", p1);

    let p2 = part2(dim, &nodes);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(8)).unwrap();
        let (dim, nodes) = parse_input(input);

        let p1 = part1(dim, &nodes);
        assert_eq!(p1, 392);

        let p2 = part2(dim, &nodes);
        assert_eq!(p2, 1235);
    }
}
