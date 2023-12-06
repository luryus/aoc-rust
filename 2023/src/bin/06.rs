use itertools::Itertools;
use std::io;

struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn will_win(&self, hold_time: usize) -> bool {
        let run_time = self.time - hold_time;
        (run_time * hold_time) > self.distance
    }
}

fn part1(input: &[String]) -> usize {
    let times = aoclib::read_ints_from_string(&input[0], false);
    let dists = aoclib::read_ints_from_string(&input[1], false);
    times
        .into_iter()
        .zip(dists)
        .map(|(t, d)| Race {
            time: t,
            distance: d,
        })
        .map(|r| (1..r.time).filter(|ht| r.will_win(*ht)).count())
        .product1()
        .unwrap()
}

fn part2(input: &[String]) -> usize {
    let time = *aoclib::read_ints_from_string(&input[0].replace(' ', ""), false).first().unwrap();
    let distance = *aoclib::read_ints_from_string(&input[1].replace(' ', ""), false).first().unwrap();

    let race = Race { time, distance };
    // Instead of going through all the options and counting wins, find the first and last
    // winning game. All between will win, too (this is a parabola or something). It's a bit
    // faster to do this. Even faster would be to find these points with binary search?
    let a = (1..race.time).map(|ht| race.will_win(ht)).take_while(|w| !w).count();
    let b = (1..race.time).rev().map(|ht| race.will_win(ht)).take_while(|w| !w).count();
    (1..race.time).len() - a - b
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(6)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 800280);

        let p2 = part2(&input);
        assert_eq!(p2, 45128024);
    }
}
