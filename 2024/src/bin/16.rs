use aoclib::coord2::Coord2;
use itertools::Itertools;
use ndarray::Array2;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    io,
};

fn rot_ccw(c: Coord2<isize>) -> Coord2<isize> {
    (c.x, -c.y).into()
}
fn rot_cw(c: Coord2<isize>) -> Coord2<isize> {
    (-c.x, c.y).into()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct State(Coord2, Coord2<isize>);

fn part1(input: &Array2<char>, start: Coord2, end: Coord2) -> usize {
    let mut visited: HashMap<(Coord2, Coord2<isize>), usize> = Default::default();

    let mut q = BinaryHeap::new();
    q.push(Reverse((0, State(start, Coord2::RIGHT))));

    while let Some(Reverse((score, State(pos, d)))) = q.pop() {
        let prev_score = visited.entry((pos, d)).or_insert(score);
        if *prev_score < score {
            continue;
        }
        *prev_score = score;

        if pos == end {
            return score;
        }

        if let Some(new_pos) = pos.checked_add_with_upper(d, input.dim()) {
            if input[new_pos] == '.' {
                q.push(Reverse((score + 1, State(new_pos, d))));
            }
        }

        q.push(Reverse((score + 1000, State(pos, rot_cw(d)))));
        q.push(Reverse((score + 1000, State(pos, rot_ccw(d)))));
    }

    unreachable!()
}

fn part2(input: &Array2<char>, start: Coord2, end: Coord2) -> usize {
    let mut visited: HashMap<State, (usize, Vec<State>)> = Default::default();
    let mut q = BinaryHeap::new();

    q.push(Reverse((
        0,
        State(start, Coord2::RIGHT),
        State(start, Coord2::RIGHT),
    )));

    while let Some(Reverse((score, State(pos, d), from))) = q.pop() {
        let prev_score = visited.entry(State(pos, d)).or_insert((score, vec![from]));
        if prev_score.0 < score {
            if pos == end {
                let pos_froms = visited
                    .into_iter()
                    .map(|(k, (_, v))| (k, v))
                    .into_group_map();

                let mut visited_coords = HashSet::new();
                let mut pos_q = VecDeque::new();
                pos_q.push_back(State(pos, d));

                while let Some(p) = pos_q.pop_front() {
                    visited_coords.insert(p.0);
                    pos_q.extend(pos_froms[&p].iter().flatten().filter(|x| x != &&p).copied())
                }

                return visited_coords.len();
            }

            continue;
        }

        if prev_score.0 == score && !prev_score.1.contains(&from) {
            prev_score.1.push(from);
        } else if prev_score.0 > score {
            *prev_score = (score, vec![from]);
        };

        if let Some(new_pos) = pos.checked_add_with_upper(d, input.dim()) {
            if input[new_pos] == '.' {
                q.push(Reverse((score + 1, State(new_pos, d), State(pos, d))));
            }
        }

        q.push(Reverse((
            score + 1000,
            State(pos, rot_cw(d)),
            State(pos, d),
        )));
        q.push(Reverse((
            score + 1000,
            State(pos, rot_ccw(d)),
            State(pos, d),
        )));
    }

    unreachable!()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_char_matrix()?;
    let (map, start, end) = find_start_end(input);

    let p1 = part1(&map, start, end);
    println!("Part 1: {}", p1);

    let p2 = part2(&map, start, end);
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
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(16)).unwrap();
        let (map, start, end) = find_start_end(input);

        let p1 = part1(&map, start, end);
        assert_eq!(p1, 103512);

        let p2 = part2(&map, start, end);
        assert_eq!(p2, 554);
    }
}
