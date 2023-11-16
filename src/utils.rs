// use core::ffi::c_uint;
// use core::ffi::c_void;
// use core::ffi::c_int;

// #[no_mangle]
// pub unsafe extern "C" fn memset(
//     dest: *mut c_void,
//     c: c_int,
//     n: c_uint
// ) -> *mut c_void { return dest; }

// #[no_mangle]
// pub unsafe extern "C" fn memcpy(
//     dest: *mut c_void,
//     src: *const c_void,
//     n: c_uint
// ) -> *mut c_void { return dest; }

// #[no_mangle]
// pub unsafe extern "C" fn memcmp(
//     cx: *const c_void,
//     ct: *const c_void,
//     n: c_uint
// ) -> c_int { return 0; }

// mod interface;

use crate::{interface::{set_color, Colors}, print, println};

pub fn print_header() {
    let ft_color = Colors::BrightYellow;
    let border_color = Colors::BrightCyan;
    let name_color = Colors::BrightGreen;
    let filename_color = Colors::BrightPurple;

    set_color(border_color);
    print!("/* ************************************************************************** */");
    print!("/*                                                                            */");
    print!("/*                                                        ");
    set_color(ft_color);
    print!(":::      ::::::::");
    set_color(border_color);
    print!("   *//*   ");
    set_color(filename_color);
    print!("main.rs                                            ");
    set_color(ft_color);
    print!(":+:      :+:    :+:");
    set_color(border_color);
    print!("   *//*                                         ,          ");
    set_color(ft_color);
    print!("+:+ +:+         +:+");
    set_color(border_color);
    print!("     *//*   By: ");
    set_color(name_color);
    print!("Arthur");
    set_color(border_color);
    print!(" and ");
    set_color(name_color);
    print!("Lucas");
    set_color(border_color);
    print!(" aka les mecs stylees      ");
    set_color(ft_color);
    print!("+#+  +:+       +#+");
    set_color(border_color);
    print!("        *//*                                                ");
    set_color(ft_color);
    print!("+#+#+#+#+#+   +#+");
    set_color(border_color);
    print!("           *//*   Created: 2023/11/14 15:09:19 by ");
    set_color(name_color);
    print!("Lucas             ");
    set_color(ft_color);
    print!("#+#    #+#");
    set_color(border_color);
    print!("             *//*   Updated: 2023/11/14 01:24:22 by ");
    set_color(name_color);
    print!("Arthur           ");
    set_color(ft_color);
    print!("###   ########.fr");
    set_color(border_color);
    print!("       *//*                                                                            */");
    print!("/* ************************************************************************** */");
}

#[allow(dead_code)]
pub fn test_color() {
    set_color(Colors::Black);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Blue);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Green);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Cyan);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Red);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Purple);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Yellow);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::White);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::Grey);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightBlue);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightGreen);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightCyan);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightRed);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightPurple);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightYellow);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
    set_color(Colors::BrightWhite);
    println!("Lucas ABCDEFGHIJKLMNOPQRSTUVXYZ !###$//.;");
}
