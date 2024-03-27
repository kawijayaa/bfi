use std::{
    io::{stdin, stdout, Write},
    num::Wrapping,
    process::exit,
};

pub enum OPERATION {
    Add,
    Subtract,
    PointerRight,
    PointerLeft,
    Input,
    Output,
    JumpIfZero(usize),
    JumpIfNonZero(usize),
}

pub type Cells = [Wrapping<u8>; 30000];

pub fn lexer(data: &String) -> Vec<OPERATION> {
    let mut bracket_stack: Vec<usize> = Vec::new();
    let mut operations: Vec<OPERATION> = Vec::new();

    for (i, char) in data.chars().enumerate() {
        match char {
            '-' => {
                operations.push(OPERATION::Subtract);
            }
            '+' => {
                operations.push(OPERATION::Add);
            }
            '>' => {
                operations.push(OPERATION::PointerRight);
            }
            '<' => {
                operations.push(OPERATION::PointerLeft);
            }
            '.' => {
                operations.push(OPERATION::Output);
            }
            ',' => {
                operations.push(OPERATION::Input);
            }
            '[' => {
                operations.push(OPERATION::JumpIfZero(0));
                bracket_stack.push(operations.len() - 1);
            }
            ']' => match bracket_stack.pop() {
                Some(pop_index) => {
                    operations[pop_index] = OPERATION::JumpIfZero(operations.len());
                    operations.push(OPERATION::JumpIfNonZero(pop_index));
                }
                None => {
                    eprintln!("ERR: Unmatched brackets on char {}", i + 1);
                    exit(1);
                }
            },
            _ => {}
        }
    }

    return operations;
}

pub fn interpret(operations: Vec<OPERATION>, cells: &mut Cells, cp: &mut usize) {
    let mut ip = 0;

    while ip < operations.len() {
        match operations[ip] {
            OPERATION::Add => {
                cells[*cp] += 1;
            }
            OPERATION::Subtract => {
                cells[*cp] -= 1;
            }
            OPERATION::Input => {
                let mut input = String::new();

                print!("input> ");
                let _ = stdout().flush();

                match stdin().read_line(&mut input) {
                    Ok(_) => {}
                    Err(_) => {
                        eprintln!("Cannot read input!");
                        exit(1);
                    }
                }

                match input.trim().parse::<u8>() {
                    Ok(num) => {
                        cells[*cp] = Wrapping(num);
                    }
                    Err(_) => {
                        eprintln!("Cannot parse input as u8!");
                        exit(1);
                    }
                }
            }
            OPERATION::Output => {
                print!("{}", cells[*cp].0 as char);
            }
            OPERATION::PointerLeft => {
                *cp -= 1;
            }
            OPERATION::PointerRight => {
                *cp += 1;
            }
            OPERATION::JumpIfZero(jump_addr) => {
                if cells[*cp].0 == 0 {
                    ip = jump_addr;
                }
            }
            OPERATION::JumpIfNonZero(jump_addr) => {
                if cells[*cp].0 != 0 {
                    ip = jump_addr;
                }
            }
        }
        ip += 1;
    }
}
