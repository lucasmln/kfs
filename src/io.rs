pub fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value);
    }
}
pub fn outw(port: u16, value: u16) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("ax") value);
    }
}

pub fn inb(port: u16) -> u8 {
    let mut value: u8;
    unsafe {
        core::arch::asm!("in al, dx", out("al") value, in("dx") port);
    }
    return value;
}
pub fn inw(port: u16) -> u16 {
    let mut value: u16;
    unsafe {
        core::arch::asm!("in al, dx", out("ax") value, in("dx") port);
    }
    return value;
}
