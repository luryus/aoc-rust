use std::collections::HashMap;
use std::io;


#[derive(PartialEq, Hash, Eq, Clone)]
enum Operand<'a> {
    Imm(u16),
    Wire(&'a str),
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum Instruction<'a> {
    Mov(Operand<'a>),
    And(Operand<'a>, Operand<'a>),
    Or(Operand<'a>, Operand<'a>),
    Not(Operand<'a>),
    Lshift(Operand<'a>, u16),
    Rshift(Operand<'a>, u16),
}

struct Definition<'a> {
    op: Instruction<'a>,
    dst: &'a str,
}

fn parse_operand(s: &str) -> Operand {
    match s.parse() {
        Ok(val) => Operand::Imm(val),
        _ => Operand::Wire(s)
    }
}

fn parse_input_line<'a>(line: &'a str) -> Definition<'a> {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    if parts.len() == 3 {
        return Definition {
            op: Instruction::Mov(parse_operand(parts[0])),
            dst: parts[2],
        }
    } else if parts.len() == 4 {
        return Definition {
            op: Instruction::Not(parse_operand(parts[1])),
            dst: parts[3],
        }
    } else {
        let op = match parts[1] {
            "AND" => Instruction::And(parse_operand(parts[0]), parse_operand(parts[2])),
            "OR" => Instruction::Or(parse_operand(parts[0]), parse_operand(parts[2])),
            "LSHIFT" => Instruction::Lshift(parse_operand(parts[0]), parts[2].parse().unwrap()),
            "RSHIFT" => Instruction::Rshift(parse_operand(parts[0]), parts[2].parse().unwrap()),
            &_ => panic!("Illegal operation"),
        };
        return Definition {
            op: op,
            dst: parts[4],
        }
    }
}

fn eval<'a>(op: &Operand<'a>, wires: &HashMap<&str, &'a Instruction<'a>>, cache: &mut HashMap<&'a Instruction<'a>, u16>) -> Option<u16> {

    let instr = match *op {
        Operand::Imm(val) => return Some(val),
        Operand::Wire(w) => wires.get(w)
    }?;

    if let Some(cached) = cache.get(*instr) {
        return Some(*cached)
    }

    let ret = match *instr {
        Instruction::Mov(w) => eval(w, wires, cache),
        Instruction::And(l, r) => 
            eval(l, wires, cache).and_then(|lv| eval(r, wires, cache).map(|rv| lv & rv)),
        Instruction::Or(l, r) => 
            eval(l, wires, cache).and_then(|lv| eval(r, wires, cache).map(|rv| lv | rv)),
        Instruction::Not(w) => eval(w, wires, cache).map(|v| !v),
        Instruction::Lshift(w, shamt) => eval(w, wires, cache).map(|v| v << shamt),
        Instruction::Rshift(w, shamt) => eval(w, wires, cache).map(|v| v >> shamt),
    };

    if let Some(val) = ret {
        cache.insert(*instr, val);
    }

    ret
}

fn run1(defs: &Vec<Definition>) -> Option<u16> {
    let mut wires: HashMap<&str, &Instruction> = HashMap::new();
    let mut cache: HashMap<&Instruction, u16> = HashMap::new();

    for def in defs {
        wires.insert(def.dst, &def.op);
    }

    let res = eval(&Operand::Wire("a"), &wires, &mut cache);
    println!("Part 1: {:?}", res);
    res
}

fn run2(defs: &Vec<Definition>, res1: u16) {
    let mut wires: HashMap<&str, &Instruction> = HashMap::new();
    let mut cache: HashMap<&Instruction, u16> = HashMap::new();

    for def in defs {
        wires.insert(def.dst, &def.op);
    }
    let i = Instruction::Mov(Operand::Imm(res1));
    wires.insert("b", &i);

    let res = eval(&Operand::Wire("a"), &wires, &mut cache);
    println!("Part 1: {:?}", res);
}

fn main() -> io::Result<()> {
    let input = aoc2015::read_stdin_lines()?;
    let defs = input.iter()
        .map(|l| parse_input_line(l))
        .collect::<Vec<_>>();

    let res1 = run1(&defs).unwrap();
    
    run2(&defs, res1);

    Ok(())
}