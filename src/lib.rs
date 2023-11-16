#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod interface;
mod utils;

use interface::Colors;

use crate::interface::{set_color, reset_screen};

#[no_mangle]
pub extern "C" fn main() -> ! {

    reset_screen();
    utils::print_header();
    set_color(Colors::White);
    println!();

    let lucas = "bonjour je m'apelle lucas ! :)";

    let nbr: u128 = 12412421412414;

    println!("{}|{}", lucas, nbr);

    let tab = [0; 25];

    for i in 0..30 {
        let _ = tab[i];
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    set_color(Colors::BrightRed);
    print!("[PANIC]: ");
    set_color(Colors::BrightWhite);
    println!("{}", _info.message().unwrap());
    loop {}
}
