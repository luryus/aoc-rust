use std::{io, str::FromStr};

#[derive(Copy, Clone)]
enum Instruction {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

#[derive(Debug)]
enum ParseInstructionError {
    InvalidFormat(&'static str),
    UnknownOp(String)
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ins, arg) = aoc2020::split_to_tuple2(s, " ")
            .ok_or(ParseInstructionError::InvalidFormat("Split failed"))?;
        let arg = arg.parse::<i64>()
            .map_err(|_| ParseInstructionError::InvalidFormat("Argument parse failed"))?;

        match ins {
            "acc" => Ok(Instruction::Acc(arg)),
            "jmp" => Ok(Instruction::Jmp(arg)),
            "nop" => Ok(Instruction::Nop(arg)),
            unknown=> Err(ParseInstructionError::UnknownOp(unknown.to_string())),
        }
    }
}

#[derive(Clone)]
struct Interpreter {
    program: Vec<Instruction>,
    acc: i64,
    ip: usize,
}

impl Interpreter {
    fn new(program: Vec<Instruction>) -> Interpreter {
        Interpreter {
            program: program,
            acc: 0,
            ip: 0
        }
    }

    fn step(&mut self) {
        if self.halted() {
            return;
        }
        let ins = &self.program[self.ip];
        self.ip = match &ins {
            Instruction::Acc(arg) => {
                self.acc += arg;
                self.ip + 1
            }
            Instruction::Jmp(arg) => (self.ip as i64 + arg) as usize,
            Instruction::Nop(_) => self.ip + 1,
        };
    }

    fn halted(&self) -> bool {
        self.ip >= self.program.len()
    }
}

fn part1(mut input: Interpreter) -> i64 {
    let mut visited = vec![false; input.program.len()];
    loop {
        visited[input.ip] = true;
        input.step();
        if visited[input.ip] {
            return input.acc;
        }
    }
}

fn part2(input: Interpreter) -> i64 {
    const MAX_CYCLES: usize = 1_000;

    input
        .program
        .iter()
        .enumerate()
        .filter(|(_, &ins)| matches!(ins, Instruction::Jmp(_) | Instruction::Nop(_)))
        .map(|(i, ins)| {
            let mut inter = input.clone();
            inter.program[i] = match ins {
                Instruction::Jmp(a) => Instruction::Nop(*a),
                Instruction::Nop(a) => Instruction::Jmp(*a),
                _ => unreachable!(),
            };
            inter
        })
        .filter_map(|mut inter| {
            for _ in 0..MAX_CYCLES {
                inter.step();
                if inter.halted() {
                    return Some(inter.acc);
                }
            }

            return None;
        })
        .next()
        .unwrap()
}

fn main() -> io::Result<()> {
    let input = aoc2020::read_stdin_lines()?;
    let program = input
        .into_iter()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();
    let interpreter = Interpreter::new(program);

    let p1 = part1(interpreter.clone());
    println!("Part 1: {}", p1);

    let p2 = part2(interpreter);
    println!("Part 2: {}", p2);

    Ok(())
}
