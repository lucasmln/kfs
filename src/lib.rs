#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod interface;

use interface::Cell;
use interface::Interface;
use interface::Colors;
use interface::print_string;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    let mut interface = Interface { cursor: 0, vga_address: interface::VGA_ADDRESS as *mut Cell};

    unsafe {
        interface::clear_screen(&mut interface);
        let str: &[u8] = b"salut\ncava ?";
        print_string(&mut interface, str, &Colors::Green);
    }
    // clear_screan(vga_buffer);
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
