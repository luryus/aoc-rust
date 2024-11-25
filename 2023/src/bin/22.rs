use itertools::Itertools;
use ndarray::{s, Array3, Axis};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

type Coord = (usize, usize, usize);

fn sub(a: Coord, other: Coord) -> Coord {
    (a.0 - other.0, a.1 - other.1, a.2 - other.2)
}

#[derive(Clone, Debug)]
struct Brick(u16, Coord, Coord);

impl Brick {
    fn min_z(&self) -> usize {
        self.1 .0.min(self.2 .0)
    }
    fn max_z(&self) -> usize {
        self.1 .0.max(self.2 .0)
    }

    fn min_x(&self) -> usize {
        self.1 .2.min(self.2 .2)
    }

    fn max_x(&self) -> usize {
        self.1 .2.max(self.2 .2)
    }

    fn min_y(&self) -> usize {
        self.1 .1.min(self.2 .1)
    }

    fn max_y(&self) -> usize {
        self.1 .1.max(self.2 .1)
    }

    fn drop(&self) -> Option<Self> {
        if self.min_z() <= 1 {
            return None;
        }
        let a = (self.1 .0 - 1, self.1 .1, self.1 .2 );
        let b = (self.2 .0 -1, self.2 .1, self.2 .2);
        Some(Self(self.0, a, b))
    }
}

fn overlaps(grid: &Array3<Option<u16>>, b: &Brick) -> bool {
    grid.slice(s![b.1 .0..=b.2 .0, b.1 .1..=b.2 .1, b.1 .2..=b.2 .2])
        .iter()
        .any(|x| x.is_some())
}

fn run(input: Vec<Brick>) -> (usize, usize) {
    let minx = input.iter().map(|b| b.min_x()).min().unwrap();
    let miny = input.iter().map(|b| b.min_y()).min().unwrap();
    let minz = input.iter().map(|b| b.min_z()).min().unwrap();

    assert!(minz > 0);

    let dx = input.iter().map(|b| b.max_x()).max().unwrap() - minx + 1;
    let dy = input.iter().map(|b| b.max_y()).max().unwrap() - miny + 1;
    let dz = input.iter().map(|b| b.max_z()).max().unwrap() - minz + 2;
    let mut grid: Array3<Option<u16>> = Array3::default((dz, dy, dx));

    let input: Vec<_> = input
        .into_iter()
        .map(|b| {
            Brick(
                b.0,
                sub(b.1, (minz - 1, miny, minx)),
                sub(b.2, (minz - 1, miny, minx)),
            )
        })
        .collect();


    for b in input.iter().sorted_by_key(|b| b.min_z()) {
        let mut b = b.clone();
        assert!(!overlaps(&grid, &b));
        while let Some(db) = b.drop() {
            if overlaps(&grid, &db) {
                break;
            } else {
                b = db;
            }
        }

        for ele in grid
            .slice_mut(s![b.1 .0..=b.2 .0, b.1 .1..=b.2 .1, b.1 .2..=b.2 .2])
            .iter_mut()
        {
            *ele = Some(b.0);
        }
    }

    let top_z = grid
        .axis_iter(Axis(0))
        .enumerate()
        .skip(1)
        .find(|(_, s)| s.iter().all(|x| x.is_none()))
        .unwrap()
        .0;

    let mut supporting_bricks: HashMap<u16, HashSet<u16>> = HashMap::default();
    let mut supported_bricks: HashMap<u16, HashSet<u16>> = HashMap::default();

    for wind in grid.axis_windows(Axis(0), 2).into_iter().take(top_z).skip(1) {
        let (dz, dy, dx) = wind.dim();
        assert_eq!(dz, 2);

        for (x, y) in (0..dx).cartesian_product(0..dy) {
            let top = wind[(1, y, x)];
            let bot = wind[(0, y, x)];

            if let Some(top_id) = top {
                if let Some(bot_id) = bot {
                    if top_id != bot_id {
                        supporting_bricks.entry(top_id).or_default().insert(bot_id);
                        supported_bricks.entry(bot_id).or_default().insert(top_id);
                    }
                }
            }
        }
    }

    let mut safe = vec![];

    for b in &input {
        let supported = supported_bricks.entry(b.0).or_default();
        if supported.is_empty() || supported.iter().all(|id| supporting_bricks[id].len() > 1) {
            safe.push(b.0);
        }
    }


    let mut other_bricks_fall_sum = 0usize;

    for bid in input.iter().map(|b| b.0) {
        let mut fall = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(bid);

        while let Some(fb) = q.pop_front() {
            if !fall.insert(fb) {
                continue
            }

            let supported = supported_bricks.entry(fb).or_default();
            for s in supported.iter() {
                if supporting_bricks[s].iter().all(|sb| fall.contains(sb)) {
                    q.push_back(*s);
                }
            }
        }

        other_bricks_fall_sum += fall.len() - 1;
    }


    (safe.len(), other_bricks_fall_sum)
}


fn parse_input(input: Vec<String>) -> Vec<Brick> {
    input
        .into_iter()
        .map(|l| aoclib::read_ints_from_string(&l, false))
        .enumerate()
        .map(|(i, nums)| {
            assert_eq!(nums.len(), 6);
            Brick(
                (i + 1) as u16,
                nums[..3].iter().rev().copied().collect_tuple().unwrap(),
                nums[3..].iter().rev().copied().collect_tuple().unwrap(),
            )
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let input = parse_input(input);

    let (p1, p2) = run(input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(22)).unwrap();
        let input = parse_input(input);

        let (p1, p2) = run(input);
        assert_eq!(p1, 432);
        assert_eq!(p2, 63166);
    }
}
