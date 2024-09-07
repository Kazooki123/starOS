mod memory;
mod process;
mod filesystem;
mod device;
mod interrupts;
mod syscalls;
mod panic;
mod allocator;
mod graphics_interface;
mod cli;

fn main() {
    println!("L.O.L");
    cli::start();
}