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

fn adjacent_indices(idx: usize, w: usize, h: usize) -> [Option<usize>; 4] {
    let up = if idx > w-1 { Some(idx - w) } else { None };
    let down = if idx < (h-1) * w { Some(idx + w) } else { None };
    let left = if (idx % w) > 0 { Some(idx - 1) } else { None };
    let right = if (idx % w) < (w-1) { Some(idx + 1) } else { None };

    [up, down, left, right]
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