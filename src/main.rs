#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

const VGA_ADDRESS: u32 = 0xB8000;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    let vga_bugger: *mut u8 = VGA_ADDRESS as *mut u8;

    unsafe {
        *vga_bugger = ('a' as u8) << 0;
        *vga_bugger.offset(1) = 24;
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
