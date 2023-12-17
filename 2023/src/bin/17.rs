use ndarray::Array2;
use num_complex::Complex;
use std::collections::BinaryHeap;
use std::{collections::HashSet, io};

#[derive(Eq, PartialEq, Clone, Hash)]
struct State {
    x: i32,
    y: i32,
    dir: Complex<i8>,
    dir_steps: u8,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dir_steps.cmp(&self.dir_steps)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct WeightState(usize, State);
impl Ord for WeightState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for WeightState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type StateQueue = BinaryHeap<WeightState>;

fn run(input: &Array2<usize>, part2: bool) -> usize {
    let (h, w) = input.dim();
    let goal = (h as i32 - 1, w as i32 - 1);
    let mut vis: HashSet<State> = HashSet::new();
    let mut q = StateQueue::new();
    q.push(WeightState(
        0,
        State {
            x: 0,
            y: 0,
            dir: -Complex::i(),
            dir_steps: 0,
        },
    ));

    let (min_steps_before_turn, max_steps_straight) = if part2 { (4, 10) } else { (0, 3) };

    while let Some(WeightState(dist, ss)) = q.pop() {
        if !vis.insert(ss.clone()) {
            continue;
        }

        let mut move_dir = |d: Complex<i8>, turn: bool| {
            let y = ss.y - d.im as i32;
            let x = ss.x + d.re as i32;
            if y >= 0 && x >= 0 && y <= goal.0 && x <= goal.1 {
                let ns = State {
                    x,
                    y,
                    dir: d,
                    dir_steps: if turn { 1 } else { ss.dir_steps + 1 },
                };
                if !vis.contains(&ns) {
                    q.push(WeightState(
                        dist + input[(y as usize, x as usize)],
                        ns,
                    ));
                }
            }
        };

        if dist == 0 || ss.dir_steps >= min_steps_before_turn {
            if (ss.y, ss.x) == goal {
                return dist;
            }

            // Left turn
            move_dir(ss.dir * Complex::i(), true);

            // Right turn
            move_dir(ss.dir * -Complex::i(), true);
        }

        // Straight
        if ss.dir_steps < max_steps_straight {
            move_dir(ss.dir, false);
        }
    }

    unreachable!()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_int_matrix()?;

    let p1 = run(&input, false);
    println!("Part 1: {}", p1);

    let p2 = run(&input, true);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_int_matrix(aoclib::get_test_input_file!(17)).unwrap();

        let p1 = run(&input, false);
        assert_eq!(p1, 797);

        let p2 = run(&input, true);
        assert_eq!(p2, 914);
    }
}
