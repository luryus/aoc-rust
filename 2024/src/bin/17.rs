use itertools::Itertools;
use std::io;

#[derive(Clone, Copy, Debug)]
enum Operand {
    Literal(u8),
    A,
    B,
    C,
    Reserved,
}

impl Operand {
    const fn eval(&self, prog: &mut Program) -> usize {
        match self {
            Operand::Literal(l) => (*l) as usize,
            Operand::A => prog.a,
            Operand::B => prog.b,
            Operand::C => prog.c,
            Operand::Reserved => todo!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instr {
    Adv(Operand),
    Bxl(Operand),
    Bst(Operand),
    Jnz(Operand),
    Bxc,
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand),
}

#[derive(Debug)]
struct Program {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,

    program: Vec<Instr>,
}

impl Program {
    fn run(&mut self) -> Vec<u8> {
        use Operand::*;
        let mut output = vec![];
        while let Some(&i) = self.program.get(self.ip) {
            match i {
                Instr::Adv(operand) => {
                    division(self, A, operand);
                    self.ip += 1;
                }
                Instr::Bxl(operand) => {
                    self.b ^= operand.eval(self);
                    self.ip += 1;
                }
                Instr::Bst(operand) => {
                    self.b = operand.eval(self) % 8;
                    self.ip += 1;
                }
                Instr::Jnz(operand) => {
                    if self.a == 0 {
                        self.ip += 1;
                    } else {
                        self.ip = operand.eval(self) / 2;
                    }
                }
                Instr::Bxc => {
                    self.b ^= self.c;
                    self.ip += 1;
                }
                Instr::Out(operand) => {
                    let val: u8 = (operand.eval(self) % 8).try_into().unwrap();
                    output.push(val);
                    self.ip += 1;
                }
                Instr::Bdv(operand) => {
                    division(self, B, operand);
                    self.ip += 1;
                }
                Instr::Cdv(operand) => {
                    division(self, C, operand);
                    self.ip += 1;
                }
            }
        }

        output
    }
}

fn combo_operand(val: u8) -> Operand {
    match val {
        0..=3 => Operand::Literal(val),
        4 => Operand::A,
        5 => Operand::B,
        6 => Operand::C,
        7 => Operand::Reserved,
        _ => unreachable!(),
    }
}

fn division(program: &mut Program, target: Operand, operand: Operand) {
    let denominator = 2usize.pow(operand.eval(program).try_into().unwrap());
    let res = program.a / denominator;

    match target {
        Operand::A => program.a = res,
        Operand::B => program.b = res,
        Operand::C => program.c = res,
        _ => unreachable!(),
    };
}

fn parse_input(input: &[usize]) -> Program {
    use Instr::*;

    let a = input[0];
    let b = input[1];
    let c = input[2];

    let program = input
        .iter()
        .skip(3)
        .tuples()
        .map(|(opcode, &operand)| match opcode {
            0 => Adv(combo_operand(operand.try_into().unwrap())),
            1 => Bxl(Operand::Literal(operand.try_into().unwrap())),
            2 => Bst(combo_operand(operand.try_into().unwrap())),
            3 => Jnz(Operand::Literal(operand.try_into().unwrap())),
            4 => Bxc,
            5 => Out(combo_operand(operand.try_into().unwrap())),
            6 => Bdv(combo_operand(operand.try_into().unwrap())),
            7 => Cdv(combo_operand(operand.try_into().unwrap())),
            _ => unreachable!(),
        })
        .collect();

    Program {
        a,
        b,
        c,
        ip: 0,
        program,
    }
}

fn part1(input: &[usize]) -> String {
    let mut prog = parse_input(input);
    let output = prog.run();

    output.into_iter().map(|n| format!("{n}")).join(",")
}

fn part2(input: &[usize]) -> usize {
    // As always, part 2 of the interpreter days need manual
    // work based on the given input.
    let mut prog = parse_input(input);
    let expected: Vec<u8> = input
        .iter()
        .copied()
        .skip(3)
        .map(|x| x.try_into().unwrap())
        .collect();

    let mut rev_expected = expected.clone();
    rev_expected.reverse();
    let res = build(0, &rev_expected).unwrap();

    prog.a = res;
    assert_eq!(prog.run(), expected);

    res
}

fn build(a: usize, expected: &[u8]) -> Option<usize> {
    // Build a number, reversing the program in the input
    let ex = expected[0];
    for i in 0..8u8 {
        let aa = a << 3 | i as usize;
        let mut b = i ^ 1;
        let c: u8 = ((aa / 2usize.pow(b.into())) % 8) as u8;
        b = b ^ 5 ^ c;
        if b == ex {
            if expected.len() == 1 {
                return Some(aa);
            } 
    
            if let Some(aaa) = build(aa, &expected[1..]) {
                return Some(aaa);
            }
        }
    }

    None
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_ints(false)?;

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
        let input = aoclib::read_ints_from_file(aoclib::get_test_input_file!(17), false).unwrap();

        let p1 = part1(&input);
        assert_eq!(p1, "6,4,6,0,4,5,7,2,7");

        let p2 = part2(&input);
        assert_eq!(p2, 164541160582845);
    }
}