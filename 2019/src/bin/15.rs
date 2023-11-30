use std::io;
use std::collections::{VecDeque, HashMap, HashSet};

const NORTH: i64 = 1;
const SOUTH: i64 = 2;
const WEST: i64 = 3;
const EAST: i64 = 4;

fn print_map(map: &HashMap<(i32, i32), char>) {
    let min_y = *map.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();
    let min_x = *map.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *map.keys().map(|(x, _)| x).max().unwrap();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", map.get(&(x, y)).cloned().unwrap_or(' '));
        }
        println!();
    }
}

fn find_ox_spread_time(map: &HashMap<(i32, i32), char>) -> usize {
    let mut q = VecDeque::new();
    let mut v = HashSet::new();
    let mut max_d = 0;
    let (&ox_coords, _) = map.iter().filter(|(_, v)| **v == 'X').next().unwrap();
    q.push_back((0, ox_coords));

    while let Some((d, (x, y))) = q.pop_front() {
        if v.contains(&(x, y)) {
            continue
        }
        v.insert((x, y));

        max_d = d.max(max_d);

        if !v.contains(&(x-1, y)) {
            if *map.get(&(x-1, y)).unwrap_or(&'#') != '#' {
                q.push_back((d+1, (x-1, y)));
            }
        }
        if !v.contains(&(x+1, y)) {
            if *map.get(&(x+1, y)).unwrap_or(&'#') != '#' {
                q.push_back((d+1, (x+1, y)));
            }
        }
        if !v.contains(&(x, y-1)) {
            if *map.get(&(x, y-1)).unwrap_or(&'#') != '#' {
                q.push_back((d+1, (x, y-1)));
            }
        }
        if !v.contains(&(x, y+1)) {
            if *map.get(&(x, y+1)).unwrap_or(&'#') != '#' {
                q.push_back((d+1, (x, y+1)));
            }
        }
    }

    max_d
}

fn run(mem: &Vec<i64>) -> (usize, usize) {
    let mut cmp = aoc2019::IntCodeComputer::new(mem);
    let mut map = HashMap::new();
    // DFS

    fn visit(cmp: &mut aoc2019::IntCodeComputer, map: &mut HashMap<(i32, i32), char>, x: i32, y: i32, dir: i64, d: usize) -> Option<usize> {
        cmp.inputs.push_back(dir);
        cmp.run();

        let stat = cmp.outputs.pop_front().expect("No output!");
        map.insert((x, y), match stat {
            2 => 'X',
            1 => '.',
            0 => '#',
            _ => ' ',
        });

        let mut found = None;
        if stat != 0 {
            if dir != SOUTH && !map.contains_key(&(x, y-1)) {
                found = found.or(visit(cmp, map, x, y-1, NORTH, d+1));
            }
            if dir != NORTH && !map.contains_key(&(x, y+1)) {
                found = found.or(visit(cmp, map, x, y+1, SOUTH, d+1));
            }
            if dir != EAST && !map.contains_key(&(x-1, y)) {
                found = found.or(visit(cmp, map, x-1, y, WEST, d+1));
            }
            if dir != WEST && !map.contains_key(&(x+1, y)) {
                found = found.or(visit(cmp, map, x+1, y, EAST, d+1));
            }
        
            cmp.inputs.push_back(match dir {
                NORTH => SOUTH,
                SOUTH => NORTH,
                EAST => WEST,
                WEST => EAST,
                _ => unreachable!()
            });
            cmp.run();
            cmp.outputs.pop_front();  // consume output
        }

        if stat == 2 {
            return Some(d)
        }
        found
    }

    map.insert((0, 0), '0');
    let found = visit(&mut cmp, &mut map, -1, 0, WEST, 1)
        .or(visit(&mut cmp, &mut map, 1, 0, EAST, 1))
        .or(visit(&mut cmp, &mut map, 0, -1, NORTH, 1))
        .or(visit(&mut cmp, &mut map, 0, 1, SOUTH, 1));
    print_map(&map);

    let ox_spread_time = find_ox_spread_time(&map);
    (found.expect("no generator distance"), ox_spread_time)
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string()?;

    let mem = input.trim().split(",").map(|i| i.parse().unwrap()).collect();

    let (part1, part2) = run(&mem);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}