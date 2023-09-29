const VGA_ADDRESS: u32 = 0xB8000;
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
impl Default for Interface {
    fn default() -> Self {
        Self { cursor: 0, vga_address: VGA_ADDRESS as *mut Cell}
    }
}
impl Interface {

    pub unsafe fn print_cell_at_offset(&mut self, x: u32, y: u32, cell: &Cell) {
        assert!(x < WIDTH);
        assert!(y < HEIGHT);
        (*self.vga_address.offset((x + (WIDTH * y)) as isize)).character = cell.character;
        (*self.vga_address.offset((x + (WIDTH * y)) as isize)).color = cell.color;
    }

    pub unsafe fn print_char_at_offset(&mut self, x: u32, y: u32, character: &u8, color: &Colors) {
        let cell = Cell { character: *character, color: *color as u8 };
        self.print_cell_at_offset(x, y, &cell)
    }

    pub unsafe fn print_cell(&mut self, cell: &Cell) {
        if self.cursor >= HEIGHT * WIDTH {
            for i in 0..HEIGHT * WIDTH - WIDTH {
                (*self.vga_address.offset(i as isize)).character = (*self.vga_address.offset((i + WIDTH) as isize)).character;
                (*self.vga_address.offset(i as isize)).color = (*self.vga_address.offset((i + WIDTH) as isize)).color;
            }
            self.clear_line(HEIGHT - 1);
            self.cursor = HEIGHT * WIDTH - WIDTH;
        }
        if cell.character == b'\n' {
            self.cursor = self.cursor + WIDTH - self.cursor % WIDTH;
        }
        else {
            (*self.vga_address.offset(self.cursor as isize)).character = cell.character;
            (*self.vga_address.offset(self.cursor as isize)).color = cell.color;
            self.cursor += 1;
        }
    }

    pub unsafe fn print_char(&mut self, character: &u8, color: &Colors) {
        let cell = Cell { character: *character, color: *color as u8 };
        self.print_cell(&cell);
    }

    pub unsafe fn print_string(&mut self, str: &[u8], color: &Colors) {
        for (_, byte) in str.iter().enumerate() {
            self.print_char(byte, color)
        }
    }

    pub fn set_cursor(&mut self, x: u32, y: u32) {
        assert!(x < WIDTH);
        assert!(y < HEIGHT);
        self.cursor = x + (HEIGHT * y)
    }

    pub unsafe fn clear_line(&mut self, n: u32) {
        let cell: Cell = Cell { character: 0, color: 0 };
        for x in 0..WIDTH { 
            self.print_cell_at_offset(x, n, &cell);
        }
    }

    pub unsafe fn clear_screen(&mut self) {
        let cell: Cell = Cell { character: 0, color: 0 };

        self.cursor = 0;
        for _ in 0..WIDTH * HEIGHT { 
            self.print_cell(&cell);
        }
        self.cursor = 0;
    }
}
