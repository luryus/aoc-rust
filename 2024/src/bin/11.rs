use std::{collections::HashMap, io};
use num_integer::Integer;

fn num_digits(num: u64) -> u32 {
    ((num as f32).log10().floor() as u32) + 1
}

fn split(num: u64) -> Option<(u64, u64)> {
    let n = num_digits(num);
    n.is_even().then(|| {
        let m = 10u64.pow(n/2);
        let l = num / m;
        let r = num % m;
        (l, r)
    })
}

fn check<const ROUNDS: u32>(level: u32, num: u64, cache: &mut HashMap<(u32, u64), usize>) -> usize {
    if level == ROUNDS {
        return 1;
    }

    if let Some(cached) = cache.get(&(level,num)) {
        return *cached;
    }

    let res = if num == 0 {
        check::<ROUNDS>(level + 1, 1, cache)
    } else if let Some((l, r)) = split(num) {
        check::<ROUNDS>(level + 1, l, cache) + check::<ROUNDS>(level + 1, r, cache)
    } else {
        check::<ROUNDS>(level + 1, num * 2024, cache)
    };

    cache.insert((level, num), res);
    res
}

fn part1(input: &[u64]) -> usize {
    let mut cache = Default::default();
    input.iter()
        .map(|n| check::<25>(0, *n, &mut cache))
        .sum()
}


fn part2(input: &[u64]) -> usize {
    let mut cache = Default::default();
    input.iter()
        .map(|n| check::<75>(0, *n, &mut cache))
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_ints(false)?;

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
        let input = aoclib::read_ints_from_file(aoclib::get_test_input_file!(11), false).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 186175);

        let p2 = part2(&input);
        assert_eq!(p2, 220566831337810);
    }
}
