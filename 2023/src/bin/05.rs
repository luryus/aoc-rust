use itertools::Itertools;
use std::{io, ops::Range};

type Map = (usize, usize, usize);

fn map(mapping: &Vec<Map>, val: usize) -> usize {
    for (drs, srs, len) in mapping {
        if val >= *srs && val < (srs + len) {
            return drs + (val - srs);
        }
    }
    val
}

fn overlap_range(left: Range<usize>, right: Range<usize>) -> Option<Range<usize>> {
    if left.start <= right.start {
        if left.start < right.end && right.start < left.end {
            Some(right.start..left.end.min(right.end))
        } else {
            None
        }
    } else {
        overlap_range(right, left)
    }
}

fn map_range(mapping: &[Map], range: Range<usize>) -> Vec<Range<usize>> {
    let overlap_mapped = mapping
        .iter()
        .copied()
        .filter_map(|m| {
            if let Some(or) = overlap_range(range.clone(), m.1..(m.1 + m.2)) {
                let start = or.start - m.1 + m.0;
                let end = start + or.len();
                Some((or, start..end))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if overlap_mapped.is_empty() {
        return vec![range];
    }

    let mut gaps: Vec<_> = overlap_mapped
        .iter()
        .tuple_windows()
        .map(|(l, r)| l.0.end..r.0.start)
        .collect();
    gaps.push(range.start..overlap_mapped[0].0.start);
    gaps.push(overlap_mapped.last().unwrap().0.end..range.end);

    let res: Vec<_> = gaps
        .into_iter()
        .filter(|r| !r.is_empty())
        .chain(overlap_mapped.into_iter().map(|p| p.1))
        .collect();

    assert_eq!(range.len(), res.iter().map(|r| r.len()).sum());
    res
}

fn map_ranges(mapping: &[Map], ranges: Vec<Range<usize>>) -> Vec<Range<usize>> {
    ranges
        .into_iter()
        .flat_map(|r| map_range(mapping, r))
        .collect()
}

fn part1(seeds: &[usize], maps: &[Vec<Map>]) -> usize {
    seeds
        .iter()
        .copied()
        .map(|s| maps.iter().fold(s, |acc, mapping| map(mapping, acc)))
        .min()
        .unwrap()
}

fn part2(seeds: &[usize], maps: &[Vec<Map>]) -> usize {
    let seed_ranges = seeds
        .iter()
        .copied()
        .tuples()
        .map(|(st, len)| st..(st + len));

    seed_ranges
        .filter_map(|srs| {
            maps.iter()
                .fold(vec![srs], |acc, mapping| map_ranges(mapping, acc))
                .into_iter()
                .map(|r| r.start)
                .min()
        })
        .min()
        .unwrap()
}

fn read_maps(input: &[String]) -> Vec<Map> {
    input
        .iter()
        .skip(1)
        .filter_map(|l| {
            aoclib::read_ints_from_string::<usize>(l, false)
                .into_iter()
                .collect_tuple()
        })
        .sorted_by_key(|m: &Map| m.1)
        .collect()
}

fn parse_input(input: Vec<String>) -> Option<(Vec<usize>, Vec<Vec<Map>>)> {
    let mut parts = input.split(|l| l.is_empty());

    let seeds = aoclib::read_ints_from_string(parts.next()?.first()?, false);

    let maps: Vec<Vec<Map>> = parts.map(read_maps).collect();
    assert_eq!(7, maps.len());
    Some((seeds, maps))
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let (seeds, maps) = parse_input(input).unwrap();

    let p1 = part1(&seeds, &maps);
    println!("Part 1: {}", p1);

    let p2 = part2(&seeds, &maps);
    println!("Part 2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(5)).unwrap();
        let (seeds, maps) = parse_input(input).unwrap();

        let p1 = part1(&seeds, &maps);
        assert_eq!(p1, 379811651);

        let p2 = part2(&seeds, &maps);
        assert_eq!(p2, 27992443);
    }
}
