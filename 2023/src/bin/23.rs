use itertools::Itertools;
use ndarray::{s, Array2};
use std::{
    collections::{HashMap, HashSet},
    io,
};

type Coord = (usize, usize);

fn adjacents(y: usize, x: usize, h: usize, w: usize) -> impl Iterator<Item = Coord> {
    [
        if x > 0 { Some((y, x - 1)) } else { None },
        if x < w - 1 { Some((y, x + 1)) } else { None },
        if y > 0 { Some((y - 1, x)) } else { None },
        if y < h - 1 { Some((y + 1, x)) } else { None },
    ]
    .into_iter()
    .flatten()
}

const ARROWS: [char; 4] = ['<', '>', '^', 'v'];

fn dfs(
    input: &Array2<char>,
    target: Coord,
    visited: &mut HashSet<Coord>,
    pos: Coord,
) -> Option<usize> {
    if pos == target {
        return Some(visited.len());
    }

    if input[pos] == '#' {
        return None;
    }

    if !visited.insert(pos) {
        return None;
    }

    let max_len = match input[pos] {
        '>' => dfs(input, target, visited, (pos.0, pos.1 + 1)),
        '<' => dfs(input, target, visited, (pos.0, pos.1 - 1)),
        '^' => dfs(input, target, visited, (pos.0 - 1, pos.1)),
        'v' => dfs(input, target, visited, (pos.0 + 1, pos.1)),
        _ => {
            let (h, w) = input.dim();
            adjacents(pos.0, pos.1, h, w)
                .flat_map(|co| dfs(input, target, visited, co))
                .max()
        }
    };

    visited.remove(&pos);

    max_len
}

fn dfs2(
    input: &Array2<char>,
    targets: &Vec<Coord>,
    from: Coord,
    pos: Coord,
    dist: usize,
) -> Option<(Coord, usize)> {
    if targets.contains(&pos) {
        return Some((pos, dist));
    }

    if input[pos] == '#' {
        return None;
    }

    let (h, w) = input.dim();
    adjacents(pos.0, pos.1, h, w)
        .filter(|&p| input[p] != '#' && p != from)
        .exactly_one()
        .ok()
        .and_then(|next| dfs2(input, targets, pos, next, dist + 1))
}

fn dfs3(
    adj: &HashMap<Coord, Vec<(Coord, usize)>>,
    target: Coord,
    visited: &mut Vec<Coord>,
    dist: usize,
    pos: Coord,
) -> Option<usize> {
    if pos == target {
        return Some(dist);
    }

    visited.push(pos);

    let candidates: Vec<_> = adj[&pos]
        .iter()
        .filter(|(ap, _)| !visited.contains(ap))
        .copied()
        .collect();

    let max_len = candidates
        .into_iter()
        .flat_map(|(ap, ad)| dfs3(adj, target, visited, dist + ad, ap))
        .max();

    visited.pop();

    max_len
}

fn part1(input: &Array2<char>) -> usize {
    let (h, _) = input.dim();
    let start_x = input
        .row(0)
        .indexed_iter()
        .find(|(_, c)| **c == '.')
        .unwrap()
        .0;
    let end_x = input
        .row(h - 1)
        .indexed_iter()
        .find(|(_, c)| **c == '.')
        .unwrap()
        .0;

    let mut visited = HashSet::new();

    dfs(input, (h - 1, end_x), &mut visited, (0, start_x)).unwrap()
}

fn part2(input: &Array2<char>) -> usize {
    let (h, w) = input.dim();
    let start_x = input
        .row(0)
        .indexed_iter()
        .find(|(_, c)| **c == '.')
        .unwrap()
        .0;
    let end_x = input
        .row(h - 1)
        .indexed_iter()
        .find(|(_, c)| **c == '.')
        .unwrap()
        .0;

    let crossroad_coords: Vec<Coord> = (1..=h - 2)
        .cartesian_product(1..=w - 2)
        .filter(|(y, x)| {
            input[(*y, *x)] == '.' &&
            input
                .slice(s![y - 1..=y + 1, x - 1..=x + 1])
                .map(|c| ARROWS.contains(c) as u8)
                .sum()
                >= 3
        })
        .chain(std::iter::once((0, start_x)))
        .chain(std::iter::once((h - 1, end_x)))
        .collect();

    let mut adj: HashMap<Coord, Vec<(Coord, usize)>> = HashMap::new();

    for c in &crossroad_coords {
        for ac in adjacents(c.0, c.1, h, w) {
            if input[ac] == '#' {
                continue;
            }

            if let Some((target, dist)) = dfs2(input, &crossroad_coords, *c, ac, 1) {
                let a = adj.entry(*c).or_default();
                if let Some(prev) = a.iter().find(|x| x.0 == target) {
                    assert_eq!(prev.1, dist);
                } else {
                    a.push((target, dist));
                    adj.entry(target).or_default().push((*c, dist));
                }
            }
        }
    }

    let mut visited = Vec::new();
    dfs3(&adj, (h - 1, end_x), &mut visited, 0, (0, start_x)).unwrap()
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
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(23)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 2254);

        let p2 = part2(&input);
        assert_eq!(p2, 6394);
    }
}
