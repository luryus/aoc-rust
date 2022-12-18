use bit_set::BitSet;
use itertools::Itertools;
use lru::LruCache;
use ndarray::Array2;
use regex;
use std::collections::{BTreeSet, VecDeque};
use std::fmt::Debug;
use std::iter::repeat;
use std::{collections::HashMap, io};

const INPUT_RE: &str = r"Valve (\w{2}) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]+)";

#[derive(Debug)]
struct Node {
    id: u8,
    flow_rate: usize,
    connected_nodes: HashMap<u8, usize>,
}

#[derive(Clone)]
struct State {
    pos: u8,
    minute: usize,
    opened_valves: BitSet,
    flow_rate: usize,
    released_pressure: usize,
}
impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: pos {}, flow_rate {}, opened_valves {:?}",
            self.minute, self.pos, self.flow_rate, self.opened_valves
        )
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State2 {
    pos: (Pos, Pos),
    minute: usize,
    opened_valves: BitSet,
    flow_rate: usize,
}
impl Debug for State2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: pos {:?}/{:?}, flow_rate {}, opened_valves {:?}",
            self.minute, self.pos.0, self.pos.1, self.flow_rate, self.opened_valves
        )
    }
}
impl State2 {
    fn positions_valves(&self) -> (BTreeSet<Pos>, BitSet) {
        let mut pos_set = BTreeSet::new();
        pos_set.insert(self.pos.0.clone());
        pos_set.insert(self.pos.1.clone());
        (pos_set, self.opened_valves.clone())
    }
}

fn part1(nodes: &HashMap<u8, Node>, aa_pos: u8) -> usize {
    fn step(
        total_valves: usize,
        nodes: &HashMap<u8, Node>,
        visited_states: &mut Vec<State>,
        current_state: State,
    ) -> usize {
        if current_state.minute == 31 {
            //dbg!(visited_states);
            return current_state.released_pressure;
        }
        visited_states.push(current_state.clone());
        let current_minute = current_state.minute;
        let pos = &nodes[&current_state.pos];

        let r = if total_valves == current_state.opened_valves.len() {
            let mut ns = current_state.clone();
            let d = 31 - current_minute;
            ns.minute += d;
            ns.released_pressure += d * ns.flow_rate;
            step(total_valves, nodes, visited_states, ns)
        } else if pos.flow_rate > 0 && !current_state.opened_valves.contains(pos.id.into()) {
            let mut ns = current_state.clone();
            ns.opened_valves.insert(ns.pos.into());
            ns.released_pressure += ns.flow_rate;
            ns.flow_rate += pos.flow_rate;
            ns.minute = current_minute + 1;

            step(total_valves, nodes, visited_states, ns)
        } else {
            let mut max = 0;
            for (conn_id, d) in &pos.connected_nodes {
                if current_minute + d > 31 {
                    continue;
                }
                //dbg!((&pos.id, conn));
                let mut ns = current_state.clone();
                ns.pos = conn_id.clone();
                ns.minute = current_minute + d;
                ns.released_pressure += ns.flow_rate * d;
                let opened_count = ns.opened_valves.len();

                let seen_before = visited_states
                    .iter()
                    .rev()
                    .take_while(|s| s.opened_valves.len() == opened_count)
                    .any(|s| &s.pos == conn_id);
                if !seen_before {
                    max = max.max(step(total_valves, nodes, visited_states, ns));
                } else {
                    //dbg!(&states);
                }
            }
            max
        };
        visited_states.pop();
        r
    }

    let init_state = State {
        pos: aa_pos,
        minute: 1,
        opened_valves: BitSet::new(),
        released_pressure: 0,
        flow_rate: 0,
    };

    let mut visited_states = Vec::new();
    let total_valves = nodes.values().filter(|n| n.flow_rate > 0).count();
    step(total_valves, nodes, &mut visited_states, init_state)
}

#[derive(Clone, Debug)]
enum Choice {
    Open(u8),
    Move(u8, u8, usize),
}
impl Choice {
    fn runtime(&self) -> usize {
        match self {
            Choice::Open(_) => 1,
            Choice::Move(_, _, d) => *d,
        }
    }
}

fn get_choices(
    current_state: &State2,
    came_from: &Pos,
    pos: &Pos,
    nodes: &HashMap<u8, Node>,
    dmtx: &Array2<usize>,
) -> Vec<Choice> {
    match pos {
        Pos::Moving(f, t, d) => vec![Choice::Move(*f, *t, *d)],
        Pos::Node(nid) => {
            let mut res = vec![];
            let n = &nodes[nid];
            if n.flow_rate > 0 && !current_state.opened_valves.contains(n.id.into()) {
                res.push(Choice::Open(*nid));
                return res;
            }

            for conn_node in nodes.values() {
                if *nid == conn_node.id || current_state.opened_valves.contains(conn_node.id as usize) || conn_node.flow_rate == 0 {
                    continue;
                }
                let d = dmtx[((*nid).into(), conn_node.id.into())];
                if d > 1_000_000 {
                    continue;
                }
                if current_state.minute + d > 26 {
                    continue;
                }

                /*if matches!(&came_from, Pos::Moving(f, _, _)|Pos::Node(f) if *f == conn_node.id) {
                    continue;
                }*/

                res.push(Choice::Move(*nid, conn_node.id, d));
            }

            if res.is_empty() {
                res.push(Choice::Move(*nid, *nid, 1));
            }

            res
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
struct PosOpened((Pos, Pos), BitSet);

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum Pos {
    Node(u8),
    Moving(u8, u8, usize),
}
impl Pos {
    fn target(&self) -> u8 {
        match self {
            Pos::Node(t) => *t,
            Pos::Moving(_, t, _) => *t,
        }
    }
}

fn part22(nodes: &HashMap<u8, Node>, dmtx: &Array2<usize>, aa_index: u8) -> usize {
    fn step(
        cache: &mut LruCache<State2, usize>,
        dmtx: &Array2<usize>,
        total_valves: usize,
        nodes: &HashMap<u8, Node>,
        current_state: State2,
        came_from: &State2,
    ) -> usize {
        let current_minute = current_state.minute;
        let all_open = total_valves == current_state.opened_valves.len();
        if current_minute == 27 {
            return 0;
        }
 
        if current_minute >= 10 && current_state.opened_valves.len() <= 1 {
            return 0;
        }

        if current_minute >= 20 && current_state.opened_valves.len() <= 5 {
            return 0;
        }

        if all_open {
            let d = 27 - current_minute;
            d * current_state.flow_rate
        } else {
            if let Some(v) = cache.get(&current_state) {
                return *v;
            }

            if current_minute <= 5 {
                dbg!(&current_state);
            }

            let (pos1, pos2) = (&current_state.pos.0, &current_state.pos.1);

            let pos1_choices = get_choices(&current_state, &came_from.pos.0, pos1, nodes, dmtx);
            let pos2_choices = get_choices(&current_state, &came_from.pos.1, pos2, nodes, dmtx);

            let mut states: Vec<_> = pos1_choices
                .into_iter()
                .cartesian_product(pos2_choices.into_iter())
                .filter_map(|(p1c, p2c)| get_next_state(nodes, &current_state, p1c, p2c))
                /*.filter(|s| {
                    let po = PosOpened(s.pos.clone(), s.opened_valves.clone());
                    if let Some(bests) = all_visited.get(&po) {
                        !bests
                            .iter()
                            .any(|(bm, brp)| *bm <= s.minute && *brp > s.released_pressure)
                    } else {
                        true
                    }
                })*/
                .unique_by(|s| s.positions_valves())
                .collect();

            let not_same_target_states: Vec<_> = states.iter()
                .filter(|s| s.pos.0.target() != s.pos.1.target())
                .cloned()
                .collect();
            if !not_same_target_states.is_empty() {
                states = not_same_target_states;
            }
            let v = states
                .into_iter()
                .map(|s| {
                    (s.minute - current_minute) * current_state.flow_rate
                        + step(cache, dmtx, total_valves, nodes, s, &current_state)
                })
                .max()
                .unwrap_or(0);
            cache.push(current_state, v);
            v
        }
    }

    let init_state = State2 {
        pos: (Pos::Node(aa_index), Pos::Node(aa_index)),
        minute: 1,
        opened_valves: BitSet::new(),
        flow_rate: 0,
    };
    let total_valves = nodes.values().filter(|n| n.flow_rate > 0).count();
    let mut cache = LruCache::new(2000.try_into().unwrap());
    step(
        &mut cache,
        dmtx,
        total_valves,
        nodes,
        init_state.clone(),
        &init_state,
    )
}

fn get_next_state(
    nodes: &HashMap<u8, Node>,
    current_state: &State2,
    p0c: Choice,
    p1c: Choice,
) -> Option<State2> {
    let mut ns = current_state.clone();
    let dmin = p0c.runtime().min(p1c.runtime());
    assert!(dmin > 0);
    ns.minute += dmin;

    match (p0c, p1c) {
        (Choice::Move(f0, t0, dmin0), Choice::Move(f1, t1, dmin1)) => {
            let new_pos0 = if dmin0 > dmin {
                Pos::Moving(f0, t0, dmin0 - dmin)
            } else {
                Pos::Node(t0)
            };
            let new_pos1 = if dmin1 > dmin {
                Pos::Moving(f1, t1, dmin1 - dmin)
            } else {
                Pos::Node(t1)
            };

            ns.pos = (new_pos0, new_pos1);
            Some(ns)
        }
        (Choice::Move(f0, t0, dmin0), Choice::Open(oid)) => {
            let new_pos0 = if dmin0 > dmin {
                Pos::Moving(f0, t0, dmin0 - dmin)
            } else {
                Pos::Node(t0)
            };
            ns.pos.0 = new_pos0;
            ns.opened_valves.insert(oid.into());
            ns.flow_rate += nodes[&oid].flow_rate;
            Some(ns)
        }
        (Choice::Open(oid), Choice::Move(f1, t1, dmin1)) => {
            let new_pos1 = if dmin1 > dmin {
                Pos::Moving(f1, t1, dmin1 - dmin)
            } else {
                Pos::Node(t1)
            };
            ns.pos.1 = new_pos1;
            ns.opened_valves.insert(oid.into());
            ns.flow_rate += nodes[&oid].flow_rate;
            Some(ns)
        }
        (Choice::Open(oid0), Choice::Open(oid1)) => {
            if oid0 == oid1 {
                None
            } else {
                ns.opened_valves.insert(oid0.into());
                ns.opened_valves.insert(oid1.into());
                ns.flow_rate += nodes[&oid0].flow_rate + nodes[&oid1].flow_rate;
                Some(ns)
            }
        }
    }
}

fn main() -> io::Result<()> {
    let input = aoc2022::read_input_lines()?;
    let (input, dmtx, aa_pos) = parse_input(input);

    let p1 = part1(&input, aa_pos);
    println!("Part 1: {}", p1);

    let p2 = part22(&input, &dmtx, aa_pos);
    println!("Part 2: {}", p2);

    Ok(())
}

fn parse_input(input: Vec<String>) -> (HashMap<u8, Node>, Array2<usize>, u8) {
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
                assert!(sum != 0 || i == j);
                if (sum) < (dmtx[(i, j)]) {
                    dmtx[(i, j)] = sum;
                    dmtx[(j, i)] = sum;
                }
            }
        }
    }

    (nodes, dbg!(dmtx), str_to_num_id["AA"])
}
