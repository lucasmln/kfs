#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod interface;
mod utils;

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

    unsafe {
        interface.clear_screen();
        // let str: &[u8] = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\nb\nc\n1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n16\n17\n18\n19\n20\n21\n22\n";
        // interface.print_string(str, &Colors::Green);
        // interface.print_string(b"salut la \nteam", &Colors::BrightBlue);
    }
    let mut save = 0;
    let s:&[u8] = b"0123456789abcdefghijklmopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ?,.;/:+=%*$&'(!)_-><";
    let key_array: [char; 90] = [
                '\0',
                '\0',
                '1',
                '2',
                '3',
                '4',
                '5',
                '6',
                '7',
                '8',
                '9',
                '0',
                '-',
                '=',
                127 as char,
                '\0',
                'q',
                'w',
                'e',
                'r',
                't',
                'y',
                'u',
                'i',
                'o',
                'p',
                '[',
                ']',
                '\n',
                '\0',
                'a',
                's',
                'd',
                'f',
                'g',
                'h',
                'j',
                'k',
                'l',
                ';',
                '\'',
                '`',
                '\0',
                '\\',
                'z',
                'x',
                'c',
                'v',
                'b',
                'n',
                'm',
                ',',
                '.',
                '/',
                '\0',
                '*',
                '\0',
                ' ',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
                '7',
                '8',
                '9',
                '-',
                '4',
                '5',
                '6',
                '+',
                '1',
                '2',
                '3',
                '0',
                '.',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
                '\0',
            ];
    loop {
        let index = utils::readline();
        unsafe {
            // if index != save {
            interface.print_number(&(key_array[index as usize] as u8), &Colors::Green);
            // }
            save = index;
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
