use itertools::Itertools;
use std::collections::HashMap;
use std::io;

#[derive(Clone)]
enum Gate {
    And(String, String),
    Or(String, String),
    Xor(String, String),
    Lit(bool),
}

fn eval(name: &str, gates: &mut HashMap<String, Gate>) -> bool {
    let g = gates.get(name).cloned().unwrap();
    let res = match &g {
        Gate::Lit(b) => *b,
        Gate::And(a, b) => eval(a, gates) && eval(b, gates),
        Gate::Or(a, b) => eval(a, gates) || eval(b, gates),
        Gate::Xor(a, b) => eval(a, gates) ^ eval(b, gates),
    };
    if !matches!(g, Gate::Lit(_)) {
        let g = gates.get_mut(name).unwrap();
        *g = Gate::Lit(res);
    }
    res
}

fn part1(input: &HashMap<String, Gate>) -> usize {
    let mut m = input.clone();
    let targets = input.keys().filter(|k| k.starts_with('z')).sorted().rev();

    targets
        .map(|t| eval(t, &mut m))
        .fold(0, |acc, b| (acc << 1) | b as usize)
}

fn part2(input: &HashMap<String, Gate>) -> String {
    // I solved part 2 by just looking at the full adder graph
    // To avoid boringness, "test" the results here with some sample values

    let mut input = input.clone();
    let swaps = [
        ("z39", "fnr"),
        ("z23", "cgq"),
        ("z15", "kqk"),
        ("nbc", "svm"),
    ];

    for (a, b) in swaps {
        let aa = input.get(a).unwrap().clone();
        let bb = input.get(b).unwrap().clone();

        *input.get_mut(b).unwrap() = aa;
        *input.get_mut(a).unwrap() = bb;
    }

    // Inputs are 45-bit
    let x = 0b100111010101011110110010101000000110100011101_usize;
    let y = 0b111001101001011001101100010111100011000001010_usize;

    for i in 0..45 {
        let xname = format!("x{i:02}");
        let yname = format!("y{i:02}");

        let xbit = ((x >> i) & 0x1) != 0;
        let ybit = ((y >> i) & 0x1) != 0;

        input.insert(xname, Gate::Lit(xbit));
        input.insert(yname, Gate::Lit(ybit));
    }

    let z = part1(&input);
    assert_eq!(x + y, z);

    swaps
        .into_iter()
        .flat_map(|(a, b)| [a, b])
        .sorted()
        .join(",")
}

fn parse_input(input: Vec<String>) -> HashMap<String, Gate> {
    let lits = input.iter().take_while(|l| !l.is_empty()).map(|l| {
        let (name, val) = aoclib::split_to_tuple2(l, ": ").unwrap();
        let val = match val {
            "1" => true,
            "0" => false,
            _ => unreachable!(),
        };
        let name = name.to_owned();
        (name, Gate::Lit(val))
    });
    let gates = input.iter().skip_while(|l| !l.is_empty()).skip(1).map(|l| {
        if let [op1, gat, op2, _, res, ..] = &l.split(" ").collect_vec().as_slice() {
            let op1 = (*op1).to_owned();
            let op2 = (*op2).to_owned();
            let gate = match *gat {
                "AND" => Gate::And(op1, op2),
                "OR" => Gate::Or(op1, op2),
                "XOR" => Gate::Xor(op1, op2),
                _ => unreachable!(),
            };

            ((*res).to_owned(), gate)
        } else {
            panic!()
        }
    });

    lits.chain(gates).collect()
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let input = parse_input(input);

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
        let input = aoclib::read_file_lines(aoclib::get_test_input_file!(24)).unwrap();
        let input = parse_input(input);

        let p1 = part1(&input);
        assert_eq!(p1, 57344080719736);

        let p2 = part2(&input);
        assert_eq!(p2, "cgq,fnr,kqk,nbc,svm,z15,z23,z39");
    }
}
