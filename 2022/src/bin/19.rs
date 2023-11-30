use std::io;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_cost: usize,
    clay_cost: usize,
    obsidian_cost: (usize, usize),
    geode_cost: (usize, usize),
}

#[derive(Clone, Copy)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

impl State {
    fn initial() -> Self {
        State {
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            ore: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            geodes: 0,
        }
    }

    fn buy(&self, rob: Robot, bp: &Blueprint) -> Option<Self> {
        let costs = match rob {
            Robot::Ore if self.ore >= bp.ore_cost => Some((bp.ore_cost, 0, 0)),
            Robot::Clay if self.ore >= bp.clay_cost => Some((bp.clay_cost, 0, 0)),
            Robot::Obsidian
                if self.ore >= bp.obsidian_cost.0 && self.clay >= bp.obsidian_cost.1 =>
            {
                Some((bp.obsidian_cost.0, bp.obsidian_cost.1, 0))
            }
            Robot::Geode if self.ore >= bp.geode_cost.0 && self.obsidian >= bp.geode_cost.1 => {
                Some((bp.geode_cost.0, 0, bp.geode_cost.1))
            }
            _ => None,
        };

        costs.map(|(core, cclay, cobs)| {
            let mut s = self.clone().produce();
            s.ore -= core;
            s.clay -= cclay;
            s.obsidian -= cobs;
            match rob {
                Robot::Ore => s.ore_robots += 1,
                Robot::Clay => s.clay_robots += 1,
                Robot::Obsidian => s.obsidian_robots += 1,
                Robot::Geode => s.geode_robots += 1,
            }
            s
        })
    }

    fn wait_and_buy(
        &self,
        rob: Robot,
        bp: &Blueprint,
        minutes_remaining: usize,
    ) -> Option<(Self, usize)> {
        let can_wait_and_buy = match rob {
            Robot::Ore => true,
            Robot::Clay => true,
            Robot::Obsidian => self.clay_robots > 0,
            Robot::Geode => self.obsidian_robots > 0,
        };
        if !can_wait_and_buy {
            return None;
        }

        let mut ss = self.clone().produce();
        let mut dm = 1;
        while ss.buy(rob, bp).is_none() {
            ss = ss.produce();
            dm += 1;
        }
        if dm < minutes_remaining {
            let ss = ss.buy(rob, bp).unwrap();
            Some((ss, dm + 1))
        } else {
            None
        }
    }

    fn produce(self) -> Self {
        State {
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            ..self
        }
    }
}

fn reward(bp: &Blueprint, minutes_remaining: usize, begin_state: State) -> usize {
    if (minutes_remaining) == 0 {
        return begin_state.geodes;
    }

    let mut res = 0;
    if begin_state.geode_robots > 0 {
        res = begin_state.geodes + minutes_remaining * begin_state.geode_robots;
    }

    let max_ore_robots = bp
        .clay_cost
        .max(bp.ore_cost)
        .max(bp.obsidian_cost.0)
        .max(bp.geode_cost.0);

    if begin_state.ore_robots < max_ore_robots {
        if let Some(s) = begin_state.buy(Robot::Ore, bp) {
            res = res.max(reward(bp, minutes_remaining - 1, s));
        } else if let Some((s, dm)) = begin_state.wait_and_buy(Robot::Ore, bp, minutes_remaining) {
            res = res.max(reward(bp, minutes_remaining - dm, s));
        }
    }

    if begin_state.clay_robots < 10 {
        if let Some(s) = begin_state.buy(Robot::Clay, bp) {
            res = res.max(reward(bp, minutes_remaining - 1, s));
        } else if let Some((s, dm)) = begin_state.wait_and_buy(Robot::Clay, bp, minutes_remaining) {
            res = res.max(reward(bp, minutes_remaining - dm, s));
        }
    }

    if let Some(s) = begin_state.buy(Robot::Obsidian, bp) {
        res = res.max(reward(bp, minutes_remaining - 1, s));
    } else if let Some((s, dm)) = begin_state.wait_and_buy(Robot::Obsidian, bp, minutes_remaining) {
        res = res.max(reward(bp, minutes_remaining - dm, s));
    }

    if let Some(s) = begin_state.buy(Robot::Geode, bp) {
        res = res.max(reward(bp, minutes_remaining - 1, s));
    } else if let Some((s, dm)) = begin_state.wait_and_buy(Robot::Geode, bp, minutes_remaining) {
        res = res.max(reward(bp, minutes_remaining - dm, s));
    }

    res
}

fn part1(input: &[Blueprint]) -> usize {
    input
        .iter()
        .map(|bp| bp.id * reward(bp, 24, State::initial()))
        .sum()
}

fn part2(input: &[Blueprint]) -> usize {
    input
        .iter()
        .take(3)
        .map(|bp| reward(bp, 32, State::initial()))
        .product()
}

fn parse_input(input: Vec<String>) -> Vec<Blueprint> {
    input
        .into_iter()
        .map(|l| aoc2022::read_ints_from_string(&l, false))
        .map(|nums| {
            assert_eq!(7, nums.len());
            Blueprint {
                id: nums[0],
                ore_cost: nums[1],
                clay_cost: nums[2],
                obsidian_cost: (nums[3], nums[4]),
                geode_cost: (nums[5], nums[6]),
            }
        })
        .collect()
}

fn main() -> io::Result<()> {
    let input = parse_input(aoc2022::read_input_lines()?);

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
