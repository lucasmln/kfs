#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod interface;
mod utils;
mod gdt;
mod shell;
mod idt;
mod keyboard;
mod asm;

use interface::Colors;

#[no_mangle]
pub extern "C" fn main() -> ! {

    gdt::init();
    idt::init();

    cli!();

    interface::reset_screen();
    utils::print_header();
    interface::set_color(Colors::White);
    println!();

    shell::print_prompt();

    sti!();
    loop {
        hlt!();
    }

}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let arg = format_args!("");
    let message =  _info.message().unwrap_or(&arg);
    let location = _info.location().unwrap();

    interface::set_color(Colors::BrightRed);
    print!("[PANIC ");
    interface::set_color(Colors::BrightWhite);
    print!("{}", location);
    interface::set_color(Colors::BrightRed);
    print!("]: ");
    interface::set_color(Colors::BrightWhite);
    println!("{}", message);
    loop {}
}
