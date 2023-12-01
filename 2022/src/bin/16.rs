use bit_set::BitSet;
use itertools::Itertools;
use ndarray::Array2;
use std::fmt::Debug;
use std::iter::repeat;
use std::{collections::HashMap, io};

const INPUT_RE: &str = r"Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]+)";

type DistanceMatrix = Array2<usize>;

#[derive(Debug)]
struct Node {
    id: u8,
    flow_rate: usize,
    connected_nodes: HashMap<u8, usize>,
}

fn get_flow_rate(nodes: &HashMap<u8, Node>, opened_valves: &BitSet) -> usize {
    opened_valves
        .iter()
        .map(|id| nodes[&(id as u8)].flow_rate)
        .sum()
}

fn reward(
    minutes_remaining: usize,
    pos: u8,
    opened_valves: BitSet,
    remaining_valves: BitSet,
    nodes: &HashMap<u8, Node>,
    dmtx: &DistanceMatrix,
) -> usize {
    let current_flow_rate = get_flow_rate(nodes, &opened_valves);

    let rem_valve_distances = remaining_valves
        .into_iter()
        .map(|n| (n, dmtx[(pos.into(), n)]));

    let mut max_reward = minutes_remaining * current_flow_rate;
    for (v, d) in rem_valve_distances {
        if d == usize::MAX || (d + 1) >= minutes_remaining {
            continue;
        }
        let mut new_opened_valves = opened_valves.clone();
        new_opened_valves.insert(v);
        let mut new_remaining_valves = remaining_valves.clone();
        assert!(new_remaining_valves.remove(v));
        let sub_reward = reward(
            minutes_remaining - d - 1,
            v as u8,
            new_opened_valves,
            new_remaining_valves,
            nodes,
            dmtx,
        );
        max_reward = max_reward.max(sub_reward + (d + 1) * current_flow_rate);
    }

    max_reward
}

fn reward_with_elephant(
    minutes_remaining: usize,
    pos: u8,
    start_pos: u8,
    opened_valves: BitSet,
    remaining_valves: BitSet,
    nodes: &HashMap<u8, Node>,
    dmtx: &DistanceMatrix,
) -> usize {
    let current_flow_rate = get_flow_rate(nodes, &opened_valves);

    let rem_valve_distances = remaining_valves
        .into_iter()
        .map(|n| (n, dmtx[(pos.into(), n)]));

    let mut max_reward = minutes_remaining * current_flow_rate
        + reward(
            26,
            start_pos,
            BitSet::new(),
            remaining_valves.clone(),
            nodes,
            dmtx,
        );
    for (v, d) in rem_valve_distances {
        if d == usize::MAX || (d + 1) >= minutes_remaining {
            continue;
        }
        let mut new_opened_valves = opened_valves.clone();
        new_opened_valves.insert(v);
        let mut new_remaining_valves = remaining_valves.clone();
        new_remaining_valves.remove(v);
        let sub_reward = reward_with_elephant(
            minutes_remaining - d - 1,
            v as u8,
            start_pos,
            new_opened_valves,
            new_remaining_valves,
            nodes,
            dmtx,
        );
        max_reward = max_reward.max(sub_reward + (d + 1) * current_flow_rate);
    }

    max_reward
}

fn part1(nodes: &HashMap<u8, Node>, dmtx: &DistanceMatrix, start_node: u8) -> usize {
    let valves = nodes
        .values()
        .filter(|n| n.flow_rate > 0)
        .map(|n| n.id.into())
        .collect();
    reward(30, start_node, BitSet::new(), valves, nodes, dmtx)
}

fn part2(nodes: &HashMap<u8, Node>, dmtx: &DistanceMatrix, start_node: u8) -> usize {
    let valves = nodes
        .values()
        .filter(|n| n.flow_rate > 0)
        .map(|n| n.id.into())
        .collect();
    reward_with_elephant(
        26,
        start_node,
        start_node,
        BitSet::new(),
        valves,
        nodes,
        dmtx,
    )
}

fn main() -> io::Result<()> {
    let (nodes, dmtx, start) = parse_input(aoclib::read_input_lines()?);

    let part1 = part1(&nodes, &dmtx, start);
    println!("Part 1: {}", part1);

    let part2 = part2(&nodes, &dmtx, start);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_input(input: Vec<String>) -> (HashMap<u8, Node>, DistanceMatrix, u8) {
    let re = regex::Regex::new(INPUT_RE).unwrap();

    let nodes = input
        .into_iter()
        .enumerate()
        .map(|(i, l)| {
            let matches = re.captures(&l).expect("No regex matches");
            let str_id = matches[1].to_string();
            let flow_rate = matches[2].parse().expect("Failed to parse flow rate");
            let paths: Vec<_> = matches[3].split(", ").map(|s| (s.to_string(), 1)).collect();

            (
                str_id,
                paths,
                Node {
                    id: i as u8,
                    flow_rate,
                    connected_nodes: HashMap::new(),
                },
            )
        })
        .collect_vec();
    let orig_node_count = nodes.len();

    let str_to_num_id = nodes
        .iter()
        .map(|(sid, _, n)| (sid.clone(), n.id))
        .collect::<HashMap<_, _>>();

    let mut nodes: HashMap<_, _> = nodes
        .into_iter()
        .map(|(_, paths, mut n)| {
            n.connected_nodes.extend(
                paths
                    .into_iter()
                    .map(|(strid, d)| (str_to_num_id[&strid], d)),
            );
            (n.id, n)
        })
        .collect();

    // compact
    loop {
        let keys: Vec<_> = nodes.keys().copied().collect();
        let mut any_compacted = false;
        for id in keys {
            let n = nodes.get_mut(&id).unwrap();
            if n.flow_rate == 0 && n.connected_nodes.len() == 2 {
                let ((lid, lcost), (rid, rcost)) =
                    n.connected_nodes.drain().collect_tuple().unwrap();
                let l = nodes.get_mut(&lid).unwrap();
                l.connected_nodes.insert(rid, rcost + lcost);
                l.connected_nodes.remove(&id).unwrap();
                let r = nodes.get_mut(&rid).unwrap();
                r.connected_nodes.insert(lid, rcost + lcost);
                r.connected_nodes.remove(&id).unwrap();
                any_compacted = true;
            }
        }
        if !any_compacted {
            break;
        }
    }

    nodes.retain(|_, n| !n.connected_nodes.is_empty());

    for n in nodes.values() {
        println!("{} -> {{ {} }}", n.id, n.connected_nodes.keys().join(" "));
    }

    // distance matrix
    let mut dmtx: Array2<usize> = Array2::from_elem((orig_node_count, orig_node_count), usize::MAX);
    for ((&to, &len), from) in nodes
        .values()
        .flat_map(|v| v.connected_nodes.iter().zip(repeat(v.id)))
    {
        dmtx[(from.into(), to.into())] = len;
        dmtx[(to.into(), from.into())] = len;
    }
    for &n in nodes.keys() {
        dmtx[(n.into(), n.into())] = 0;
    }
    for i in nodes.keys().map(|i| *i as usize) {
        for j in nodes.keys().map(|j| *j as usize) {
            for k in nodes.keys().map(|k| *k as usize) {
                let sum = dmtx[(i, k)].saturating_add(dmtx[(k, j)]);
                if (sum) < (dmtx[(i, j)]) {
                    dmtx[(i, j)] = sum;
                    dmtx[(j, i)] = sum;
                }
            }
        }
    }

    (nodes, dmtx, str_to_num_id["AA"])
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_real_input() {
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(16)).unwrap();

        let (nodes, dmtx, start) = parse_input(input);

        let p1 = part1(&nodes, &dmtx, start);
        assert_eq!(p1, 1595);

        let p2 = part2(&nodes, &dmtx, start);
        assert_eq!(p2, 2189);
    }
}
