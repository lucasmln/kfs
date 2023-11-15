const VGA_ADDRESS: u32 = 0xB8000;
const WIDTH: usize = 80;
const HEIGHT: usize = 25;
const SCREEN_AMOUNT: u32 = 7;

#[derive(Clone, Copy)]
pub enum Colors {
    Black,
    Blue,
    Green,
    Yellow,
    Red,
    Purple,
    Cyan,
    White,
    Grey,
    BrightBlue,
    BrightGreen,
    BrightYellow,
    BrightRed,
    BrightPurple,
    BrightCyan,
    BrightWhite,
}

#[repr(C)]
pub struct Cell {
    character: u8,
    color: Colors
}
#[repr(transparent)]
struct Buffer {
    cells: [[Cell; WIDTH as usize]; HEIGHT as usize]
}

pub struct Interface {
    pub x: usize,
    pub y: usize,
    // pub vga_address: *mut Cell,
    pub color: Colors,
    pub vga_address: &'static mut Buffer,
}
impl Default for Interface {
    fn default() -> Self {
        Self { x: 0, y: 0, color: Colors::White, vga_address: unsafe { &mut *(VGA_ADDRESS as *mut Buffer) } }
    }
}
impl Interface {

    pub fn get_cursor(&self) -> (usize, usize) {
        return (self.x, self.y)
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        assert!(x < WIDTH);
        assert!(y < HEIGHT);
        self.x = x;
        self.y = y;
    }

    fn clear_line(&mut self, n: u32) {
        let cell: Cell = Cell { character: 0, color: Colors::Black };
        for x in 0..WIDTH { 
            // self.print_char(x, n, &cell);
        }
    }

    // check color and reset it
    pub fn reset_screen(&mut self) {
        let cell: Cell = Cell { character: 0, color: Colors::Black };

        self.x = 0;
        self.y = 0;
        self.color = Colors::Black;
        for _ in 0..HEIGHT { 
            for _ in 0..WIDTH {
                self.print_char(0);
            }
        }
        self.x = 0;
        self.y = 0;
        self.color = Colors::White;
    }

    pub fn print_char(&mut self, character: u8) {
        // if self.x >= WIDTH {

        //     // Reprint the screen moving each line 1 above
        //     for i in 0..HEIGHT * WIDTH - WIDTH {
        //         unsafe {
        //             // (*self.vga_address.offset(i as isize)).character = (*self.vga_address.offset((i + WIDTH) as isize)).character;
        //             // (*self.vga_address.offset(i as isize)).color = (*self.vga_address.offset((i + WIDTH) as isize)).color;
        //         }
        //     }

        //     // Clear the last line 
        //     self.clear_line(HEIGHT - 1);
        //     self.cursor = HEIGHT * WIDTH - WIDTH;
        // }
        // if character == b'\n' {
        //     self.cursor = self.cursor + WIDTH - self.cursor % WIDTH;
        // }
        // else {
            self.vga_address.cells[self.y][self.x].character = character;
            self.vga_address.cells[self.y][self.x].color = self.color;
            // (*self.vga_address.offset(self.cursor as isize)).character = cell.character;
            // (*self.vga_address.offset(self.cursor as isize)).color = cell.color;
            self.x += 1;
        // }
    }

    pub fn print_string(&mut self, str: &str) {
        for byte in str.bytes() {
            self.print_char(byte)
        }
    }

    // pub fn print_number(&mut self, nbr: i32, color: &Colors)
    // {
    //     let mut is_neg = 1;
    //     if nbr < 0 {
    //         self.print_char(&b'-', color);
    //         is_neg = -1;
    //     }
    //     if nbr >= 10 || nbr <= -10 {
    //         self.print_number(nbr / (10 * is_neg), color);
    //     }
    //     self.print_char( &u8::try_from(((nbr % (10 * is_neg)) * is_neg) + 48).unwrap(), color);
    // }
}
impl fmt::Write for Interface {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print_string(s);
        // Obligé de return un result, d'ou le Ok(())
        // et comme ca, le unwrap() dans le _print() va jamais panic.
        Ok(())
    }
}

use core::fmt::{self, Write};

use lazy_static::lazy_static;
// Have to use spin 
use spin::Mutex;

// https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
lazy_static! {
    pub static ref INTERFACE: Mutex<Interface> = Mutex::new(Interface::default());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::interface::_print(format_args!($($arg)*));
    };
}

pub fn _print(args: fmt::Arguments) {
    INTERFACE.lock().write_fmt(args).unwrap();
}

pub fn set_color(color: Colors) {
    INTERFACE.lock().color = color;
}
