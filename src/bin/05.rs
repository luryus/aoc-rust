use std::io;

const ADD: i32 = 1;
const MUL: i32 = 2;
const HALT: i32 = 99;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
const JUMP_IF_TRUE: i32 = 5;
const JUMP_IF_FALSE: i32 = 6;
const LT : i32 = 7;
const EQ: i32 = 8;

enum Parameter {
    Imm(i32),
    Pos(i32)
}

enum Operation {
    Add(Parameter, Parameter, usize),
    Mul(Parameter, Parameter, usize),
    Halt,
    Input(usize),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, usize),
    Equal(Parameter, Parameter, usize)
}

fn read_parameter(pnum: u32, instr_offset: usize, mem: &Vec<i32>) -> Parameter {
    let opcode = mem[instr_offset] as usize;
    let mode = opcode % 10usize.pow(pnum + 2) / 10usize.pow(pnum + 1);
    if mode == 0 {
        Parameter::Pos(mem[instr_offset + pnum as usize])
    } else {
        Parameter::Imm(mem[instr_offset + pnum as usize])
    }
}

fn get_instruction(mem: &Vec<i32>, pos: i32) -> Operation {
    assert!(pos >= 0);
    let pos = pos as usize;
    let opcode = mem[pos];
    match opcode % 100 {
        ADD => Operation::Add(
            read_parameter(1, pos, mem),
            read_parameter(2, pos, mem),
            mem[pos + 3] as usize),
        MUL => Operation::Mul(
            read_parameter(1, pos, mem),
            read_parameter(2, pos, mem),
            mem[pos + 3] as usize),
        LT => Operation::LessThan(
            read_parameter(1, pos, mem),
            read_parameter(2, pos, mem),
            mem[pos + 3] as usize),
        EQ => Operation::Equal(
            read_parameter(1, pos, mem),
            read_parameter(2, pos, mem),
            mem[pos + 3] as usize),
        JUMP_IF_FALSE => Operation::JumpIfFalse(
            read_parameter(1, pos, mem),
            read_parameter(2, pos, mem)),
        JUMP_IF_TRUE => Operation::JumpIfTrue(
            read_parameter(1, pos, mem),
            read_parameter(2, pos, mem)),
        HALT => Operation::Halt,
        INPUT => Operation::Input(mem[pos + 1] as usize),
        OUTPUT =>  Operation::Output(read_parameter(1, pos, mem)),
        _ => panic!("Invalid opcode {}", opcode),
    }
}

fn get_val(p: Parameter, mem: &Vec<i32>) -> i32 {
    match p {
        Parameter::Imm(v) => v,
        Parameter::Pos(p) => mem[p as usize],
    }
}

fn run(inp: &Vec<i32>, i: i32) {
    let mut mem = inp.clone();

    let mut ip = 0;
    loop {
        match get_instruction(&mem, ip) {
            Operation::Add(a, b, dest) => {
                mem[dest] = get_val(a, &mem) + get_val(b, &mem);
                ip += 4;
            },
            Operation::Mul(a, b, dest) => {
                mem[dest] = get_val(a, &mem) * get_val(b, &mem);
                ip += 4;
            },
            Operation::Halt => return,
            Operation::Output(a) => {
                println!("OUT {}", get_val(a, &mem));
                ip += 2;
            },
            Operation::Input(dest) => {
                mem[dest] = i;
                ip += 2;
            },
            Operation::Equal(a, b, dest) => {
                mem[dest] = if get_val(a, &mem) == get_val(b, &mem) { 1 } else { 0 };
                ip += 4;
            },
            Operation::LessThan(a, b, dest) => {
                mem[dest] = if get_val(a, &mem) < get_val(b, &mem) { 1 } else { 0 };
                ip += 4;
            },
            Operation::JumpIfFalse(a, d) => {
                if get_val(a, &mem) == 0 {
                    ip = get_val(d, &mem);
                } else {
                    ip += 3;
                }
            },
            Operation::JumpIfTrue(a, d) => {
                if get_val(a, &mem) != 0 {
                    ip = get_val(d, &mem);
                } else {
                    ip += 3;
                }
            }
        };
    }
}

fn main() -> io::Result<()> {
    let input = aoc2019::read_stdin_to_string().map(|l| {
        l.split(",")
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    })?;

    run(&input, 1);
    println!("---");
    run(&input, 5);

    Ok(())
}