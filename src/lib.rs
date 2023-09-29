#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod interface;

use interface::Interface;
use interface::Colors;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    let mut interface = Interface::default();

    unsafe {
        interface.clear_screen();
        let str: &[u8] = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\nb\nc\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n";
        interface.print_string(str, &Colors::Green);
        interface.print_string(b"salut la \nteam", &Colors::BrightBlue);
        core::arch::asm!(
            "hlt"
        )
    }
    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
