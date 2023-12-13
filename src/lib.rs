#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(const_mut_refs)]
#![feature(const_trait_impl)]

mod interface;
mod utils;
mod gdt;
mod io;
mod shell;
mod idt;
mod keyboard;

use interface::Colors;

use crate::interface::{set_color, reset_screen};

#[no_mangle]
pub extern "C" fn main() -> ! {

    interface::init();
    gdt::init();
    idt::init();

    reset_screen();
    utils::print_header();
    set_color(Colors::White);
    println!();

    shell::print_prompt();

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
