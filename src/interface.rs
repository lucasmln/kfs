const VGA_ADDRESS: u32 = 0xB8000;
const WIDTH: u32 = 80;
const HEIGHT: u32 = 25;
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

    fn print_cell_at_offset(&mut self, x: u32, y: u32, cell: &Cell) {
        assert!(x < WIDTH);
        assert!(y < HEIGHT);
        unsafe {
            (*self.vga_address.offset((x + (WIDTH * y)) as isize)).character = cell.character;
            (*self.vga_address.offset((x + (WIDTH * y)) as isize)).color = cell.color;
        }
    }

    fn print_char_at_offset(&mut self, x: u32, y: u32, character: &u8, color: &Colors) {
        let cell = Cell { character: *character, color: *color as u8 };
        self.print_cell_at_offset(x, y, &cell)
    }

    fn print_cell(&mut self, cell: &Cell) {
        if self.cursor >= HEIGHT * WIDTH {

            // Reprint the screen moving each line 1 above
            for i in 0..HEIGHT * WIDTH - WIDTH {
                unsafe {
                    (*self.vga_address.offset(i as isize)).character = (*self.vga_address.offset((i + WIDTH) as isize)).character;
                    (*self.vga_address.offset(i as isize)).color = (*self.vga_address.offset((i + WIDTH) as isize)).color;
                }
            }

            // Clear the last line 
            self.clear_line(HEIGHT - 1);
            self.cursor = HEIGHT * WIDTH - WIDTH;
        }
        if cell.character == b'\n' {
            self.cursor = self.cursor + WIDTH - self.cursor % WIDTH;
        }
        else {
            unsafe {
                (*self.vga_address.offset(self.cursor as isize)).character = cell.character;
                (*self.vga_address.offset(self.cursor as isize)).color = cell.color;
            }
            self.cursor += 1;
        }
    }

    pub fn get_cursor(&self) -> (u32, u32) {
        return (self.cursor % WIDTH, self.cursor / WIDTH)
    }

    pub fn set_cursor(&mut self, x: u32, y: u32) {
        assert!(x < WIDTH);
        assert!(y < HEIGHT);
        self.cursor = x + (WIDTH * y)
    }

    fn clear_line(&mut self, n: u32) {
        let cell: Cell = Cell { character: 0, color: 0 };
        for x in 0..WIDTH { 
            self.print_cell_at_offset(x, n, &cell);
        }
    }

    pub fn clear_screen(&mut self) {
        let cell: Cell = Cell { character: 0, color: 0 };

        self.cursor = 0;
        for _ in 0..WIDTH * HEIGHT { 
            self.print_cell(&cell);
        }
        self.cursor = 0;
    }

    pub fn print_char(&mut self, character: &u8, color: &Colors) {
        let cell = Cell { character: *character, color: *color as u8 };
        self.print_cell(&cell);
    }

    pub fn print_string(&mut self, str: &[u8], color: &Colors) {
        for (_, byte) in str.iter().enumerate() {
            self.print_char(byte, color)
        }
    }

    pub fn print_number(&mut self, nbr: i32, color: &Colors)
    {
        let mut is_neg = 1;
        if nbr < 0 {
            self.print_char(&b'-', color);
            is_neg = -1;
        }
        if nbr >= 10 || nbr <= -10 {
            self.print_number(nbr / (10 * is_neg), color);
        }
        self.print_char( &u8::try_from(((nbr % (10 * is_neg)) * is_neg) + 48).unwrap(), color);
    }
}

pub static interface = Interface::default();