use aoc2019::read_stdin_lines;
use std::io;
use std::vec::Vec;
use std::collections::HashSet;

fn part1(map: Vec<bool>, w: usize, h: usize) -> u32 {

    let mut seen = HashSet::new();

    let mut prev = map;

    while !seen.contains(&prev) {
        let next = next(&prev, w, h);
        seen.insert(prev);
        prev = next;
    }


    prev.into_iter().enumerate().filter(|(_, cell)| *cell)
        .map(|(idx, _)| 1 << idx)
        .sum()
}

fn part2(map: Vec<bool>, w: usize, h: usize) -> u32 {

    let mut prev = Vec::new();
    let mut min_level = 102;
    let mut max_level = 102;
    prev.resize_with(204, || Vec::new());
    prev[102] = map;
    

    for _ in 0..200 {
        let next = next2(&prev, w, h, min_level, max_level);
        prev = next.0;
        min_level = next.1;
        max_level = next.2;
    }

    prev.into_iter()
        .flatten()
        .map(|x| x as u32)
        .sum()
}

fn next(prev: &Vec<bool>, w: usize, h: usize) -> Vec<bool> {
    let mut next = Vec::with_capacity(prev.len());

    for (idx, cell) in prev.iter().enumerate() {
        let adj_count: u8 = adjacent_indices(idx, w, h).iter().flatten().map(|i| prev[*i] as u8).sum();


        next.push(match *cell {
            true if adj_count != 1 => false,
            false if adj_count == 1 || adj_count == 2 => true,
            x => x 
        });
    }
    
    next
}

fn next2(prev: &Vec<Vec<bool>>, w: usize, h: usize, min_level: usize, max_level: usize) -> (Vec<Vec<bool>>, usize, usize) {
    let mut next = Vec::new();
    next.resize_with(prev.len(), || Vec::new());

    for l in min_level..=max_level {
        for (idx, cell) in prev[l].iter().enumerate() {
            if idx == h*w/2 {
                next[l].push(false);
                continue;
            }

            let adj_count: u8 = adjacent_indices2((l, idx), w, h)
                .into_iter()
                .map(|(level, i)| if level < min_level || level > max_level {
                    0
                } else {
                    prev[level][i] as u8
                }).sum();

            next[l].push(match *cell {
                true if adj_count != 1 => false,
                false if adj_count == 1 || adj_count == 2 => true,
                x => x 
            });
        }
    }

    // Min_level - 1
    for idx in 0..h*w {
        if idx == h*w/2 {
            next[min_level-1].push(false);
            continue;
        }
        let adj_count: u8 = adjacent_indices2((min_level-1, idx), w, h)
            .into_iter()
            .map(|(level, i)| if level < min_level || level > max_level {
                0
            } else {
                prev[level][i] as u8
            }).sum();
        next[min_level-1].push(idx != h*w/2 && (adj_count == 1 || adj_count == 2));
    }

    // Max_level + 1
    for idx in 0..h*w {
        if idx == h*w/2 {
            next[max_level+1].push(false);
            continue;
        }
        let adj_count: u8 = adjacent_indices2((max_level+1, idx), w, h)
        .into_iter()
        .map(|(level, i)| if level < min_level || level > max_level {
            0
        } else {
            prev[level][i] as u8
        }).sum();
        next[max_level+1].push(idx != h*w/2 && (adj_count == 1 || adj_count == 2));
    }

    let new_min = min_level - next[min_level-1].iter().any(|x| *x) as usize;
    let new_max = max_level + next[max_level+1].iter().any(|x| *x) as usize;

    (next, new_min, new_max)
}

fn adjacent_indices(idx: usize, w: usize, h: usize) -> [Option<usize>; 4] {
    let up = if idx > w-1 { Some(idx - w) } else { None };
    let down = if idx < (h-1) * w { Some(idx + w) } else { None };
    let left = if (idx % w) > 0 { Some(idx - 1) } else { None };
    let right = if (idx % w) < (w-1) { Some(idx + 1) } else { None };

    [up, down, left, right]
}

fn adjacent_indices2(coord: (usize, usize),  w: usize, h: usize) -> Vec<(usize, usize)> {
    let (level, idx) = coord;

    let mid = h*w / 2;

    let mut res = Vec::new();

    // Up
    if idx > w-1 { 
        if idx - w == mid {
            for i in (h-1)*w..h*w {
                res.push((level+1, i));
            }
        } else {
            res.push((level, idx-w));
        }
    } else { 
        res.push((level-1, h*w / 2 - w));
    }

    // Down
    if idx < (h-1) * w {
        if idx + w == mid {
            for i in 0..w {
                res.push((level+1, i));
            }
        } else {
            res.push((level, idx+w));
        }
    } else {
        res.push((level-1, h*w/2 + w));
    }

    // Left
    if (idx % w) > 0 {
        if idx-1 == mid {
            for i in 1..=h {
                res.push((level+1, i*w-1));
            }
        } else {
            res.push((level, idx-1));
        }
    } else {
        res.push((level-1, h*w / 2 - 1));
    }

    // Right
    if (idx % w) < (w-1) {
        if idx + 1 == mid {
            for i in 0..h {
                res.push((level+1, i*w));
            }
        } else {
            res.push((level, idx + 1));
        }
    } else {
        res.push((level -1, h*w / 2 + 1));
    }


    res
}

fn main() -> io::Result<()> {
    let input_lines = read_stdin_lines()?;
    let (w, h) = (input_lines.first().unwrap().trim().len(), input_lines.len());
    let input = input_lines
        .iter()
        .flat_map(|l| {
            l.chars()
                .filter_map(|c| match c {
                    '#' => Some(true),
                    '.' => Some(false),
                    _ => None,
                })
        })
        .collect::<Vec<bool>>();

    println!("Part 1: {}", part1(input.clone(), w, h));
    println!("Part 2: {}", part2(input, w, h));

    Ok(())
}

#[test]
fn test_adjacent_indices() {
    let res0 = adjacent_indices(0, 5, 5);
    assert_eq!(2, res0.iter().filter(|&&x| x == None).count());
    assert!(res0.iter().any(|&x| x == Some(1)));
    assert!(res0.iter().any(|&x| x == Some(5)));

    let res5 = adjacent_indices(5, 5, 5);
    assert_eq!(1, res5.iter().filter(|&&x| x == None).count());
    assert!(res5.iter().any(|&x| x == Some(6)));
    assert!(res5.iter().any(|&x| x == Some(0)));
    assert!(res5.iter().any(|&x| x == Some(10)));

    let res4 = adjacent_indices(4, 5, 5);
    assert_eq!(2, res4.iter().filter(|&&x| x == None).count());
    assert!(res4.iter().any(|&x| x == Some(3)));
    assert!(res4.iter().any(|&x| x == Some(9)));

    let res24 = adjacent_indices(24, 5, 5);
    assert_eq!(2, res24.iter().filter(|&&x| x == None).count());
    assert!(res24.iter().any(|&x| x == Some(23)));
    assert!(res24.iter().any(|&x| x == Some(19)));

    let res12 = adjacent_indices(12, 5, 5);
    assert_eq!(0, res12.iter().filter(|&&x| x == None).count());
    assert!(res12.iter().any(|&x| x == Some(11)));
    assert!(res12.iter().any(|&x| x == Some(13)));
    assert!(res12.iter().any(|&x| x == Some(7)));
    assert!(res12.iter().any(|&x| x == Some(17)));
}

#[test]
fn test_adjacent_indices2() {
    let res0 = adjacent_indices2((1, 0), 5, 5);
    assert_eq!(4, res0.len());
    assert!(res0.contains(&(0, 11)));
    assert!(res0.contains(&(0, 7)));
    assert!(res0.contains(&(1, 1)));
    assert!(res0.contains(&(1, 5)));

    let res7 = adjacent_indices2((1, 7), 5, 5);
    assert_eq!(8, res7.len());
    assert!(res7.contains(&(2, 0)));
    assert!(res7.contains(&(2, 1)));
    assert!(res7.contains(&(2, 2)));
    assert!(res7.contains(&(2, 3)));
    assert!(res7.contains(&(2, 4)));
    assert!(res7.contains(&(1, 6)));
    assert!(res7.contains(&(1, 8)));
    assert!(res7.contains(&(1, 2)));

    let res17 = adjacent_indices2((1, 17), 5, 5);
    assert_eq!(8, res17.len());
    assert!(res17.contains(&(2, 20)));
    assert!(res17.contains(&(2, 21)));
    assert!(res17.contains(&(2, 22)));
    assert!(res17.contains(&(2, 23)));
    assert!(res17.contains(&(2, 24)));
    assert!(res17.contains(&(1, 16)));
    assert!(res17.contains(&(1, 18)));
    assert!(res17.contains(&(1, 22)));

    let res22 = adjacent_indices2((1, 22), 5, 5);
    assert_eq!(4, res22.len());
    assert!(res22.contains(&(1, 23)));
    assert!(res22.contains(&(1, 21)));
    assert!(res22.contains(&(1, 17)));
    assert!(res22.contains(&(0, 17)));

    let res14 = adjacent_indices2((1, 14), 5, 5);
    assert_eq!(4, res14.len());
    assert!(res14.contains(&(1, 13)));
    assert!(res14.contains(&(1, 9)));
    assert!(res14.contains(&(1, 19)));
    assert!(res14.contains(&(0, 13)));

    let res13 = adjacent_indices2((1, 13), 5, 5);
    assert_eq!(8, res13.len());
    assert!(res13.contains(&(1, 14)));
    assert!(res13.contains(&(1, 8)));
    assert!(res13.contains(&(1, 18)));
    assert!(res13.contains(&(2, 4)));
    assert!(res13.contains(&(2, 9)));
    assert!(res13.contains(&(2, 14)));
    assert!(res13.contains(&(2, 19)));
    assert!(res13.contains(&(2, 24)));
}