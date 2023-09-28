pub const VGA_ADDRESS: u32 = 0xB8000;
const WIDTH: u32 = 80;
const HEIGHT: u32 = 25;

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

pub struct Cell {
    character: u8,
    color: u8
}

pub struct Interface {
    pub cursor: u32,
    pub vga_address: *mut Cell,
}

pub unsafe fn print_at_offset(x: u32, y: u32, interface: &Interface, cell: &Cell) {
    assert!(x < WIDTH);
    assert!(y < HEIGHT);
    (*interface.vga_address.offset((x + (HEIGHT * y)) as isize)).character = cell.character;
    (*interface.vga_address.offset((x + (HEIGHT * y)) as isize)).color = cell.color;
}

pub unsafe fn print_cell(interface: &mut Interface, cell: &Cell) {
    (*interface.vga_address.offset(interface.cursor as isize)).character = cell.character;
    (*interface.vga_address.offset(interface.cursor as isize)).color = cell.color;
    interface.cursor += 1;
}

pub unsafe fn print_char(interface: &mut Interface, character: &u8, color: &Colors) {
    (*interface.vga_address.offset(interface.cursor as isize)).character = *character;
    (*interface.vga_address.offset(interface.cursor as isize)).color = *color as u8;
    interface.cursor += 1;
}

pub fn set_cursor(x: u32, y: u32, interface: &mut Interface) {
    assert!(x < WIDTH);
    assert!(y < HEIGHT);
    interface.cursor = x + (HEIGHT * y)
}

pub unsafe fn print_string(interface: &mut Interface, str: &[u8], color: &Colors) {
    for (_, byte) in str.iter().enumerate() {
        if *byte == b'\n' {
            interface.cursor = interface.cursor + WIDTH - interface.cursor % WIDTH
        }
        else {
            print_char(interface, byte, color)
        }
    }
}

pub unsafe fn clear_screen(interface: &mut Interface) {
    let cell: Cell = Cell { character: 0, color: 0 };

    interface.cursor = 0;
    for _ in 0..WIDTH * HEIGHT { 
        print_cell(interface, &cell);
    }
    interface.cursor = 0;
}
