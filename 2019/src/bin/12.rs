use std::io;
use regex;
use itertools::Itertools;
use std::fmt::Debug;
use aoc2019::OrderingExt;
use num_integer::lcm;

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
struct Vector3 {
    i: i32,
    j: i32,
    k: i32,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct Moon {
    position: Vector3,
    velocity: Vector3,
}

impl Moon {
    fn apply_vel(&mut self) {
        self.position.i += self.velocity.i;
        self.position.j += self.velocity.j;
        self.position.k += self.velocity.k;
    }

    fn gravity_coeffs(&self, other: &Moon) -> Vector3 {
        Vector3 {
            i: -self.position.i.cmp(&other.position.i).as_number() as i32,
            j: -self.position.j.cmp(&other.position.j).as_number() as i32,
            k: -self.position.k.cmp(&other.position.k).as_number() as i32,
        }
    }

    fn energy(&self) -> i32 {
        let pot = self.position.i.abs() + self.position.j.abs() + self.position.k.abs();
        let kin = self.velocity.i.abs() + self.velocity.j.abs() + self.velocity.k.abs();

        pot * kin
    }
}


fn parse_moon(s: &str) -> Moon {
    let regex = regex::Regex::new(r"(-?\d+)").unwrap();

    let mut captures = regex.captures_iter(s);

    let i = captures.next().and_then(|c| c[0].parse::<i32>().ok()).unwrap();
    let j = captures.next().and_then(|c| c[0].parse::<i32>().ok()).unwrap();
    let k = captures.next().and_then(|c| c[0].parse::<i32>().ok()).unwrap();

    Moon {
        position: Vector3 { i: i, j: j, k: k },
        velocity: Vector3 { i: 0, j: 0, k: 0 }
    }
}

fn run1(moons: &Vec<Moon>) -> i32 {
    let mut ms: Vec<Moon> = moons.to_vec();

    let steps = 1000;

    for _ in 0..steps {
        step(&mut ms);
    }

    ms.iter().map(|m| m.energy()).sum()
}

fn step(moons: &mut Vec<Moon>) {
    for (ai, bi) in (0..moons.len()).tuple_combinations() {
        let a = &moons[ai];
        let b = &moons[bi];
        let v = a.gravity_coeffs(b);
        let a = moons.get_mut(ai).unwrap();
        a.velocity.i += v.i;
        a.velocity.j += v.j;
        a.velocity.k += v.k;
        let b = moons.get_mut(bi).unwrap();
        b.velocity.i -= v.i;
        b.velocity.j -= v.j;
        b.velocity.k -= v.k;
    }
    for m in moons.iter_mut() {
        m.apply_vel();
    }
}

fn find_dimen_loop<F: Fn(&Vector3)->i32>(moons: &Vec<Moon>, get_dim: F) -> usize {
    let mut search = moons.to_vec();

    for i in 1.. {
        step(&mut search);

        if search.iter().zip(moons)
            .all(|(s, m)| get_dim(&s.position) == get_dim(&m.position) && get_dim(&s.velocity) == get_dim(&m.velocity)) {
            return i;
        }
    }

    unreachable!();
}

fn run2(moons: &Vec<Moon>) -> usize {
    let loop_i = find_dimen_loop(moons, |v| v.i);
    let loop_j = find_dimen_loop(moons, |v| v.j);
    let loop_k = find_dimen_loop(moons, |v| v.k);

    lcm(lcm(loop_i, loop_j), loop_k)
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_lines()?;

    let moons = input.into_iter()
        .map(|s| parse_moon(&s))
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", run1(&moons));
    println!("Part 2: {:?}", run2(&moons));

    Ok(())
}