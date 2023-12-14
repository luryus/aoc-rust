use itertools::Itertools;
use lru::LruCache;
use std::{io, num::NonZeroUsize};

type ArrangementCache<'a> = LruCache<(&'a[usize], &'a[Option<bool>]), usize>;

#[derive(Clone)]
struct Row {
    springs: Vec<Option<bool>>,
    damaged_group_sizes: Vec<usize>,
}

fn parse_input(input: Vec<String>) -> Vec<Row> {
    input
        .into_iter()
        .map(|l| {
            let (spring_chars, groups) = aoclib::split_to_tuple2(&l, " ").unwrap();
            let damaged_group_sizes = aoclib::read_ints_from_string(groups, false);
            let springs = spring_chars
                .chars()
                .map(|c| match c {
                    '#' => Some(true),
                    '.' => Some(false),
                    '?' => None,
                    _ => panic!("Invalid spring: '{c}'"),
                })
                .collect();

            Row {
                springs,
                damaged_group_sizes,
            }
        })
        .collect()
}

fn arrangements<'a>(
    group_lengths: &'a [usize],
    springs: &'a [Option<bool>],
    cache: &mut ArrangementCache<'a>,
) -> usize {
    if let Some(cached) = cache.get(&(group_lengths, springs)) {
        return *cached;
    }

    if group_lengths.is_empty() {
        if springs.iter().contains(&Some(true)) {
            return 0;
        } else {
            return 1;
        }
    }
    if springs.len() < (group_lengths.iter().sum::<usize>() + group_lengths.len() - 1) {
        return 0;
    }
    let first = springs[0];
    if first == Some(false) {
        return arrangements(group_lengths, &springs[1..], cache);
    }
    if first == Some(true) {
        // If the group can be formed starting from the first position,
        // and the next spring is not damaged, it's valid
        if springs[..group_lengths[0]]
            .iter()
            .all(|&s| s.unwrap_or(true))
        {
            if springs.len() == group_lengths[0] {
                return 1;
            } else if springs[group_lengths[0]] != Some(true) {
                return arrangements(
                    &group_lengths[1..],
                    &springs[(group_lengths[0] + 1)..],
                    cache,
                );
            }
        } 
        
        // If not, this cannot be a valid arrangement
        return 0;
    }

    // First element is unknown.

    // If the spring that would be included in a group starting from the beginning
    // _don't_ include a damaged spring...
    if !springs[..group_lengths[0]].contains(&Some(true)) {
        // ...and there's at least one certainly undamaged spring in the group...
        if let Some((last_undamaged_pos, _)) = springs[..group_lengths[0]]
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == Some(false))
            .last()
        {
            if last_undamaged_pos == springs.len() - 1 {
                // ...bail (this is not valid)
                return 0;
            } else {
                // ...skip springs until the next undamaged or unknown one
                return arrangements(group_lengths, &springs[last_undamaged_pos + 1..], cache);
            }
        }
    }

    // The first element is unknown. Try both ways.

    let mut first_damaged_arrangements = 0;
    if springs.len() >= group_lengths[0]
        && springs[..group_lengths[0]]
            .iter()
            .all(|s| s.unwrap_or(true))
    {
        if springs.len() == group_lengths[0] {
            first_damaged_arrangements = 1;
        } else if !springs[group_lengths[0]].unwrap_or(false) {
            first_damaged_arrangements =
                arrangements(&group_lengths[1..], &springs[group_lengths[0] + 1..], cache)
        }
    }

    let first_undamaged_arrangements = if springs.len() >= 2 {
        arrangements(group_lengths, &springs[1..], cache)
    } else {
        0
    };

    let res = first_damaged_arrangements + first_undamaged_arrangements;
    cache.put((group_lengths, springs), res);
    res
}

fn part1(input: &[Row]) -> usize {
    let mut cache = LruCache::new(NonZeroUsize::new(20).unwrap());
    input
        .iter()
        .map(|r| arrangements(&r.damaged_group_sizes, &r.springs, &mut cache))
        .sum()
}

fn part2(input: &[Row]) -> usize {
    input
        .iter()
        .map(|r| {
            let r = Row {
                // Clippy complains about the vec! being useless, but it's not,
                // because of the .clone(). Changing to an array does not work.
                #[allow(clippy::useless_vec)]
                springs: vec![r.springs.clone(); 5].join(&None),
                damaged_group_sizes: r.damaged_group_sizes.repeat(5),
            };
            let mut cache = LruCache::new(NonZeroUsize::new(20).unwrap());
            arrangements(&r.damaged_group_sizes, &r.springs, &mut cache)
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let input = parse_input(input);

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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(12)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 7771);

        let p2 = part2(&input);
        assert_eq!(p2, 10861030975833);
    }
}
