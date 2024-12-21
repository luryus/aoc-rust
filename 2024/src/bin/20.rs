use aoclib::coord2::Coord2;
use itertools::Itertools;
use ndarray::Array2;
use std::{collections::HashMap, io};
use tailcall::tailcall;

fn find_no_cheat_distances(map: &Array2<char>, start: Coord2) -> HashMap<Coord2, usize> {
    #[allow(unreachable_code)]
    #[tailcall]
    fn visit(
        map: &Array2<char>,
        pos: Coord2,
        from: Coord2,
        d: usize,
        distances: &mut HashMap<Coord2, usize>,
    ) {
        assert!(distances.insert(pos, d).is_none());
        let Some(next) = [Coord2::DOWN, Coord2::RIGHT, Coord2::LEFT, Coord2::UP]
            .into_iter()
            .filter_map(|dir| pos.checked_add_with_upper(dir, map.dim()))
            .find(|p| p != &from && map[*p] != '#')
        else {
            return;
        };

        visit(map, next, pos, d + 1, distances);
    }

    let mut no_cheat_distances: HashMap<Coord2, usize> = HashMap::new();
    visit(map, start, start, 0, &mut no_cheat_distances);
    no_cheat_distances
}

#[tailcall]
fn find_cheats<const N: usize>(
    map: &Array2<char>,
    no_cheat_dists: &HashMap<Coord2, usize>,
    pos: Coord2,
    from: Coord2,
    d: usize,
    cheat_count: &mut usize,
) {
    let y_min = pos.y.saturating_sub(N);
    let x_min = pos.x.saturating_sub(N);
    let y_max = (pos.y + N).min(map.dim().0 - 1);
    let x_max = (pos.x + N).min(map.dim().1 - 1);

    for (ty, tx) in (y_min..=y_max).cartesian_product(x_min..=x_max) {
        let tc: Coord2 = (ty, tx).into();
        let dist = pos.manhattan_dist(&tc);
        if dist > N {
            continue;
        }
        if map[tc] == '#' || tc == pos {
            continue;
        }
        let target_nocheat_dist = *no_cheat_dists.get(&tc).unwrap();
        if target_nocheat_dist < (d + dist) {
            continue;
        }
        let save = target_nocheat_dist - (d + dist);
        if save >= 100 {
            *cheat_count += 1;
        }
    }

    let Some(next) = [Coord2::DOWN, Coord2::RIGHT, Coord2::LEFT, Coord2::UP]
        .into_iter()
        .filter_map(|dir| pos.checked_add_with_upper(dir, map.dim()))
        .find(|p| p != &from && map[*p] != '#')
    else {
        return;
    };

    find_cheats::<N>(map, no_cheat_dists, next, pos, d + 1, cheat_count);
}

fn run<const N: usize>(map: &Array2<char>, start: Coord2, end: Coord2) -> usize {
    let no_cheat_distances = find_no_cheat_distances(map, start);
    assert!(no_cheat_distances.contains_key(&end));

    let mut cc = 0;
    find_cheats::<N>(map, &no_cheat_distances, start, start, 0, &mut cc);
    cc
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_char_matrix()?;
    let (map, start, end) = find_start_end(input);

    let p1 = run::<2>(&map, start, end);
    println!("Part 1: {}", p1);

    let p2 = run::<20>(&map, start, end);
    println!("Part 2: {}", p2);

    Ok(())
}

fn find_start_end(mut input: Array2<char>) -> (Array2<char>, Coord2, Coord2) {
    let start: Coord2 = input
        .indexed_iter()
        .find(|&(_, c)| *c == 'S')
        .unwrap()
        .0
        .into();
    let end: Coord2 = input
        .indexed_iter()
        .find(|&(_, c)| *c == 'E')
        .unwrap()
        .0
        .into();

    input[start] = '.';
    input[end] = '.';

    (input, start, end)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(20)).unwrap();
        let (map, start, end) = find_start_end(input);

        let p1 = run::<2>(&map, start, end);
        assert_eq!(p1, 1327);

        let p2 = run::<20>(&map, start, end);
        assert_eq!(p2, 985737);
    }
}
