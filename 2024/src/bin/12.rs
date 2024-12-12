use aoclib::coord2::Coord2;
use itertools::Itertools;
use ndarray::Array2;
use std::io;

fn part1(input: &Array2<char>) -> usize {
    let mut visited: Array2<bool> = Array2::from_elem(input.dim(), false);

    let mut sum = 0;

    for (pos, _) in input.indexed_iter() {
        let (area, perimeter) = check(pos.into(), &mut visited, input);
        sum += area * perimeter;
    }

    return sum;

    fn check(pos: Coord2<usize>, visited: &mut Array2<bool>, map: &Array2<char>) -> (usize, usize) {
        if visited[pos] {
            return (0, 0);
        }
        visited[pos] = true;

        let c = map[pos];

        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)].map(|c| c.into());

        let (area, perim) = dirs
            .into_iter()
            .map(|d| match pos.checked_add_with_upper(d, map.dim()) {
                Some(n) if map[n] == c => check(n, visited, map),
                _ => (0, 1),
            })
            .fold((0, 0), |(aa, ap), (a, p)| (aa + a, ap + p));

        (area + 1, perim)
    }
}

fn part2(input: &Array2<char>) -> usize {
    let mut visited: Array2<bool> = Array2::from_elem(input.dim(), false);

    let mut sum = 0;

    for (pos, _) in input.indexed_iter() {
        let mut fences = vec![];
        let area = check(pos.into(), &mut visited, input, &mut fences);

        let fences = fences.into_iter().into_group_map();
        let perimeter: usize = fences
            .into_iter()
            .flat_map(|(d, poss)| match d.y {
                0 => poss
                    .into_iter()
                    .map(|x| {
                        let (a, b) = x.usizes();
                        (b, a)
                    })
                    .into_group_map(),
                _ => poss.into_iter().map(|x| x.usizes()).into_group_map(),
            })
            .map(|(_, cs)| {
                1 + cs.into_iter()
                    .sorted()
                    .tuple_windows()
                    .filter(|&(a, b)| b > a + 1)
                    .count()
            })
            .sum();

        sum += area * perimeter;
    }

    return sum;

    fn check(
        pos: Coord2<usize>,
        visited: &mut Array2<bool>,
        map: &Array2<char>,
        fences: &mut Vec<(Coord2<isize>, Coord2<usize>)>,
    ) -> usize {
        if visited[pos] {
            return 0;
        }
        visited[pos] = true;
        let c = map[pos];

        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)].map(|c| c.into());
        let area: usize = dirs
            .into_iter()
            .map(|d| match pos.checked_add_with_upper(d, map.dim()) {
                Some(n) if map[n] == c => check(n, visited, map, fences),
                _ => {
                    fences.push((d, pos));
                    0
                }
            })
            .sum();

        area + 1
    }
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_char_matrix()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(12)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 1483212);

        let p2 = part2(&input);
        assert_eq!(p2, 897062);
    }
}
