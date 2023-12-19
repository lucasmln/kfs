use crate::{gdt, print, println, interface::{self, get_color, Colors}, utils::{self, get_kernel_address}, idt, printdel};

fn print_gdt() {
    gdt::print_gdt();
}

fn print_idt() {
    idt::print_idt();
}

fn print_help(command: Option<&[u8]>) {
    let help_help = "This is the main help command.\nAvailable commands:
    - help <command>
    - echo <string>
    - print_gdt
    - print_idt
    - set_color <color>
    - panic
    - x <address>
    - reboot
Type `help <command>` for help on a specific command.";
    let help_echo = "echo:\nDisplay the line of text submitted";
    let help_print_gdt = "print_gdt:\nRetreive and print the gdt pointer retreived from `sgdt` as well as the gdt table";
    let help_print_idt = "print_idt:\nRetreive and print the idt pointer retreived from `sidt` as well as the idt table";
    let help_set_color = "set_color:\nChoose the color of the text outputted to the screen\nChoose from:
    - black\n    - blue\n    - green\n    - cyan\n    - red\n    - purple\n    - yellow\n    - white
    - grey\n    - bright_blue\n    - bright_green\n    - bright_cyan\n    - bright_red
    - bright_purple\n    - bright_yellow\n    - bright_white\n";
    let help_panic = "panic:\nJust call panic!();";
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
    let help_reboot = "reboot:\nReboot the kernel by loading an invalid idt.";

    let _help = "help".as_bytes();
    let _echo = "echo".as_bytes();
    let _print_gdt = "print_gdt".as_bytes();

    match command {
        Some(x) => {
            if x.starts_with("help".as_bytes()) { println!("{}", help_help); }
            else if x.starts_with("echo".as_bytes()) { println!("{}", help_echo); }
            else if x.starts_with("print_gdt".as_bytes()) { println!("{}", help_print_gdt); }
            else if x.starts_with("print_idt".as_bytes()) { println!("{}", help_print_idt); }
            else if x.starts_with("set_color".as_bytes()) { println!("{}", help_set_color); }
            else if x.starts_with("panic".as_bytes()) { println!("{}", help_panic); }
            else if x.starts_with("x".as_bytes()) { println!("{}", help_x); }
            else if x.starts_with("reboot".as_bytes()) { println!("{}", help_reboot); }
            else { unknown_command(Some(x)); }
        }
        None => { println!("{}", help_help); }
    }
}

pub fn print_prompt() {
    let prompt = ">> ";
    let color = get_color();
    interface::set_color(Colors::White);
    print!("{}", prompt);
    interface::set_color(color);
}

fn set_color(s: Option<&[u8]>) {
    match s {
        Some(color_str) => {
            let color = interface::color_str_to_color(color_str).unwrap_or(Colors::White);
            interface::set_color(color);
        }
        None => {}
    }
}

fn unknown_command(command: Option<&[u8]>) {
    let command = command.unwrap_or("None".as_bytes());

    print!("Unknown command ");
    for c in command { print!("{}", *c as char); }
    println!(" type `help` for a list of available commands.");
}

fn echo(a: core::slice::Split<'_, u8, impl FnMut(&u8) -> bool>) {
    for i in a {
        for j in i {
            print!("{}", *j as char);
        }
        print!(" ");
    }
    println!();
}

fn get_entry_amout_per_line(format: u8, smod: u8) -> usize {
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

fn get_entry_len(format: u8, smod: u8) -> usize {
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

use atoi::FromRadix16;
use atoi::FromRadix10;

fn print_memory(mut command: &[u8], address_str: Option<&[u8]>) {
    let mut format = b'x';
    let mut size: usize = 1;
    let mut smod = b'w';

    let mut address: usize;
    let address_str = address_str.unwrap_or(&[b'0']);
    if address_str.starts_with(b"0x") {
        (address, _) = usize::from_radix_16(&address_str[2..])
    }
    else {
        (address, _) = usize::from_radix_10(address_str);
    }

    if command.contains(&b'/') && command.len() > 2 {
        command = &command[2..];
        if command[0].is_ascii_digit() {
            (command, size) = utils::atoi_with_rest::<usize>(command).unwrap_or((command, 1));
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
                let mut counter: usize = 0;
                unsafe {
                    print!("\"");
                    for _ in 0..200 {
                        let c = *get_kernel_address::<u8>(address);
                        if c == 0 {
                            if counter == 0 {
                                address += 1;
                            }
                            break;
                        }
                        address += 1;
                        if (32..126).contains(&c) {
                            print!("{}", c as u8 as char);
                        }
                        else {
                            print!("\\{:o}", c);
                        }
                        counter += 1;
                    }
                    print!("\"");
                    if counter == 200 && *get_kernel_address::<u8>(address) != 0 {
                        print!("...");
                    }
                }
            }
            _ => {}
        }
        if (i + 1) % entry_amount_per_line == 0 && i != size - 1 {
            println!();
        }
        address += entry_len;
    }
    println!();
}

pub fn interpret(mut shell_str: &[u8])
{
    while shell_str.len() >= 1 && shell_str[shell_str.len() - 1] == 0 {
        shell_str = shell_str.strip_suffix(&[0]).unwrap_or(&[]);
    }

    let mut shell_str_splitted = shell_str.split(|x| *x == b' ');
    let command = shell_str_splitted.next();

    match command {
        Some(x) => {
            if x.starts_with("help".as_bytes()) { print_help(shell_str_splitted.next()); }
            else if x.starts_with("echo".as_bytes()) { echo(shell_str_splitted); }
            else if x.starts_with("print_gdt".as_bytes()) { print_gdt(); }
            else if x.starts_with("print_idt".as_bytes()) { print_idt(); }
            else if x.starts_with("set_color".as_bytes()) { set_color(shell_str_splitted.next()); }
            else if x.starts_with("panic".as_bytes()) { panic!("You called panic !"); }
            else if x.starts_with("x".as_bytes()) { print_memory(x, shell_str_splitted.next()); }
            else if x.starts_with("reboot".as_bytes()) { idt::reboot(); }
            else if x == [] {}
            else { unknown_command(Some(x)); }
        }
        None => { }
    }
}

pub fn read(key: &[u8]) {
    static mut C_LEN: usize = 0;
    static mut BUFFER: [u8; 100] = [0; 100];

    for c in key {
        match c {
            10 => {
                println!();
                unsafe {
                    interpret(&mut BUFFER);
                    C_LEN = 0;
                    for e in BUFFER.iter_mut() { *e = 0; }
                }
                print_prompt();
            }
            127 => {
                unsafe {
                    if C_LEN > 0 {
                        C_LEN -= 1;
                        BUFFER[C_LEN] = b'\x00';
                        printdel!();
                    }
                }

            }
            _ => {
                unsafe {
                    if C_LEN < 99 {
                        print!("{}", *c as char);
                        BUFFER[C_LEN] = *c;
                        C_LEN += 1;
                    }
                }
            }
        }
    }
}
