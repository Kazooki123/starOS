
pub fn parse_command(input: &str) -> (String, Vec<String>) {
    let mut parts = input.split_whitespace();
    let command = parts.next().unwrap_or("").to_string();
    let args = parts.map(String::from).collect();
    (command, args)
}