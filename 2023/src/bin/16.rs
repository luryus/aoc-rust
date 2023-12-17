use itertools::Itertools;
use ndarray::Array2;
use std::{collections::HashSet, io};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

fn coord_in_dir(c: (usize, usize), dim: (usize, usize), dir: Dir) -> Option<(usize, usize, Dir)> {
    match dir {
        Dir::Left if c.1 > 0 => Some((c.0, c.1 - 1, dir)),
        Dir::Right if c.1 < dim.1 - 1 => Some((c.0, c.1 + 1, dir)),
        Dir::Up if c.0 > 0 => Some((c.0 - 1, c.1, dir)),
        Dir::Down if c.0 < dim.0 - 1 => Some((c.0 + 1, c.1, dir)),
        _ => None,
    }
}

fn find_beam_positions(
    dir: Dir,
    pos: (usize, usize),
    grid: &Array2<char>,
    vis: &mut HashSet<(usize, usize, Dir)>,
) {
    if !vis.insert((pos.0, pos.1, dir)) {
        return;
    }

    let dim = grid.dim();
    let this_tile = grid[pos];

    let next_pos = match dir {
        Dir::Left if (this_tile == '.' || this_tile == '-') => [coord_in_dir(pos, dim, dir), None],
        Dir::Right if (this_tile == '.' || this_tile == '-') => [coord_in_dir(pos, dim, dir), None],
        Dir::Up if (this_tile == '.' || this_tile == '|') => [coord_in_dir(pos, dim, dir), None],
        Dir::Down if (this_tile == '.' || this_tile == '|') => [coord_in_dir(pos, dim, dir), None],

        Dir::Left | Dir::Right if (this_tile == '|') => [
            coord_in_dir(pos, dim, Dir::Up),
            coord_in_dir(pos, dim, Dir::Down),
        ],
        Dir::Up | Dir::Down if (this_tile == '-') => [
            coord_in_dir(pos, dim, Dir::Left),
            coord_in_dir(pos, dim, Dir::Right),
        ],

        Dir::Right if this_tile == '/' => [coord_in_dir(pos, dim, Dir::Up), None],
        Dir::Down if this_tile == '/' => [coord_in_dir(pos, dim, Dir::Left), None],
        Dir::Up if this_tile == '/' => [coord_in_dir(pos, dim, Dir::Right), None],
        Dir::Left if this_tile == '/' => [coord_in_dir(pos, dim, Dir::Down), None],

        Dir::Right if this_tile == '\\' => [coord_in_dir(pos, dim, Dir::Down), None],
        Dir::Down if this_tile == '\\' => [coord_in_dir(pos, dim, Dir::Right), None],
        Dir::Up if this_tile == '\\' => [coord_in_dir(pos, dim, Dir::Left), None],
        Dir::Left if this_tile == '\\' => [coord_in_dir(pos, dim, Dir::Up), None],

        _ => panic!("Invalid state"),
    };

    for (py, px, pd) in next_pos.into_iter().flatten() {
        find_beam_positions(pd, (py, px), grid, vis)
    }
}

fn part1(input: &Array2<char>) -> usize {
    let mut visited = HashSet::new();
    find_beam_positions(Dir::Right, (0, 0), input, &mut visited);
    visited
        .into_iter()
        .map(|(py, px, _)| (py, px))
        .unique()
        .count()
}

fn part2(input: &Array2<char>) -> usize {
    let (h, w) = input.dim();
    let top = (0..w).map(|x| (0, x, Dir::Down));
    let bot = (0..w).map(|x| (h - 1, x, Dir::Up));
    let left = (0..h).map(|y| (y, 0, Dir::Right));
    let right = (0..h).map(|y| (y, w - 1, Dir::Left));

    top.chain(bot)
        .chain(left)
        .chain(right)
        .map(|(y, x, d)| {
            let mut visited = HashSet::new();
            find_beam_positions(d, (y, x), input, &mut visited);
            visited
                .into_iter()
                .map(|(py, px, _)| (py, px))
                .unique()
                .count()
        })
        .max()
        .unwrap()
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
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(16)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 8125);

        let p2 = part2(&input);
        assert_eq!(p2, 8489);
    }
}
