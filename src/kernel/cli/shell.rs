use std::io::Write;
use crate::cli::executor::execute_command;
use crate::cli::parser::parse_command;

pub fn run() {
    loop {
        print!("starOS> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let command = parse_command(&input);
        execute_command(command)
    }
}