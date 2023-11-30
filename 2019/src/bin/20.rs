use std::io;
use aoc2019::{read_stdin_lines};
use std::collections::{VecDeque, BTreeMap,BTreeSet};
use itertools::Itertools;

fn parse_maze(input: Vec<String>) -> (Vec<Vec<Option<bool>>>, BTreeMap<(usize, usize), (usize, usize)>, (usize, usize), (usize, usize)) {
    let hole_x_start = (&input[input.len() / 2][2..]).chars().enumerate()
        .skip_while(|(_, c)| *c == '.' || *c == '#').map(|(i, _)| i).next().unwrap();
    let hole_x_end = (&input[input.len() / 2][2..]).chars().enumerate()
        .skip_while(|(_, c)| *c == '.' || *c == '#')
        .skip_while(|(_, c)| *c != '.' && *c != '#')
        .map(|(i, _)| i).next().unwrap() - 1;

    let center_y = input[2].len() / 2;
    let hole_y_start = input.iter().map(|l| l.chars().nth(center_y).unwrap()).skip(2).enumerate()
        .skip_while(|(_, c)| *c == '.' || *c == '#')
        .map(|(i, _)| i).next().unwrap();
    let hole_y_end = input.iter().skip(2)
        .map(|l| l.chars().nth(center_y).unwrap()).enumerate()
        .skip_while(|(_, c)| *c == '.' || *c == '#')
        .skip_while(|(_, c)| *c != '.' && *c != '#')
        .map(|(i, _)| i).next().unwrap() - 1;


    let maze = input[2..(input.len()-2)].iter().enumerate()
        .map(|(row, l)| {
            (&l[2..(l.len()-2)]).chars().enumerate().map(|(col, c)| {
                if row >= hole_y_start && row <= hole_y_end && col >= hole_x_start && col <= hole_x_end {
                    None
                } else {
                    Some(c == '#')
                }
            }).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut start = (0, 0);
    let mut end = (0, 0);
    let mut warps: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut warp_pairs = BTreeMap::new();

    for (i, l) in input[2..input.len()-2].iter().enumerate() {
        if &l[..2] == "AA" {
            start = (0, i);
        } else if &l[..2] == "ZZ" {
            end = (0, i);
        } else if &l[..2] != "  " {
            warps.insert((&l[..2]).to_owned(), (0, i));
        }

        let e = l.len() - 2;
        if &l[e..] == "AA" {
            start = (e-3, i);
        } else if &l[e..] == "ZZ" {
            end = (e-3, i);
        } else if &l[e..] != "  " {
            warps.insert((&l[e..]).to_owned(), (e-3, i));
        }
    }

    let l = &input[0];
    for (i, c) in l.chars().skip(2).enumerate() {
        if c != ' ' {
            let name = [c, input[1].chars().nth(i+2).unwrap()].iter().collect::<String>();
            if &name == "AA" {
                start = (i, 0);
            } else if &name == "ZZ" {
                end = (i, 0);
            } else {
                warps.insert(name, (i, 0));
            }
        }
    }
    let l = input.last().unwrap();
    for (i, c) in l.chars().skip(2).enumerate() {
        if c != ' ' {
            let name = [input[input.len() - 2].chars().nth(i+2).unwrap(), c].iter().collect::<String>();
            if &name == "AA" {
                start = (i, input.len() - 5);
            } else if &name == "ZZ" {
                end = (i, input.len() - 5);
            } else {
                warps.insert(name, (i, input.len() - 5));
            }
        }
    }

    for (i, l) in input.iter().skip(2+hole_y_start).take(hole_y_end-hole_y_start+1).enumerate() {
        if &l[1+hole_x_start..=1+hole_x_start] == "." {
            let name = &l[2+hole_x_start..hole_x_start+4];
            let pair = warps.remove(name).expect(&format!("No name match found for {}", &name));
            warp_pairs.insert(pair, (hole_x_start-1, i+hole_y_start));
            warp_pairs.insert((hole_x_start-1, i+hole_y_start), pair);
        }

        if &l[hole_x_end+3..=hole_x_end+3] == "." {
            let name = &l[hole_x_end+1..hole_x_end+3];
            let pair = warps.remove(name).expect(&format!("No name match found for {}", &name));
            warp_pairs.insert(pair, (hole_x_end+1, i+hole_y_start));
            warp_pairs.insert((hole_x_end+1, i+hole_y_start), pair);
        }
    }

    let l = &input[1+hole_y_start];
    for (i, c) in l.chars().skip(2).enumerate().skip(hole_x_start).take(hole_x_end-hole_x_start+1) {
        if c == '.' {
            let name = [input[hole_y_start+2].chars().nth(i+2).unwrap(), input[hole_y_start+3].chars().nth(i+2).unwrap()].iter().collect::<String>();
            let pair = warps.remove(&name).expect(&format!("No name match found for {}", &name));

            warp_pairs.insert(pair, (i, hole_x_start-1));
            warp_pairs.insert((i, hole_x_start-1), pair);
        }
    }
    let l = &input[3+hole_y_end];
    for (i, c) in l.chars().skip(2).enumerate().skip(hole_x_start).take(hole_x_end-hole_x_start+1) {
        if c == '.' {
            let name = [input[hole_y_end+1].chars().nth(i+2).unwrap(), input[hole_y_end+2].chars().nth(i+2).unwrap()].iter().collect::<String>();
            let pair = warps.remove(&name).expect(&format!("No name match found for {}", &name));

            warp_pairs.insert(pair, (i, hole_y_end+1));
            warp_pairs.insert((i, hole_y_end+1), pair);
        }
    }

    (maze, warp_pairs, start, end)
}

fn fill_maze(mut maze: Vec<Vec<Option<bool>>>) -> Vec<Vec<Option<bool>>> {
    let mut any_filled = true;
    while any_filled {
        any_filled = false;
        for (y, x) in (1..maze.len()-1).cartesian_product(1..maze[0].len()-1) {
            let mut c = 0;
            if maze[y][x] == None || maze[y][x] == Some(true) {
                continue;
            }
            if maze[y][x-1] == Some(true) {
                c += 1;
            }
            if maze[y][x+1] == Some(true) {
                c += 1;
            }
            if maze[y-1][x] == Some(true) {
                c += 1;
            }
            if maze[y+1][x] == Some(true) {
                c += 1;
            }

            if c >= 3 {
                maze[y][x] = Some(true);
                any_filled = true;
            }
        }
    }

    maze
}

fn run1(maze: &Vec<Vec<Option<bool>>>, warps: &BTreeMap<(usize, usize), (usize, usize)>, start: (usize, usize), end: (usize, usize)) {
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    let h = maze.len();
    let w = maze.first().unwrap().len();
    while let Some(((x, y), d)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }

        if (x, y) == end {
            println!("Part 1: {}", d);
            return;
        }

        visited.insert((x, y));

        let coord = if (x == 0 || maze[y][x-1] == None) {
            if (x, y) != start {
                Some(*warps.get(&(x, y)).unwrap())
            } else { None }
        } else if let Some(false) = maze[y][x-1] {
            Some((x-1, y))
        } else { 
            None
        };
        if let Some(c) = coord {
            if !visited.contains(&c) {
                queue.push_back((c, d+1));
            }
        }

        let coord = if (x == w-1 || maze[y][x+1] == None) {
            if (x, y) != start {
                Some(*warps.get(&(x, y)).unwrap())
            } else { None }
        } else if let Some(false) = maze[y][x+1] {
            Some((x+1, y))
        } else { 
            None
        };
        if let Some(c) = coord {
            if !visited.contains(&c) {
                queue.push_back((c, d+1));
            }
        }

        let coord = if (y == 0 || maze[y-1][x] == None) {
            if (x, y) != start {
                Some(*warps.get(&(x, y)).unwrap())
            } else { None }
        } else if let Some(false) = maze[y-1][x] {
            Some((x, y-1))
        } else { 
            None
        };
        if let Some(c) = coord {
            if !visited.contains(&c) {
                queue.push_back((c, d+1));
            }
        }

        let coord = if (y == h-1 || maze[y+1][x] == None) {
            if (x, y) != start {
                Some(*warps.get(&(x, y)).unwrap())
            } else { None }
        } else if let Some(false) = maze[y+1][x] {
            Some((x, y+1))
        } else { 
            None
        };
        if let Some(c) = coord {
            if !visited.contains(&c) {
                queue.push_back((c, d+1));
            }
        }
    }
}

fn run2(maze: &Vec<Vec<Option<bool>>>, warps: &BTreeMap<(usize, usize), (usize, usize)>, start: (usize, usize), end: (usize, usize)) {
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(((start.0, start.1, 0), 0));
    let h = maze.len();
    let w = maze.first().unwrap().len();
    while let Some(((x, y, level), d)) = queue.pop_front() {
        if visited.contains(&(x, y, level)) {
            continue;
        }

        if (x, y) == end && level == 0 {
            println!("Part 2: {}", d);
            return;
        }

        visited.insert((x, y, level));

        let coord = if x == 0 || maze[y][x-1] == None {
            warps.get(&(x, y)).copied().map(|(wx, wy)| (wx, wy, if x == 0 { level - 1 } else { level + 1 }))
        } else if let Some(false) = maze[y][x-1] {
            Some((x-1, y, level))
        } else { 
            None
        };
        if let Some(c) = coord {
            if !visited.contains(&c) && c.2 >= 0 {
                queue.push_back((c, d+1));
            }
        }

        let coord = if (x == w-1 || maze[y][x+1] == None) {
            warps.get(&(x, y)).copied().map(|(wx, wy)| (wx, wy, if x == w-1 { level - 1 } else { level + 1 }))
        } else if let Some(false) = maze[y][x+1] {
            Some((x+1, y, level))
        } else { 
            None
        };
        if let Some(c) = coord {
            if !visited.contains(&c) && c.2 >= 0 {
                queue.push_back((c, d+1));
            }
        }

        let coord = if (y == 0 || maze[y-1][x] == None) {
            warps.get(&(x, y)).copied().map(|(wx, wy)| (wx, wy, if y == 0 { level - 1 } else { level + 1 }))
        } else if let Some(false) = maze[y-1][x] {
            Some((x, y-1, level))
        } else { 
            None
        };
        if let Some(c) = coord {
            if !visited.contains(&c) && c.2 >= 0 {
                queue.push_back((c, d+1));
            }
        }

        let coord = if (y == h-1 || maze[y+1][x] == None) {
            warps.get(&(x, y)).copied().map(|(wx, wy)| (wx, wy, if y == h-1 { level - 1 } else { level + 1 }))
        } else if let Some(false) = maze[y+1][x] {
            Some((x, y+1, level))
        } else { 
            None
        };
        if let Some(c) = coord {
            if !visited.contains(&c) && c.2 >= 0 {
                queue.push_back((c, d+1));
            }
        }
    }
}

fn main() -> io::Result<()> {
    let input = read_stdin_lines()?;

    let (maze, warps, start, end) = parse_maze(input);
    let maze = fill_maze(maze);

    println!("{:?}", &warps);

    run1(&maze, &warps, start, end);
    run2(&maze, &warps, start, end);

    Ok(())
}