#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod interface;
mod utils;
mod gdt;
mod io;
mod idt;
mod keyboard;

use interface::Colors;
use crate::gdt::gdt_install;

use crate::interface::{set_color, reset_screen, get_kernel_address};

extern "C" {
    fn test_function();
}

#[no_mangle]
pub extern "C" fn main() -> ! {

    gdt_install();
    idt::idt_init();

    reset_screen();
    utils::print_header();
    set_color(Colors::White);
    println!();

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let arg = format_args!("");
    let message =  _info.message().unwrap_or(&arg);
    let location = _info.location().unwrap();

    set_color(Colors::BrightRed);
    print!("[PANIC ");
    set_color(Colors::BrightWhite);
    print!("{}", location);
    set_color(Colors::BrightRed);
    print!("]: ");
    set_color(Colors::BrightWhite);
    println!("{}", message);
    loop {}
}
