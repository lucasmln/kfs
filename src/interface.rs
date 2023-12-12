use crate::io;

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

    fn enable_cursor(&mut self) {
        // We call `out 0x3d4, 0xa` so that on the `out 0x3d5, 0xe` the cursor will change it's look
        io::outb(0x3d4, 0xa);
        // bits 0-4 control the cursor shape (0x0-0xf range), we chose 0xe because it looks cool
        io::outb(0x3d5, 0xe);
    }
    fn disable_cursor(&mut self) {
        // We call `out 0x3d4, 0xa` so that on the `out 0x3d5, 0x10` the cursor will disapear
        io::outb(0x3d4, 0xa);
        // bit 5 disables the cursor (0xf or 1 << 4)
        io::outb(0x3d5, 0xf);
    }

    fn set_cursor_position(&mut self) {
        // pos of the cursor is calculated the same way character are placed on the screen
        // pos should (and must) be in the range (0-WIDTH*HEIGHT-1)
        let mut x = self.x;
        if x >= WIDTH {
            x -= 1;
        }
        let pos = self.y * WIDTH + x;

        // say we are going to put the lower bits (0-7)
        io::outb(0x3D4, 0x0F);
        // put the lower 8 bits
        io::outb(0x3D5, (pos & 0xff).try_into().unwrap());
        // say we are going to put the upper bits (8-15)
        io::outb(0x3D4, 0x0E);
        // put the upper 8 bits
        io::outb(0x3D5, ((pos >> 8) & 0xff).try_into().unwrap());
    }

    fn clear_line(&mut self, n: usize) {
        for x in 0..WIDTH {
            self.vga_address.cells[n][x].character = b' ';
            self.vga_address.cells[n][x].color = Colors::White;
        }
    }

    pub fn reset_screen(&mut self) {
        self.disable_cursor();
        self.enable_cursor();
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
        if self.x >= WIDTH && character != b'\n' {
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
        self.set_cursor_position();
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

pub fn color_str_to_color(s: &[u8]) -> Option<Colors> {
    // YES, and ?
    match s {
        [b'b', b'l', b'a', b'c', b'k'] => { return Some(Colors::Black) }
        [b'b', b'l', b'u', b'e'] => { return Some(Colors::Blue) }
        [b'g', b'r', b'e', b'e', b'n'] => { return Some(Colors::Green) }
        [b'c', b'y', b'a', b'n']  => { return Some(Colors::Cyan) }
        [b'r', b'e', b'd'] => { return Some(Colors::Red) }
        [b'p', b'u', b'r', b'p', b'l', b'e'] => { return Some(Colors::Purple) }
        [b'y', b'e', b'l', b'l', b'o', b'w'] => { return Some(Colors::Yellow) }
        [b'w', b'h', b'i', b't', b'e'] => { return Some(Colors::White) }
        [b'g', b'r', b'e', b'y'] => { return Some(Colors::Grey) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'b', b'l', b'u', b'e'] => { return Some(Colors::BrightBlue) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'g', b'r', b'e', b'e', b'n'] => { return Some(Colors::BrightGreen) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'c', b'y', b'a', b'n'] => { return Some(Colors::BrightCyan) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'r', b'e', b'd'] => { return Some(Colors::BrightRed) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'p', b'u', b'r', b'p', b'l', b'e'] => { return Some(Colors::BrightPurple) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'y', b'e', b'l', b'l', b'o', b'w'] => { return Some(Colors::BrightYellow) }
        [b'b', b'r', b'i', b'g', b'h', b't', b'_', b'w', b'h', b'i', b't', b'e'] => { return Some(Colors::BrightWhite) }
        _ => { return None }
    }
}

pub fn get_color() -> Colors {
    return INTERFACE.lock().color;
}

pub fn reset_screen() {
    INTERFACE.lock().reset_screen()
}
