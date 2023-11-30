use std::{io, str::FromStr};

enum GameResult {
    Lost = 0,
    Draw = 3,
    Won = 6,
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lost),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Won),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    fn draw(&self) -> Choice {
        *self
    }

    fn win(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Paper,
            Choice::Paper => Choice::Scissors,
            Choice::Scissors => Choice::Rock,
        }
    }

    fn lose(&self) -> Choice {
        match self {
            Choice::Rock => Choice::Scissors,
            Choice::Paper => Choice::Rock,
            Choice::Scissors => Choice::Paper,
        }
    }

    fn result(&self, other: &Self) -> GameResult {
        if self == other {
            GameResult::Draw
        } else if self == &other.win() {
            GameResult::Won
        } else {
            GameResult::Lost
        }
    }
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            "X" => Ok(Choice::Rock),
            "Y" => Ok(Choice::Paper),
            "Z" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

fn part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(a, b)| (a.parse::<Choice>().unwrap(), b.parse::<Choice>().unwrap()))
        .map(|(a, b)| (b, b.result(&a)))
        .map(|(b, r)| (b as usize) + (r as usize))
        .sum()
}

fn part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(a, b)| {
            (
                a.parse::<Choice>().unwrap(),
                b.parse::<GameResult>().unwrap(),
            )
        })
        .map(|(a, r)| {
            (
                match &r {
                    GameResult::Lost => a.lose(),
                    GameResult::Draw => a.draw(),
                    GameResult::Won => a.win(),
                },
                r,
            )
        })
        .map(|(b, r)| (b as usize) + (r as usize))
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoc2022::read_input_lines()?;

    let p1 = part1(&input);
    println!("Part 1: {}", p1);

    let p2 = part2(&input);
    println!("Part 2: {}", p2);

    Ok(())
}
