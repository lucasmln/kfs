use crate::{gdt, print, println, interface::{self, get_color, Colors, get_kernel_address}, utils};

fn print_gdt() {
    gdt::print_gdt();
}

fn print_help(command: Option<&str>) {
    let main_help = "This is the main help command.\nAvailable commands:
    - help
    - echo
    - print_gdt
    - print_idt
    - set_color
Type `help <command>` for help on a specific command.";
    let set_color = "set color help";
    
    match command {
        Some(x) => {
            match x {
                "set_color" => { println!("{}", set_color); }
                _ => {}
            }
        }
        None => { println!("{}", main_help); }
    }
}

fn print_prompt() {
    let prompt = ">> ";
    let color = get_color();
    interface::set_color(Colors::White);
    print!("{}", prompt);
    interface::set_color(color);
}

fn set_color(s: Option<&str>) {
    match s {
        Some(color_str) => {
            let color = interface::color_str_to_color(color_str).unwrap_or(Colors::White);
            interface::set_color(color);
        }
        None => {}
    }
}

fn unknown_command(command: Option<&str>) {
    let command = command.unwrap_or("None");

    println!("Unknown command {} type `help` for a list of available commands.", command);
}

fn echo(a: core::str::Split<'_, char>) {
    for i in a {
        print!("{} ", i);
    }
    println!();
}

fn get_entry_amout_per_line(smod: u8) -> u32 {
    match smod {
        b'b' => { return 8; }
        b'h' => { return 8; }
        b'w' => { return 4; }
        b'g' => { return 2; }
        _ => { return 1; }
    }
}

fn get_entry_len(smod: u8) -> u32 {
    match smod {
        b'b' => { return 1; }
        b'h' => { return 2; }
        b'w' => { return 4; }
        b'g' => { return 8; }
        _ => { return 1; }
    }
}

fn print_memory(command: &str, address_str: Option<&str>) {
    let mut format = b'x';
    let mut size: u32 = 1;
    let mut smod = b'w';

    let mut address: u32;
    let address_str = address_str.unwrap();
    if address_str.starts_with("0x") {
        address = u32::from_str_radix(&address_str[2..], 16).unwrap();
    }
    else {
        address = u32::from_str_radix(address_str, 10).unwrap();
    }

    let mut command = command.as_bytes();
    if command.contains(&b'/') && command.len() > 2 {
        command = &command[2..];
        if command[0].is_ascii_digit() {
            (command, size) = utils::atoi_with_rest::<u32>(command).unwrap_or((command, 1));
        }
        for char in command {
            if [b'o', b'x', b'd', b'u', b't', b'f', b'a', b'c', b's', b'i'].contains(char) {
                format = *char;
            }
            else if [b'b', b'h', b'w', b'g'].contains(char) {
                smod = *char;
            }
            else {
                // unsupported
            }
        }
    }
    // 1 for byte, 2
    let entry_amount_per_line = get_entry_amout_per_line(smod);
    let entry_len = get_entry_len(smod);
    for i in 0..size {
        if i % entry_amount_per_line == 0 {
            print!("\n{:x}: ", address);
        }
        match format {
            b'x' => {
                match smod {
                    b'b' => { print!("{:#04x} ", unsafe { *get_kernel_address::<u8>(address) } ); }
                    b'h' => { print!("{:#06x} ", unsafe { *get_kernel_address::<u16>(address) } ); }
                    b'w' => { print!("{:#010x} ", unsafe { *get_kernel_address::<u32>(address) } ); }
                    b'g' => { print!("{:#018x} ", unsafe { *get_kernel_address::<u64>(address) } ); }
                    _ => {}
                }
            }
            b'd' => {
                match smod {
                    b'b' => { print!("{:2} ", unsafe { *get_kernel_address::<i8>(address) } ); }
                    b'h' => { print!("{:4} ", unsafe { *get_kernel_address::<i16>(address) } ); }
                    b'w' => { print!("{:8} ", unsafe { *get_kernel_address::<i32>(address) } ); }
                    b'g' => { print!("{:16} ", unsafe { *get_kernel_address::<i64>(address) } ); }
                    _ => {}
                }
            }
            b'u' => {
                match smod {
                    b'b' => { print!("{:2} ", unsafe { *get_kernel_address::<u8>(address) } ); }
                    b'h' => { print!("{:4} ", unsafe { *get_kernel_address::<u16>(address) } ); }
                    b'w' => { print!("{:8} ", unsafe { *get_kernel_address::<u32>(address) } ); }
                    b'g' => { print!("{:16} ", unsafe { *get_kernel_address::<u64>(address) } ); }
                    _ => {}
                }
            }
            b't' => {
                match smod {
                    b'b' => { print!("{:08b} ", unsafe { *get_kernel_address::<u8>(address) } ); }
                    b'h' => { print!("{:016b} ", unsafe { *get_kernel_address::<u16>(address) } ); }
                    b'w' => { print!("{:032b} ", unsafe { *get_kernel_address::<u32>(address) } ); }
                    b'g' => { print!("{:064b} ", unsafe { *get_kernel_address::<u64>(address) } ); }
                    _ => {}
                }
            }
            _ => {}
        }
        address += entry_len;
    }
    // println!("{} {} {}", format, size, smod);
}

pub fn interpret(s: &str) {

    let mut a = s.split(' ');
    let command = a.next();
    match command {
        Some(x) => {
            match x {
                "help"      => { print_help(a.next()); }
                "echo"      => { echo(a); }
                "print_gdt" => { print_gdt(); }
                "print_idt" => { print_idt(); }
                "set_color" => { set_color(a.next()); }
                "x"         => { print_memory(x, a.next()); }
                _           => {
                    match &x[..2] {
                        "x/" => { print_memory(x, a.next()) }
                        _ => { unknown_command(Some(x)); }
                    }
                }
            }
        }
        None => { unknown_command(command); }
    }
}

pub fn print_idt() {
    // let mut dtr = IdtPtr::default();

    // unsafe {
    //     core::arch::asm!("sgdt [{0}]", in(reg) &mut dtr);
    //     println!("{:?}", dtr);
    //     let gdt_entry_amout = (dtr.limit + 1) / size_of::<IdtEntry>() as u16;
    //     for i in 0..gdt_entry_amout {
    //         let gdt = &mut *((dtr.base + (i * size_of::<IdtEntry>() as u16) as u32) as *mut IdtEntry);
    //         println!("{:x?}", gdt);
    //     }
    // }
}
