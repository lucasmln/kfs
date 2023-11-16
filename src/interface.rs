const VGA_ADDRESS: u32 = 0xB8000;
const WIDTH: usize = 80;
const HEIGHT: usize = 25;
// const SCREEN_AMOUNT: u32 = 7;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Colors {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Purple,
    Yellow,
    White,
    Grey,
    BrightBlue,
    BrightGreen,
    BrightCyan,
    BrightRed,
    BrightPurple,
    BrightYellow,
    BrightWhite,
}

#[repr(C)]
struct Cell {
    character: u8,
    color: Colors
}

struct Buffer {
    cells: [[Cell; WIDTH as usize]; HEIGHT as usize]
}

struct Interface {
    pub x: usize,
    pub y: usize,
    pub color: Colors,
    pub vga_address: &'static mut Buffer,
}

impl Default for Interface {
    fn default() -> Self {
        Self { x: 0, y: 0, color: Colors::White, vga_address: unsafe { &mut *(VGA_ADDRESS as *mut Buffer) } }
    }   
}

impl Interface {

    fn clear_line(&mut self, n: usize) {
        for x in 0..WIDTH {
            self.vga_address.cells[n][x].character = b' ';
            self.vga_address.cells[n][x].color = Colors::White;
        }
    }

    pub fn reset_screen(&mut self) {
        self.x = 0;
        self.y = 0;
        self.color = Colors::White;
        for _ in 0..HEIGHT * WIDTH {
            self.print_char(b' ');
        }
        self.x = 0;
        self.y = 0;
    }

    pub fn print_char(&mut self, character: u8) {
        if self.x >= WIDTH {
            self.x = 0;
            self.y += 1;
        }
        if self.y >= HEIGHT {

            for i in 0..HEIGHT - 1 { 
                for j in 0..WIDTH {
                    self.vga_address.cells[i][j].character = self.vga_address.cells[i + 1][j].character;
                    self.vga_address.cells[i][j].color = self.vga_address.cells[i + 1][j].color;
                }
            }
            self.clear_line(HEIGHT - 1);
            self.y = HEIGHT -1;
        }
        if character == b'\n' {
            self.x = 0;
            self.y += 1;
        }
        else {
            self.vga_address.cells[self.y][self.x].character = character;
            self.vga_address.cells[self.y][self.x].color = self.color;
            self.x += 1;
        }
    }

    pub fn print_string(&mut self, str: &str) {
        for byte in str.bytes() {
            self.print_char(byte)
        }
    }

}

use core::fmt::{self, Write};
impl fmt::Write for Interface {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s);
        Ok(())
    }
}

// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    static ref INTERFACE: Mutex<Interface> = Mutex::new(Interface::default());
}

#[macro_export]
macro_rules! println {
    () => { print!("\n") };
    ($($arg:tt)*) => {
        print!($($arg)*);
        print!("\n");
    }
}

#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => {
        crate::interface::_print(format_args!($($arg)*));
    };
}

// I don't know how to make this function only visible to this file
pub(crate) fn _print(args: fmt::Arguments) {
    INTERFACE.lock().write_fmt(args).unwrap();
}

pub fn set_color(color: Colors) {
    INTERFACE.lock().color = color;
}

pub fn reset_screen() {
    INTERFACE.lock().reset_screen()
}
