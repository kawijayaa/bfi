use std::{
    env::args,
    fs::File,
    io::{stdin, stdout, BufReader, Read, Write},
    num::Wrapping,
    process::exit,
};

enum OPERATION {
    Add,
    Subtract,
    PointerRight,
    PointerLeft,
    Input,
    Output,
    JumpIfZero(usize),
    JumpIfNonZero(usize),
}

fn main() {
    if args().len() != 2 {
        println!("Usage: ./bfi <source_code.bf>");
        exit(1);
    }

    let source_path = args().into_iter().nth(1).unwrap();
    let source_file = match File::open(&source_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Cannot open file {}", &source_path);
            exit(1);
        }
    };

    let mut buffer_reader = BufReader::new(source_file);
    let mut source_data = String::new();
    let _ = buffer_reader.read_to_string(&mut source_data);

    let mut bracket_stack: Vec<usize> = Vec::new();
    let mut operations: Vec<OPERATION> = Vec::new();

    // Lexer
    for (i, char) in source_data.chars().enumerate() {
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

    // Interpreter
    let mut cells: [Wrapping<u8>; 30000] = [Wrapping(0); 30000];
    let mut ip = 0;
    let mut cp = 0;

    while ip < operations.len() {
        match operations[ip] {
            OPERATION::Add => {
                cells[cp] += 1;
            }
            OPERATION::Subtract => {
                cells[cp] -= 1;
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
                        cells[cp] = Wrapping(num);
                    }
                    Err(_) => {
                        eprintln!("Cannot parse input as u8!");
                        exit(1);
                    }
                }
            }
            OPERATION::Output => {
                print!("{}", cells[cp].0 as char);
            }
            OPERATION::PointerLeft => {
                cp -= 1;
            }
            OPERATION::PointerRight => {
                cp += 1;
            }
            OPERATION::JumpIfZero(jump_addr) => {
                if cells[cp].0 == 0 {
                    ip = jump_addr;
                }
            }
            OPERATION::JumpIfNonZero(jump_addr) => {
                if cells[cp].0 != 0 {
                    ip = jump_addr;
                }
            }
        }
        ip += 1;
    }
}
