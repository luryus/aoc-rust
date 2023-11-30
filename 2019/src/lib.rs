use std::io::{Read, self};
use std::iter::Iterator;
use std::collections::VecDeque;
use std::cmp::Ordering;

pub fn read_stdin_to_string() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.to_owned())
}

pub fn read_stdin_lines() -> io::Result<Vec<String>> {
    let input = read_stdin_to_string()?;
    Ok(input.lines().filter_map(|l| {
        if l.is_empty() {
            None
        } else {
            Some(l.to_owned())
        }
    }).collect())
}

const ADD: i64 = 1;
const MUL: i64 = 2;
const HALT: i64 = 99;
const INPUT: i64 = 3;
const OUTPUT: i64 = 4;
const JUMP_IF_TRUE: i64 = 5;
const JUMP_IF_FALSE: i64 = 6;
const LT : i64 = 7;
const EQ: i64 = 8;
const REL_BASE_OFFSET: i64 = 9;

enum Parameter {
    Imm(i64),
    Pos(i64),
    Rel(i64),
}

enum Operation {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    Halt,
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equal(Parameter, Parameter, Parameter),
    RelBaseOffset(Parameter),
}

#[derive(Clone)]
pub struct IntCodeComputer {
    mem: Vec<i64>,
    ip: usize,
    pub inputs: VecDeque<i64>,
    pub outputs: VecDeque<i64>,
    rel_base: i64
}

impl IntCodeComputer {
    pub fn new(mem: &Vec<i64>) -> IntCodeComputer {
        IntCodeComputer::new_with_input(mem, vec![])
    }

    pub fn new_with_input(mem: &Vec<i64>, initial_inputs: Vec<i64>) -> IntCodeComputer {
        let mut c = IntCodeComputer {
            mem: mem.clone(),
            ip: 0,
            inputs: VecDeque::from(initial_inputs),
            outputs: VecDeque::new(),
            rel_base: 0
        };
        c.mem.resize(10000, 0);
        c
    }

    pub fn run(&mut self) -> bool {
        loop {
            match self.get_instruction() {
                Operation::Add(a, b, dest) => {
                    let dest = self.get_addr(dest) as usize;
                    self.mem[dest as usize] = self.get_val(a) + self.get_val(b);
                    self.ip += 4;
                },
                Operation::Mul(a, b, dest) => {
                    let dest = self.get_addr(dest) as usize;
                    self.mem[dest as usize] = self.get_val(a) * self.get_val(b);
                    self.ip += 4;
                },
                Operation::Halt => return false,
                Operation::Output(a) => {
                    self.outputs.push_back(self.get_val(a));
                    self.ip += 2;
                },
                Operation::Input(dest) => {
                    if let Some(i) = self.inputs.pop_front() {
                        self.ip += 2;
                        let dest = self.get_addr(dest) as usize;
                        self.mem[dest as usize] = i;
                    } else {
                        return true;
                    }
                },
                Operation::Equal(a, b, dest) => {
                    let dest = self.get_addr(dest) as usize;
                    self.mem[dest as usize] = if self.get_val(a) == self.get_val(b) { 1 } else { 0 };
                    self.ip += 4;
                },
                Operation::LessThan(a, b, dest) => {
                    let dest = self.get_addr(dest) as usize;
                    self.mem[dest as usize] = if self.get_val(a) < self.get_val(b) { 1 } else { 0 };
                    self.ip += 4;
                },
                Operation::JumpIfFalse(a, d) => {
                    if self.get_val(a) == 0 {
                        self.ip = self.get_val(d) as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                Operation::JumpIfTrue(a, d) => {
                    if self.get_val(a) != 0 {
                        self.ip = self.get_val(d) as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                Operation::RelBaseOffset(p) => {
                    self.rel_base += self.get_val(p);
                    self.ip += 2;
                }
            };
        }
    }

    fn get_val(&self, p: Parameter) -> i64 {
        match p {
            Parameter::Imm(v) => v,
            Parameter::Pos(p) => self.mem[p as usize],
            Parameter::Rel(offset) => self.mem[(self.rel_base + offset) as usize]
        }
    }

    fn get_addr(&self, p: Parameter) -> usize {
        match p {
            Parameter::Pos(p) => p as usize,
            Parameter::Rel(offset) => (self.rel_base + offset) as usize,
            _ => panic!("Tried to get address from imm parameter"),
        }
    }

    fn get_instruction(&self) -> Operation {
        let opcode = self.mem[self.ip];
        match opcode % 100 {
            ADD => Operation::Add(
                self.read_parameter(1),
                self.read_parameter(2),
                self.read_parameter(3)),
            MUL => Operation::Mul(
                self.read_parameter(1),
                self.read_parameter(2),
                self.read_parameter(3)),
            LT => Operation::LessThan(
                self.read_parameter(1),
                self.read_parameter(2),
                self.read_parameter(3)),
            EQ => Operation::Equal(
                self.read_parameter(1),
                self.read_parameter(2),
                self.read_parameter(3)),
            JUMP_IF_FALSE => Operation::JumpIfFalse(
                self.read_parameter(1),
                self.read_parameter(2)),
            JUMP_IF_TRUE => Operation::JumpIfTrue(
                self.read_parameter(1),
                self.read_parameter(2)),
            HALT => Operation::Halt,
            INPUT => Operation::Input(self.read_parameter(1)),
            OUTPUT =>  Operation::Output(self.read_parameter(1)),
            REL_BASE_OFFSET => Operation::RelBaseOffset(self.read_parameter(1)),
            _ => panic!("Invalid opcode {}", opcode),
        }
    }

    fn read_parameter(&self, pnum: u32) -> Parameter {
        let opcode = self.mem[self.ip] as usize;
        let mode = opcode % 10usize.pow(pnum + 2) / 10usize.pow(pnum + 1);
        if mode == 0 {
            Parameter::Pos(self.mem[self.ip + pnum as usize])
        } else if mode == 1 {
            Parameter::Imm(self.mem[self.ip + pnum as usize])
        } else {
            Parameter::Rel(self.mem[self.ip + pnum as usize])
        }
    }
}

pub trait OrderingExt {
    fn as_number(&self) -> i8;
}

impl OrderingExt for Ordering {
    fn as_number(&self) -> i8 {
        match self {
            Ordering::Less => -1,
            Ordering::Greater => 1,
            _ => 0
        }
    }
}