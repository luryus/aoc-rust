use std::{io, collections::{HashMap, VecDeque}};
use itertools::Itertools;
use num_integer::Integer;

trait ModuleInput<'a> {
    fn pulse<'b>(&'b mut self, pulse: Pulse<'a>) -> Vec<Pulse<'a>> where 'a : 'b;
}

impl<'a> ModuleInput<'a> for ConjunctionModule<'a> {
    fn pulse<'b>(&'b mut self, pulse: Pulse<'a>) -> Vec<Pulse<'a>> where 'a : 'b {
        self.inputs.insert(pulse.0, pulse.2);

        let out_pulse = !self.inputs.values().all(|i| *i);
        self.output_ids.iter().map(|o| Pulse(pulse.1, o, out_pulse)).collect()
    }
}

struct ConjunctionModule<'a> {
    inputs: HashMap<&'a str, bool>,
    output_ids: Vec<&'a str>,
}

struct FlipFlopModule<'a> {
    state: bool,
    output_ids: Vec<&'a str>
}
impl<'a> ModuleInput<'a> for FlipFlopModule<'a> {
    fn pulse<'b>(&'b mut self, pulse: Pulse<'a>) -> Vec<Pulse<'a>> where 'a : 'b {
        if pulse.2 {
            return vec![];
        }

        self.state = !self.state;
        self.output_ids.iter().map(|o| Pulse(pulse.1, o, self.state)).collect()
    }
}

struct BroadcastModule<'a> {
    output_ids: Vec<&'a str>
}
impl<'a> ModuleInput<'a> for BroadcastModule<'a> {
    fn pulse<'b>(&'b mut self, pulse: Pulse<'a>) -> Vec<Pulse<'a>> where 'a : 'b {
        self.output_ids.iter().map(|o| Pulse(pulse.1, o, pulse.2)).collect()
    }
}


struct Modules<'a> {
    broadcast: BroadcastModule<'a>,

    conjunctions: HashMap<&'a str, ConjunctionModule<'a>>,
    flipflops: HashMap<&'a str, FlipFlopModule<'a>>,
}

#[derive(Clone, Copy, Debug)]
struct Pulse<'a>(&'a str, &'a str, bool);

impl<'b> Modules<'b> {
    fn send_pulse<'a>(&'a mut self, pulse: Pulse<'b>) -> Vec<Pulse<'b>> {
        if pulse.1 == "broadcaster" {
            self.broadcast.pulse(pulse)
        } else if let Some(c) = self.conjunctions.get_mut(pulse.1) {
            c.pulse(pulse)
        } else if let Some(f) = self.flipflops.get_mut(pulse.1) {
            f.pulse(pulse)
        } else {
            vec![]
        }
    }
}

fn parse(input: &Vec<String>) -> Modules {
    let mut broadcast = None;
    let mut conjunctions = HashMap::new();
    let mut flipflops = HashMap::new();

    for l in input {
        let (l, r) = aoclib::split_to_tuple2(l, " -> ").unwrap();
        let targets: Vec<_> = r.split(", ").collect();
        if l == "broadcaster" {
            broadcast = Some(BroadcastModule { output_ids: targets })
        } else if let Some(id) = l.strip_prefix('%') {
            let ff = FlipFlopModule { output_ids: targets, state: false };
            flipflops.insert(id, ff);
        } else if let Some(id) = l.strip_prefix('&') {
            let conj = ConjunctionModule { inputs: HashMap::default(), output_ids: targets };
            conjunctions.insert(id, conj);
        }
    }
    let broadcast = broadcast.unwrap();

    let bcast_outputs = broadcast.output_ids.iter().copied().map(|o| ("broadcaster", o));
    let ff_outputs = flipflops.iter().flat_map(|(id, f)| f.output_ids.iter().map(|o| (*id, *o)));
    let conj_outputs = conjunctions.iter().flat_map(|(id, f)| f.output_ids.iter().map(|o| (*id, *o)));
    let all_output_ids: Vec<_> = bcast_outputs.chain(ff_outputs).chain(conj_outputs).collect();

    for (in_id, out_id) in all_output_ids {
        if let Some(c) = conjunctions.get_mut(&out_id) {
            c.inputs.insert(in_id, false);
        }
    }

    Modules {
        broadcast, conjunctions, flipflops
    }
}

fn part1(input: &Vec<String>) -> usize {
    let mut modules = parse(input);
    let mut q = VecDeque::new();
    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 0..1000 {
        let pulse = Pulse("", "broadcaster", false);
        q.push_back(pulse);

        while let Some(p) = q.pop_front() {
            if p.2 {
                high_count += 1;
            } else {
                low_count += 1;
            }
            let pulses = modules.send_pulse(p);
            q.extend(pulses);
        }
    }

    low_count * high_count
}


fn part2(input: &Vec<String>) -> usize {
    let mut modules = parse(input);
    let mut q = VecDeque::new();

    // Hand-crafted based on the input
    let (&rx_module_id, rx_module) = modules.conjunctions.iter().find(|(_, m)| m.output_ids.contains(&"rx")).unwrap();
    assert_eq!(1, rx_module.output_ids.len());

    let interesting_ids: Vec<_> = rx_module.inputs.keys().copied().sorted().collect();
    assert_eq!(interesting_ids.len(), 4);

    // For rx to get a low pulse, all of cn inputs must be high at the same time
    
    // The inputs all have cycles: they go high every (x*n - 1) button presses
    // (here x is input-specific constant and n is number of cycles)

    // Run the loop until we find three values for each input
    let mut high_vals: HashMap<_, Vec<usize>> = interesting_ids.into_iter().map(|id| (id, Vec::with_capacity(3))).collect();

    for b in 0.. {
        let pulse = Pulse("", "broadcaster", false);
        q.push_back(pulse);

        while let Some(p) = q.pop_front() {
            if p.2 && p.1 == rx_module_id {
                if let Some(bs) = high_vals.get_mut(&p.0) {
                    bs.push(b);
                }
            }
            let pulses = modules.send_pulse(p);
            q.extend(pulses);
        }

        if high_vals.values().all(|x| x.len() >= 3) {
            break;
        }
    }

    // Find the diff for each module
    let diffs: Vec<_> = high_vals.values().map(|bs| {
        let (a, b, c) = &bs[..3].iter().copied().collect_tuple().unwrap();
        assert_eq!(c - b, b - a);
        c - b
    }).collect();

    // Now we know that the inputs 1-4 go high with these patterns
    // d1 * n1 - 1
    // d2 * n2 - 1
    // d3 * n3 - 1
    // d4 * n4 - 1

    // The first button press that has all enabled can be computed with lcm
    diffs.into_iter().reduce(|a, b| a.lcm(&b)).unwrap()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;

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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(20)).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, 818723272);

        let p2 = part2(&input);
        assert_eq!(p2, 243902373381257);
    }
}
