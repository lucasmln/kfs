use core::mem::size_of;
use core::ffi::c_void;

use crate::{print, println};

const IDT_ENTRY_AMOUT: usize = 256;

extern "C" {
    fn test_function();
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

    fn load_idt(idt_table: &mut IdtPtr);
}

pub struct Regs
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

#[repr(C, packed)]
pub struct IdtEntry {
    isr_low: u16,       // The lower 16 bits of the ISR's address
    kernel_cs: u16,     // The GDT segment selector that the CPU will load into CS before calling the ISR
    reserved: u8,      // Set to zero
    attributes: u8,     // Type and attributes; see the IDT page
    isr_high: u16       // The higher 16 bits of the ISR's address
}

#[repr(C, packed)]
pub struct IdtPtr {
    limit: u16,
    base: u32
}

pub struct IdtTable {
    pub idt: &'static mut [IdtEntry; IDT_ENTRY_AMOUT]
}

impl Default for IdtTable {
    fn default() -> Self {
        Self { idt: unsafe { &mut *(0x0 as *mut [IdtEntry; IDT_ENTRY_AMOUT]) } }
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

pub fn set_idt_descriptor(desc: &mut IdtEntry, isr: *const c_void, flags: u8) {
    desc.isr_low = ((isr as u32) & 0xffff) as u16;
    desc.kernel_cs = 0x08;      // this value can be whatever offset your kernel code selector is in your GDT
    desc.attributes = flags;
    desc.isr_high = ((isr as u32) >> 16) as u16;
    desc.reserved = 0;
    return;
}


use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
    static ref IDT: Mutex<IdtTable> = Mutex::new(IdtTable::default());
}

pub fn idt_init() {
    let mut idt_ptr: IdtPtr = IdtPtr::default();
    
    unsafe {
        set_idt_descriptor(&mut IDT.lock().idt[0x00], isr_stub_0 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x01], isr_stub_1 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x02], isr_stub_2 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x03], isr_stub_3 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x04], isr_stub_4 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x05], isr_stub_5 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x06], isr_stub_6 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x07], isr_stub_7 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x08], isr_stub_8 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x09], isr_stub_9 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x0a], isr_stub_10 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x0b], isr_stub_11 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x0c], isr_stub_12 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x0d], isr_stub_13 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x0e], isr_stub_14 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x0f], isr_stub_15 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x10], isr_stub_16 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x11], isr_stub_17 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x12], isr_stub_18 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x13], isr_stub_19 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x14], isr_stub_20 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x15], isr_stub_21 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x16], isr_stub_22 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x17], isr_stub_23 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x18], isr_stub_24 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x19], isr_stub_25 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x1a], isr_stub_26 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x1b], isr_stub_27 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x1c], isr_stub_28 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x1d], isr_stub_29 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x1e], isr_stub_30 as _, 0x8e);
        set_idt_descriptor(&mut IDT.lock().idt[0x1f], isr_stub_31 as _, 0x8e);
        
        idt_ptr.limit = ((size_of::<IdtEntry>() * IDT_ENTRY_AMOUT) - (1 as usize)) as u16;
        idt_ptr.base = &IDT.lock().idt[0] as *const _ as u32;
        load_idt(&mut idt_ptr);
    }

}

#[no_mangle]
extern "C" fn exception_handler() {
    // if reg.int_no < 32 {

        // println!("Exception handler from isr a:{}", reg.int_no);
        unsafe {
            core::arch::asm!("cli");
            //core::arch::asm!("hlt");
        }
}
