use aoclib::coord2::Coord2;
use ndarray::Array2;
use std::{collections::HashSet, io};

fn check(
    pos: Coord2<usize>,
    map: &Array2<u8>,
    cache: &mut Array2<Option<HashSet<Coord2<usize>>>>,
) -> HashSet<Coord2<usize>> {
    if let Some(Some(c)) = cache.get(pos.usizes()) {
        return c.clone();
    }

    let v = map[pos.usizes()];
    if v == 9 {
        let mut h = HashSet::new();
        h.insert(pos);
        return h;
    }

    let coords = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];

    let res: HashSet<Coord2<usize>> = coords
        .into_iter()
        .filter_map(|c| pos.checked_add_with_upper(c.into(), map.dim()))
        .filter(|c| map[c.usizes()] == v + 1)
        .flat_map(|c| check(c, map, cache))
        .collect();

    cache[pos.usizes()] = Some(res.clone());
    res
}

fn check2(pos: Coord2<usize>, map: &Array2<u8>, cache: &mut Array2<Option<usize>>) -> usize {
    if let Some(Some(c)) = cache.get(pos.usizes()) {
        return *c;
    }

    let v = map[pos.usizes()];
    if v == 9 {
        return 1;
    }

    let coords = [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)];

    let res: usize = coords
        .into_iter()
        .filter_map(|c| pos.checked_add_with_upper(c.into(), map.dim()))
        .filter(|c| map[c.usizes()] == v + 1)
        .map(|c| check2(c, map, cache))
        .sum();

    cache[pos.usizes()] = Some(res);
    res
}

fn part1(input: &Array2<u8>) -> usize {
    let mut cache = Array2::default(input.dim());
    input
        .indexed_iter()
        .filter(|&(_, v)| *v == 0u8)
        .map(|(c, _)| check(c.into(), input, &mut cache).len())
        .sum()
}

fn part2(input: &Array2<u8>) -> usize {
    let mut cache = Array2::default(input.dim());
    input
        .indexed_iter()
        .filter(|&(_, v)| *v == 0u8)
        .map(|(c, _)| check2(c.into(), input, &mut cache))
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_int_matrix()?;

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
        let input = aoclib::read_file_int_matrix(aoclib::get_test_input_file!(10)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 468);

        let p2 = part2(&input);
        assert_eq!(p2, 966);
    }
}
