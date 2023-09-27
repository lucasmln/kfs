#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

const VGA_ADDRESS: u32 = 0xB8000;
const WIDTH: u32 = 80;
const HEIGHT: u32 = 25;

struct Cell {
    character: u8,
    color: u8
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default

    let vga_buffer: *mut Cell = VGA_ADDRESS as *mut Cell;

    clear_screan(vga_buffer);
    unsafe {
        (*vga_buffer).character = b'a';
        (*vga_buffer).color = 25;
        (*vga_buffer.offset(1)).character = b'c';
    }
    clear_screan(vga_buffer);
    loop {}
}

use core::ffi::c_void;
use core::ffi::c_int;
use core::ffi::c_char;
use core::ffi::c_uint;

#[no_mangle]
pub unsafe extern "C" fn memset(dest: *mut c_void, c: c_int, n: c_uint) -> *mut c_void {
    let rdest: *mut c_int = dest as *mut c_int;
    *rdest.offset(0) = c;
    return dest;
}

#[no_mangle]
pub unsafe extern "C" fn strlen(cs: *const c_char) -> c_uint {
    let mut i: c_uint = 0;
    while cs.offset(i as isize) != core::ptr::null() {
        i += 1;
    }
    return i
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn clear_screan(vga_buffer: *mut Cell) {
    for i in 0..WIDTH * HEIGHT { 
        unsafe {
            (*vga_buffer.offset(i as isize)).character = b'a';
            (*vga_buffer.offset(i as isize)).color = 0;
        }
    }
}