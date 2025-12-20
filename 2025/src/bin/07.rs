use std::{collections::HashMap, io};
use ndarray::Array2;

fn part1(input: &Array2<char>) -> usize {
    let mut map = input.clone();
    // First and 2nd line special
    let (h, w) = map.dim();

    for x in 0..w {
        if map[(0, x)] == 'S' {
            map[(1, x)] = '|';
            break;
        }
    }

    // Fill beams
    let mut splits = 0;
    for y in 2..h {
        for x in 0..w {
            if map[(y, x)] == '^' && map[(y - 1, x)] == '|' {
                map[(y, x-1)] = '|';
                map[(y, x+1)] = '|';
                splits += 1;
            } else if map[(y, x)] == '.' && map[(y-1, x)] == '|' {
                map[(y, x)] = '|';
            }
        }
    }

    splits
}


fn get_timeline_count(map: &Array2<char>, y: usize, x: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(cached) = cache.get(&(y, x)) {
        return *cached;
    }

    if y == (map.dim().0 -1) {
        return 1;
    }

    let res = match map[(y, x)] {
        '.'|'S' => get_timeline_count(map, y + 1, x, cache),
        '^' => get_timeline_count(map, y, x - 1, cache) + get_timeline_count(map, y, x + 1, cache),
        _ => unreachable!()
    };

    cache.insert((y, x), res);

    res
}

fn part2(input: &Array2<char>) -> usize {
    let mut cache = HashMap::new();

    let (_, w) = input.dim();
    for x in 0..w {
        if input[(0, x)] == 'S' {
            return get_timeline_count(input, 0, x, &mut cache);
        }
    }

    unreachable!()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_char_matrix()?;

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
        let input = aoclib::read_file_char_matrix(aoclib::get_test_input_file!(7)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 1656);

        let p2 = part2(&input);
        assert_eq!(p2, 76624086587804);
    }
}
