use arrayvec::ArrayVec;
use itertools::Itertools;
use std::{io, ops::RangeInclusive};

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    bx: i32,
    by: i32,
    r: i32,
}

const MAX_SENSORS: usize = 30;

impl Sensor {
    fn beam_range_at_y(&self, target_y: i32) -> Option<RangeInclusive<i32>> {
        let dy = target_y.abs_diff(self.y) as i32;

        if dy > self.r {
            None
        } else {
            let d_edge = self.r - dy;
            Some((self.x - d_edge)..=(self.x + d_edge))
        }
    }
}

fn dist(x: i32, y: i32, bx: i32, by: i32) -> i32 {
    (x.abs_diff(bx) + y.abs_diff(by)) as i32
}

fn part1(input: &[Sensor], test_mode: bool) -> usize {
    let row = if test_mode { 10 } else { 2_000_000 };

    let ranges = empty_ranges(row, input);

    let overlap_beacons = input
        .iter()
        .filter(|s| s.by == row && ranges.iter().any(|r| r.contains(&s.bx)))
        .map(|s| s.bx)
        .unique()
        .count();
    let overlap_sensors = input
        .iter()
        .filter(|s| s.y == row && ranges.iter().any(|r| r.contains(&s.x)))
        .count();

    ranges
        .into_iter()
        .map(|r| *r.end() - *r.start() + 1)
        .sum::<i32>() as usize
        - overlap_beacons
        - overlap_sensors
}

fn empty_ranges(row: i32, input: &[Sensor]) -> ArrayVec<RangeInclusive<i32>, MAX_SENSORS> {
    let mut ranges: ArrayVec<_, MAX_SENSORS> = input
        .iter()
        .filter_map(|sensor| sensor.beam_range_at_y(row))
        .collect();

    ranges.sort_by_key(|r| *r.start());

    while ranges.len() > 1 {
        let mut new_ranges = ArrayVec::new();
        new_ranges.push(ranges.first().unwrap().clone());

        for r in ranges.iter() {
            let l = new_ranges.pop().unwrap();
            if l.end() >= r.start() {
                new_ranges.push(*l.start()..=*(l.end().max(r.end())));
            } else {
                new_ranges.push(*r.start()..=*r.end());
            }
        }

        if ranges.len() == new_ranges.len() {
            ranges = new_ranges;
            break;
        }
        ranges = new_ranges
    }
    ranges
}

fn part2(input: &[Sensor], test_mode: bool) -> u64 {
    let max_coord = if test_mode { 20 } else { 4_000_000 };

    for yy in 0..=max_coord {
        let rs = empty_ranges(yy, input);
        match rs.len() {
            1 => {
                let (l, r) = (*rs[0].start(), *rs[0].end());
                if l <= 0 && r >= max_coord {
                    continue;
                } else if l == 1 {
                    return yy as u64;
                } else if r == max_coord - 1 {
                    return max_coord as u64 * 4_000_000 + yy as u64;
                } else {
                    panic!("Single range but what??? {rs:?}")
                }
            }
            2 => return (*rs[0].end() + 1) as u64 * 4_000_000 + yy as u64,
            _ => panic!("???"),
        }
    }

    unreachable!()
}

fn parse_input(input: Vec<String>) -> ArrayVec<Sensor, MAX_SENSORS> {
    input
        .into_iter()
        .filter_map(|l| {
            aoclib::read_ints_from_string(&l, true)
                .into_iter()
                .collect_tuple()
        })
        .map(|(x, y, bx, by)| Sensor {
            x,
            y,
            bx,
            by,
            r: dist(x, y, bx, by),
        })
        .collect()
}

fn main() -> io::Result<()> {
    let test_mode = std::env::var("TEST_MODE").is_ok();

    let input = aoclib::read_input_lines()?;
    let input = parse_input(input);

    let p1 = part1(&input, test_mode);
    println!("Part 1: {}", p1);

    let p2 = part2(&input, test_mode);
    println!("Part 2: {}", p2);

    Ok(())
}
