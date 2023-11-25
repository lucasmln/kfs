use crate::{gdt, print, println, interface::{self, get_color, Colors}, utils::{self, get_kernel_address}};

fn print_gdt() {
    gdt::print_gdt();
}

fn print_idt() {
    // idt::print_idt();
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

fn print_help(command: Option<&str>) {
    let help_help = "This is the main help command.\nAvailable commands:
    - help <command>
    - echo <string>
    - print_gdt
    - print_idt
    - set_color <color>
    - x <address>
Type `help <command>` for help on a specific command.";
    let help_echo = "echo:\nDisplay the line of text submitted";
    let help_print_gdt = "print_gdt:\nRetreive and print the gdt pointer retreived from `sgdt` as well as the gdt table";
    let help_print_idt = "print_idt:\nRetreive and print the idt pointer retreived from `sidt` as well as the idt table";
    let help_set_color = "set_color:\nChoose the color of the text outputted to the screen\nChoose from:
    - black\n    - blue\n    - green\n    - cyan\n    - red\n    - purple\n    - yellow\n    - white
    - grey\n    - bright_blue\n    - bright_green\n    - bright_cyan\n    - bright_red
    - bright_purple\n    - bright_yellow\n    - bright_white\n";
    let help_x = "x:\nDisplays the memory contents at a given address using the specified format.
    x [Address expression]
    x/[Format] [Address expression]
    x/[Length][Format] [Address expression]

    Format:
        o - octal
        x - hexadecimal
        d - decimal
        u - unsigned decimal
        t - binary
        f - floating point
        a - address
        c - char
        s - string

    Size modifier:
        b - byte
        h - halfword (16-bit value)
        w - word (32-bit value)
        g - giant word (64-bit value)";

    match command {
        Some(x) => {
            match x {
                "help" => { println!("{}", help_help); }
                "echo" => { println!("{}", help_echo); }
                "print_gdt" => { println!("{}", help_print_gdt); }
                "print_idt" => { println!("{}", help_print_idt); }
                "set_color" => { println!("{}", help_set_color); }
                "x" => { println!("{}", help_x); }
                _ => { unknown_command(Some(x)); }
            }
        }
        None => { println!("{}", help_help); }
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

fn get_entry_amout_per_line(format: u8, smod: u8) -> u32 {
    if format == b's' {
        return 1;
    }
    match smod {
        b'b' => { return 8; }
        b'h' => { return 8; }
        b'w' => { return 4; }
        b'g' => { return 2; }
        _ => { return 1; }
    }
}

fn get_entry_len(format: u8, smod: u8) -> u32 {
    if format == b's' {
        return 0;
    }
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
            if [b'o', b'x', b'd', b'u', b't', b'f', b'a', b'c', b's'].contains(char) {
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
    // 1/b/byte, 2/h/halfword, 4/w/word, 8/g/giant_word
    let entry_amount_per_line = get_entry_amout_per_line(format, smod);
    let entry_len = get_entry_len(format, smod);
    for i in 0..size {
        if i % entry_amount_per_line == 0 {
            print!("{:#x}: ", address);
        }
        match format {
            b'o' => {
                match smod {
                    b'b' => { print!("{:#0o} ", unsafe { *get_kernel_address::<u8>(address) } ); }
                    b'h' => { print!("{:#0o} ", unsafe { *get_kernel_address::<u16>(address) } ); }
                    b'w' => { print!("{:#0o} ", unsafe { *get_kernel_address::<u32>(address) } ); }
                    b'g' => { print!("{:#0o} ", unsafe { *get_kernel_address::<u64>(address) } ); }
                    _ => {}
                }
            }
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
            b'f' => {
                match smod {
                    b'b' => { print!("unsupported"); }
                    b'h' => { print!("unsupported"); }
                    b'w' => { print!("{:} ", unsafe { *get_kernel_address::<f32>(address) } ); }
                    b'g' => { print!("{:} ", unsafe { *get_kernel_address::<f64>(address) } ); }
                    _ => {}
                }
            }
            b'a' => {
                match smod {
                    b'b' => { print!("{:#x} ", unsafe { *get_kernel_address::<u8>(address) } ); }
                    b'h' => { print!("{:#x} ", unsafe { *get_kernel_address::<u16>(address) } ); }
                    b'w' => { print!("{:#x} ", unsafe { *get_kernel_address::<u32>(address) } ); }
                    b'g' => { print!("{:#x} ", unsafe { *get_kernel_address::<u64>(address) } ); }
                    _ => {}
                }
            }
            b'c' => {
                let c: i8;
                unsafe { c = *get_kernel_address::<i8>(address); };
                if (33..126).contains(&c) {
                    print!("{} {} ", c, c as u8 as char);
                }
                else if c == 32 {
                    print!("{} ' ' ", c);
                }
                else {
                    print!("{} '\\{:o}' ", c, c);
                }
            }
            b's' => {
                unsafe {
                    print!("\"");
                    for _ in 0..200 {
                        let c = *get_kernel_address::<i8>(address);
                        if c == 0 {
                            break;
                        }
                        if (32..126).contains(&c) {
                            print!("{}", c as u8 as char);
                        }
                        else {
                            print!("\\{:o}", c);
                        }
                        address += 1;
                    }
                    print!("\"");
                    if *get_kernel_address::<i8>(address) != 0 {
                        print!("...");
                    }
                }
            }
            _ => {}
        }
        if i % entry_amount_per_line == 0 {
            println!();
        }
        address += entry_len;
    }
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
