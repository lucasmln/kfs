#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod interface;

use interface::Interface;
use interface::Colors;

fn print_header(interface: &mut Interface) {
    interface.print_string(b"/* ************************************************************************** */", &Colors::BrightWhite);
    interface.print_string(b"/*                                                                            */", &Colors::BrightWhite);
    interface.print_string(b"/*                                                        :::      ::::::::   */", &Colors::BrightWhite);
    interface.print_string(b"/*   main.rs                                            :+:      :+:    :+:   */", &Colors::BrightWhite);
    interface.print_string(b"/*                                         ,          +:+ +:+         +:+     */", &Colors::BrightWhite);
    interface.print_string(b"/*   By: Arthur and Lucas aka les mecs stylees      +#+  +:+       +#+        */", &Colors::BrightWhite);
    interface.print_string(b"/*                                                +#+#+#+#+#+   +#+           */", &Colors::BrightWhite);
    interface.print_string(b"/*   Created: 2023/11/14 15:09:19 by Lucas             #+#    #+#             */", &Colors::BrightWhite);
    interface.print_string(b"/*   Updated: 2023/11/14 01:24:22 by Arthur           ###   ########.fr       */", &Colors::BrightWhite);
    interface.print_string(b"/*                                                                            */", &Colors::BrightWhite);
    interface.print_string(b"/* ************************************************************************** */", &Colors::BrightWhite);
    let saved_cursor = interface.get_cursor();

    interface.set_cursor(9, saved_cursor.1 - 6);
    interface.print_string(b"Arthur", &Colors::BrightRed);
    interface.set_cursor(37, saved_cursor.1 - 3);
    interface.print_string(b"Arthur", &Colors::BrightRed);

    interface.set_cursor(20, saved_cursor.1 - 6);
    interface.print_string(b"Lucas", &Colors::Green);
    interface.set_cursor(37, saved_cursor.1 - 4);
    interface.print_string(b"Lucas", &Colors::Green);


    interface.set_cursor(saved_cursor.0, saved_cursor.1);
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn main() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `main` by default

    let mut interface = Interface::default();

    interface.clear_screen();
    print_header(&mut interface);
    let str: &[u8] = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\nb\nc\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n";
    interface.print_string(str, &Colors::Green);
    interface.print_string(b"salut la \nteam", &Colors::BrightBlue);
    interface.print_number(-2147483648, &Colors::BrightWhite);

    let a = interface.get_cursor();

    interface.print_number(a.0 as i32, &Colors::White);
    interface.print_char(&b'\n', &Colors::White);
    interface.print_number(a.1 as i32, &Colors::White);

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
