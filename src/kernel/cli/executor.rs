
pub fn execute_command(command: (String, Vec<String>)) {
    match command.0.as_str() {
        "echo" => println!("{}", command.1.join(" ")),
        "exit" => std::process::exit(0),

        _ => println!("Unknown command: {}, please read the docs for more commands", command.0),
    }
}