use std::io;
use std::collections::{BTreeSet, BTreeMap, HashMap, VecDeque, HashSet, BinaryHeap};
use std::cmp::{Ordering, Reverse};
use itertools::Itertools;

fn run1(map: &Vec<Vec<char>>, start: (usize, usize), keys: &Vec<(char, usize, usize)>) {
    let all_keys = keys.iter()
        .filter(|(c, _, _)| c.is_lowercase())
        .map(|(c, _, _)| *c)
        .collect::<Vec<_>>();
    let all_doors = keys.iter()
        .filter(|(c, _, _)| c.is_uppercase())
        .map(|(c, _, _)| *c)
        .collect::<Vec<_>>();

    let mut visited_states = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start.0, start.1, 0, BTreeSet::new()));

    while let Some((x, y, d, mut collected_keys)) = queue.pop_front() {
        //print!("\r  {}", d);
        if visited_states.contains(&(x, y, collected_keys.clone())) {
            continue;
        }
        let curr = map[y][x];
        if all_keys.contains(&map[y][x]) && !collected_keys.contains(&map[y][x]) {
            collected_keys.insert(curr);

            if collected_keys.len() >= all_keys.len() {
                println!("Part 1: {}", d);
                break;
            }
        }

        visited_states.insert((x, y, collected_keys.clone()));

        if map[y][x-1] != '#' && (!all_doors.contains(&map[y][x-1]) || collected_keys.contains(&map[y][x-1].to_lowercase().next().unwrap())) {
            let st = (x-1, y, collected_keys.clone());
            let q = (x-1, y, d+1, collected_keys.clone());
            if !visited_states.contains(&st) && !queue.contains(&q) {
                queue.push_back(q);
            }
        }
        if map[y][x+1] != '#' && (!all_doors.contains(&map[y][x+1]) || collected_keys.contains(&map[y][x+1].to_lowercase().next().unwrap())) {
            let st = (x+1, y, collected_keys.clone());
            let q = (x+1, y, d+1, collected_keys.clone());
            if !visited_states.contains(&st) && !queue.contains(&q) {
                queue.push_back(q);
            }
        }
        if map[y-1][x] != '#' && (!all_doors.contains(&map[y-1][x]) || collected_keys.contains(&map[y-1][x].to_lowercase().next().unwrap())) {
            let st = (x, y-1, collected_keys.clone());
            let q = (x, y-1, d+1, collected_keys.clone());
            if !visited_states.contains(&st) && !queue.contains(&q) {
                queue.push_back(q);
            }
        }
        if map[y+1][x] != '#' && (!all_doors.contains(&map[y+1][x]) || collected_keys.contains(&map[y+1][x].to_lowercase().next().unwrap())) {
            let st = (x, y+1, collected_keys.clone());
            let q = (x, y+1, d+1, collected_keys.clone());
            if !visited_states.contains(&st) && !queue.contains(&q) {
                queue.push_back(q);
            }
        }
    }
    
}

fn advance(map: &Vec<Vec<char>>, collected_keys: &BTreeSet<char>, all_keys: &Vec<char>, all_doors: &Vec<char>, x: usize, y: usize, d: usize, came_from: (usize, usize)) -> Option<Vec<((usize, usize), usize, BTreeSet<char>)>> {
    let curr = map[y][x];
    if curr == '#' || (all_doors.contains(&curr) && !collected_keys.contains(&curr.to_lowercase().next().unwrap())) {
        return None;
    }

    if all_keys.contains(&curr) && !collected_keys.contains(&curr) {
        let mut collected_keys = collected_keys.clone();
        collected_keys.insert(curr);
        return Some(vec![((x, y), d, collected_keys)]);
    } else {
        let mut new_states = vec![];

        if came_from != (x-1, y) {
            if let Some(s) = advance(map, collected_keys, all_keys, all_doors, x-1, y, d+1, (x, y)) {
                new_states.extend(s);
            }
        }

        if came_from != (x+1, y) {
            if let Some(s) = advance(map, collected_keys, all_keys, all_doors, x+1, y, d+1, (x, y)) {
                new_states.extend(s);
            }
        }

        if came_from != (x, y-1) {
            if let Some(s) = advance(map, collected_keys, all_keys, all_doors, x, y-1, d+1, (x, y)) {
                new_states.extend(s);
            }
        }

        if came_from != (x, y+1) {
            if let Some(s) = advance(map, collected_keys, all_keys, all_doors, x, y+1, d+1, (x, y)) {
                new_states.extend(s);
            }
        }

        return Some(new_states)
    }
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
struct QueueItem(Vec<(usize, usize)>, usize, BTreeSet<char>, (BTreeSet<char>, usize));

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_sumd: usize = self.1;
        let other_sumd: usize = other.1;
        self_sumd.cmp(&other_sumd)
    }
}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn run2(map: &Vec<Vec<char>>, start: (usize, usize), keys: &Vec<(char, usize, usize)>) {
    let all_keys = keys.iter()
        .filter(|(c, _, _)| c.is_lowercase())
        .map(|(c, _, _)| *c)
        .collect::<Vec<_>>();
    let all_doors = keys.iter()
        .filter(|(c, _, _)| c.is_uppercase())
        .map(|(c, _, _)| *c)
        .collect::<Vec<_>>();
    let mut map = map.clone();

    map[start.1][start.0] = '#';
    map[start.1-1][start.0] = '#';
    map[start.1+1][start.0] = '#';
    map[start.1][start.0-1] = '#';
    map[start.1][start.0+1] = '#';

    let mut visited_states = HashMap::new();
    let mut queue = VecDeque::new();

    let start_coords = vec![
        (start.0 - 1, start.1 - 1),
        (start.0 + 1, start.1 - 1),
        (start.0 - 1, start.1 + 1),
        (start.0 + 1, start.1 + 1),
    ];
    queue.push_back(QueueItem(start_coords, 0, BTreeSet::new(), (BTreeSet::new(), 0)));

    while let Some(QueueItem(coords, d, collected_keys, prev)) = queue.pop_front() {
        if let Some(min_d) = visited_states.get(&(collected_keys.clone(), coords.clone())) {
            if *min_d <= d {
                continue;
            }
        }
        visited_states.insert((collected_keys.clone(), coords.clone()), d);

        if collected_keys.len() >= all_keys.len() {
            println!("Part 2: {}", d);
            return;
        }

        for q in 0..4 {
            let (x, y) = coords[q];

            if let Some(new_states) = advance(&map, &collected_keys, &all_keys, &all_doors, x, y, d, (x, y)) {
                new_states.into_iter()
                    .map(|((x, y), dd, coll)| {
                        let mut coords = coords.clone();
                        coords[q] = (x, y);
                        QueueItem(coords, dd, coll, (collected_keys.clone(), d))
                    })
                    .for_each(|qi| {
                        if let Some(prev_d) = visited_states.get(&(qi.2.clone(), qi.0.clone())) {
                            if qi.1 < *prev_d {
                                queue.push_back(qi);
                            }
                        } else {
                            queue.push_back(qi);
                        }
                    });
            }
        }
        
        let mut qv = queue.into_iter().collect::<Vec<_>>();
        qv.sort_by_key(|qi| qi.1);
        queue = VecDeque::from(qv);
    }
}

fn fill_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map.clone();

    let mut any_found = true;
    while any_found {
        any_found = false;
        for (y, x) in (1..map.len()-1).cartesian_product(1..map[0].len()-1) {
            if map[y][x] != '.' {
                continue;
            }
            let mut walls = 0;
            if map[y-1][x] == '#' {
                walls += 1;
            }
            if map[y+1][x] == '#' {
                walls += 1;
            }
            if map[y][x-1] == '#' {
                walls += 1;
            }
            if map[y][x+1] == '#' {
                walls += 1;
            }
            if walls >= 3 {
                map[y][x] = '#';
                any_found = true;
            }
        }
    }

    for l in &map {
        println!("{}", l.iter().collect::<String>());
    }

    map
}

fn main() -> io::Result<()> {
    let input_lines = aoc2019::read_stdin_lines()?;
    let map = input_lines.into_iter().map(
        |l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let start = map.iter().enumerate().filter_map(|(y, l)| l.iter().position(|c| *c == '@').map(|x| (x,y))).next().expect("No start point found");
    let keys = map.iter().enumerate()
        .flat_map(|(y, l)| l.iter().cloned().enumerate()
            .filter(|(_, c)| *c != '.' && *c != '#' && *c != '@')
            .map(move |(x, c)| (c, x, y)))
        .collect::<Vec<_>>();

    let map = fill_map(&map);

    run1(&map, start, &keys);
    run2(&map, start, &keys);

    Ok(())
}