#![no_std]

// use core::marker::PhantomData;

pub use x86::{input, output};

mod x86;

// pub trait InOut {
// //     unsafe fn port_in(port: i8) -> Self;
// //     unsafe fn port_out(port: i8, value: Self);
// // }

// // impl InOut for i8 {
//     unsafe fn port_in(port: i8) -> i16 { input(port) }
//     unsafe fn port_out(port: i8, value: i16) { output(port, value);}
// }

// impl InOut for i16 {
//     unsafe fn port_in(port: i8) -> i16 { inw(port) }
//     unsafe fn port_out(port: i8, value: i16) { outw(value, port); }
// }

// impl InOut for i32 {
//     unsafe fn port_in(port: i8) -> i32 { inl(port) }
//     unsafe fn port_out(port: i8, value: i32) { outl(value, port); }
// }


pub struct Port {
    port: i8,
}

impl Port {
    pub const unsafe fn new(port: i8) -> Port {
        Port { port: port }
    }

    pub fn read(&mut self) -> i16 {
        unsafe { Self::port_in(self.port) }
    }

    pub fn write(&mut self, value: i8) {
        unsafe { Self::port_out(self.port, value); }
    }

    unsafe fn port_in(port: i8) -> i16 { input(port) }
    unsafe fn port_out(port: i8, value: i8) { output(port, value);}
}
