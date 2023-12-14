use core::mem::size_of;
use core::ffi::c_void;

use crate::{print, println, sti, cli};
use crate::asm;
use crate::keyboard::handle_keypress;

const IDT_ENTRY_AMOUT: usize = 256;

extern "C" {
    fn isr_stub_0();
    fn isr_stub_1();
    fn isr_stub_2();
    fn isr_stub_3();
    fn isr_stub_4();
    fn isr_stub_5();
    fn isr_stub_6();
    fn isr_stub_7();
    fn isr_stub_8();
    fn isr_stub_9();
    fn isr_stub_10();
    fn isr_stub_11();
    fn isr_stub_12();
    fn isr_stub_13();
    fn isr_stub_14();
    fn isr_stub_15();
    fn isr_stub_16();
    fn isr_stub_17();
    fn isr_stub_18();
    fn isr_stub_19();
    fn isr_stub_20();
    fn isr_stub_21();
    fn isr_stub_22();
    fn isr_stub_23();
    fn isr_stub_24();
    fn isr_stub_25();
    fn isr_stub_26();
    fn isr_stub_27();
    fn isr_stub_28();
    fn isr_stub_29();
    fn isr_stub_30();
    fn isr_stub_31();

    fn irq_0();
    fn irq_1();
    fn irq_2();
    fn irq_3();
    fn irq_4();
    fn irq_5();
    fn irq_6();
    fn irq_7();
    fn irq_8();
    fn irq_9();
    fn irq_10();
    fn irq_11();
    fn irq_12();
    fn irq_13();
    fn irq_14();
    fn irq_15();

    fn load_idt(idt_table: &mut IdtPtr);
}

#[derive(Debug)]
#[repr(C)]
struct Regs
{
    gs: u32,
    fs: u32,
    es: u32,
    ds: u32,
    edi: u32,
    esi: u32,
    ebp: u32,
    esp: u32,
    ebx: u32,
    edx: u32,
    ecx: u32,
    eax: u32,
    int_no: u32,
    err_code: u32,
    eip: u32,
    cs: u32,
    eflags: u32,
    useresp: u32,
    ss: u32
}

#[derive(Debug)]
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    isr_low: u16,       // The lower 16 bits of the ISR's address
    kernel_cs: u16,     // The GDT segment selector that the CPU will load into CS before calling the ISR
    reserved: u8,      // Set to zero
    attributes: u8,     // Type and attributes; see the IDT page
    isr_high: u16       // The higher 16 bits of the ISR's address
}

#[derive(Debug)]
#[repr(C, packed)]
struct IdtPtr {
    limit: u16,
    base: u32
}

struct IdtTable {
    pub _idt: &'static mut [IdtEntry; IDT_ENTRY_AMOUT]
}

impl Default for IdtTable {
    fn default() -> Self {
        #[allow(deref_nullptr)]
        Self { _idt: unsafe { &mut *(0x0 as *mut [IdtEntry; IDT_ENTRY_AMOUT]) } }
    }
}

impl Default for IdtEntry {
    fn default() -> Self {
        Self { isr_low: 0, kernel_cs: 0, reserved: 0, attributes: 0, isr_high: 0 }
    }
}

impl Default for IdtPtr {
    fn default() -> Self {
        Self { limit: 0, base: 0 }
    }
}

fn set_idt_descriptor(desc: &mut IdtEntry, isr: *const c_void, flags: u8) {
    desc.isr_low = ((isr as u32) & 0xffff) as u16;
    desc.kernel_cs = 0x08;      // this value can be whatever offset your kernel code selector is in your GDT
    desc.attributes = flags;
    desc.isr_high = ((isr as u32) >> 16) as u16;
    desc.reserved = 0;
    return;
}

fn irq_remap()
{
    asm::outb(0x20, 0x11);
    asm::outb(0xA0, 0x11);
    asm::outb(0x21, 0x20);
    asm::outb(0xA1, 0x28);
    asm::outb(0x21, 0x04);
    asm::outb(0xA1, 0x02);
    asm::outb(0x21, 0x01);
    asm::outb(0xA1, 0x01);
    asm::outb(0x21, 0x0);
    asm::outb(0xA1, 0x0);
}

use once_cell::unsync::Lazy;
use spin::Mutex;

static _IDT: Mutex<Lazy<IdtTable>> = Mutex::new(Lazy::new(|| IdtTable::default()));

pub fn reboot() {
    let mut idt_ptr: IdtPtr = IdtPtr::default();

    unsafe {
        load_idt(&mut idt_ptr);
    }
}

pub fn init() {
    let mut idt_ptr: IdtPtr = IdtPtr::default();
    let mut fake_idt_entries = [IdtEntry::default(); IDT_ENTRY_AMOUT];

    for i in 0..IDT_ENTRY_AMOUT {
        set_idt_descriptor(&mut fake_idt_entries[i], isr_stub_0 as _, 0x8e);
    }

    unsafe {
        set_idt_descriptor(&mut fake_idt_entries[0x00], isr_stub_0 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x01], isr_stub_1 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x02], isr_stub_2 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x03], isr_stub_3 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x04], isr_stub_4 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x05], isr_stub_5 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x06], isr_stub_6 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x07], isr_stub_7 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x08], isr_stub_8 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x09], isr_stub_9 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x0a], isr_stub_10 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x0b], isr_stub_11 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x0c], isr_stub_12 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x0d], isr_stub_13 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x0e], isr_stub_14 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x0f], isr_stub_15 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x10], isr_stub_16 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x11], isr_stub_17 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x12], isr_stub_18 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x13], isr_stub_19 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x14], isr_stub_20 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x15], isr_stub_21 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x16], isr_stub_22 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x17], isr_stub_23 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x18], isr_stub_24 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x19], isr_stub_25 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x1a], isr_stub_26 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x1b], isr_stub_27 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x1c], isr_stub_28 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x1d], isr_stub_29 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x1e], isr_stub_30 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x1f], isr_stub_31 as _, 0x8e);

        irq_remap();
        set_idt_descriptor(&mut fake_idt_entries[0x20], irq_0 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x21], irq_1 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x22], irq_2 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x23], irq_3 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x24], irq_4 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x25], irq_5 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x26], irq_6 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x27], irq_7 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x28], irq_8 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x29], irq_9 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x2a], irq_10 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x2b], irq_11 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x2c], irq_12 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x2d], irq_13 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x2e], irq_14 as _, 0x8e);
        set_idt_descriptor(&mut fake_idt_entries[0x2f], irq_15 as _, 0x8e);

        for i in 0..IDT_ENTRY_AMOUT {
            let entry = (i * size_of::<IdtEntry>()) as *mut IdtEntry;
            *entry = fake_idt_entries[i];
        }

        idt_ptr.limit = ((size_of::<IdtEntry>() * IDT_ENTRY_AMOUT) - (1 as usize)) as u16;
        idt_ptr.base = 0;
        load_idt(&mut idt_ptr);
    }

}

#[no_mangle]
extern "C" fn exception_handler(reg: Regs) {

    println!("Exception handler from ISR a:{:?}", reg);

    unsafe {
        core::arch::asm!("cli");
        core::arch::asm!("hlt");
    }
}

#[no_mangle]
extern "C" fn irq_handler(reg: Regs) {
    cli!();
    match reg.int_no {
        1 => { handle_keypress(); }
        12 => { asm::inb(0x60); }
        _ => { }
    }

    if reg.int_no >= 8 {
        asm::outb(0xA0, 0x20);
    }
    asm::outb(0x20, 0x20);
    sti!();
}

pub fn print_idt() {
    let mut dtr = IdtPtr::default();

    unsafe {
        core::arch::asm!("sidt [{0}]", in(reg) &mut dtr);
        let gdt_entry_amout = (dtr.limit + 1) / size_of::<IdtEntry>() as u16;
        for i in 0..gdt_entry_amout {
            let idt = &mut *((dtr.base + (i * size_of::<IdtEntry>() as u16) as u32) as *mut IdtEntry);
            println!("{:x?}", idt);
        }
        println!("{:?}", dtr);
    }
}
