use std::io;

fn run(input: Vec<Instruction>) {
    let mut inp_iter = input.iter();
    let mut current_op = None;
    let mut x = 1i32;
    let mut res = 0;

    let mut disp = vec![vec![false; 40]; 6];

    for c in 0..240usize {
        let crt_y = c / 40;
        let crt_x = c % 40;

        if crt_x as i32 <= x + 1 && crt_x as i32 >= x - 1 {
            disp[crt_y][crt_x] = true;
        }

        if [20, 60, 100, 140, 180, 220].contains(&(c + 1)) {
            res += (c + 1) as i32 * x;
        }

        if let Some(Instruction::Addx(val)) = current_op.take() {
            x += val;
            current_op = None;
        } else {
            let op = inp_iter.next().expect("No more instructions!");
            if let addx @ Instruction::Addx(_) = op {
                current_op = Some(addx.clone())
            }
        }
    }

    println!("Part 1: {res}");

    println!("Part 2:");
    aoclib::print_bool_matrix(&disp);
}

fn main() -> io::Result<()> {
    let input = aoclib::read_input_lines()?;
    let input = parse_input(input);

    run(input);

    Ok(())
}

#[derive(Clone, Debug)]
enum Instruction {
    Nop,
    Addx(i32),
}

fn parse_input(input: Vec<String>) -> Vec<Instruction> {
    input
        .into_iter()
        .map(|l| {
            if l == "noop" {
                return Instruction::Nop;
            } else if let Some(val) = l.strip_prefix("addx ") {
                if let Ok(val) = val.parse() {
                    return Instruction::Addx(val);
                }
            }
            panic!("Unknown instruction {l}")
        })
        .collect()
}
