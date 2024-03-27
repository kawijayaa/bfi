use bfi::{interpret, lexer};
use std::{
    env::args,
    fs::File,
    io::{stdin, stdout, BufReader, Read, Write},
    num::Wrapping,
    process::exit,
};

fn main() {
    let mut source_data = String::new();
    let mut cells: [Wrapping<u8>; 30000] = [Wrapping(0); 30000];
    let mut cp: usize = 0;

    match args().len() {
        1 => loop {
            let mut input = String::new();

            print!(">> ");
            let _ = stdout().flush();

            match stdin().read_line(&mut input) {
                Ok(_) => {}
                Err(_) => {
                    eprintln!("Cannot read input!");
                    exit(1);
                }
            }

            let operations = lexer(&input);
            interpret(operations, &mut cells, &mut cp);
            println!();
        },
        2 => {
            let source_path = args().into_iter().nth(1).unwrap();
            let source_file = match File::open(&source_path) {
                Ok(f) => f,
                Err(_) => {
                    eprintln!("Cannot open file {}", &source_path);
                    exit(1);
                }
            };

            let mut buffer_reader = BufReader::new(source_file);
            let _ = buffer_reader.read_to_string(&mut source_data);

            let operations = lexer(&source_data);
            interpret(operations, &mut cells, &mut cp);
        }
        _ => {
            println!("Usage: ./bfi <source_code.bf>");
            exit(1);
        }
    }
}
