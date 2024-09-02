// VIDEO GRAPHIC ARRAY
// AUTHOR: KAZOOKI123
// LICENSED BY MIT

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Clone)]
struct VgaChar {
    ascii_char: u8,
    color: u8,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;
const VGA_BUFFER: *mut VgaChar = 0xb8000 as *mut VgaChar;

// VGA MAIN
pub fn print_string(s: &str, color: Color) {
    let color_code = color as u8;
    for (i, c) in s.chars().enumerate() {
        let vga_char = VgaChar {
            ascii_char: c as u8,
            color: color_code,
        };
        unsafe {
            *VGA_BUFFER.offset(i as isize) = vga_char;
        }
    }
}