use std::io;
use std::collections::HashMap;

struct Node<'a> {
    parent_id: Option<&'a str>,
    children: Vec<&'a str>,
}

fn orbit_number(n: &Node, orbits: &HashMap<&str, Node>, h: u32) -> u32 {
    h + n.children.iter()
        .filter_map(|id| orbits.get(id).map(|n| orbit_number(n, orbits, h + 1)))
        .sum::<u32>()
}

fn run1(orbits: &HashMap<&str, Node>) -> u32 {
    orbits.values().filter(|n| n.parent_id == None)
        .map(|n| orbit_number(n, orbits, 0))
        .sum()
}

fn run2(orbits: &HashMap<&str, Node>) -> u32 {
    let start = orbits.get("YOU").unwrap().parent_id.unwrap();
    let end = orbits.get("SAN").unwrap().parent_id.unwrap();

    // LCA
    let start_path = 
        std::iter::successors(Some(start), |id| orbits.get(id).and_then(|n| n.parent_id))
            .collect::<Vec<_>>();

    let end_path = std::iter::successors(
        Some(end), |id| orbits.get(id).and_then(|n| n.parent_id));
    
    let (end_path_pos, lca) = end_path
        .enumerate()
        .find(|(_, item)| start_path.contains(item))
        .unwrap();
    (end_path_pos + start_path.iter().position(|x| *x == lca).unwrap()) as u32
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_lines()?;

    let mut orbits: HashMap<&str, Node> = HashMap::new();

    input.iter()
        .map(|s| {
            let mut parts = s.split(')');
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .for_each(|(parent, child)| {
            let p = orbits.entry(parent).or_insert(Node {
                parent_id: None,
                children: Vec::new(),
            });
            p.children.push(child);
            let c = orbits.entry(child).or_insert(Node {
                parent_id: None,
                children: Vec::new(),
            });
            c.parent_id = Some(parent);
        });

    println!("Part 1: {}", run1(&orbits));
    println!("Part 2: {}", run2(&orbits));
    Ok(())
}