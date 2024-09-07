mod shell;
mod parser;
mod executor;
mod builtins;

pub fn start() {
    shell::run();
}