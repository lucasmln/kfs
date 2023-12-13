#[macro_export]
macro_rules! cli {
    () => { unsafe { core::arch::asm!("cli") }; };
}

#[macro_export]
macro_rules! sti {
    () => { unsafe { core::arch::asm!("sti") }; };
}

#[macro_export]
macro_rules! hlt {
    () => { unsafe { core::arch::asm!("hlt") }; };
}

pub fn outb(port: u16, value: u8) {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value);
    }
}

pub fn inb(port: u16) -> u8 {
    let mut value: u8;
    unsafe {
        core::arch::asm!("in al, dx", out("al") value, in("dx") port);
    }
    return value;
}

