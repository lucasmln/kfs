use core::mem::size_of;

const GDT_ENTRY_AMOUNT: usize = 7;

extern "C" {
    fn gdt_flush(gp: &mut GdtPtr);
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct GdtEntry {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub access: u8,
    pub granularity: u8,
    pub base_high: u8
}

pub struct GdtTable {
    pub gdt: &'static mut [GdtEntry; GDT_ENTRY_AMOUNT]
}
impl Default for GdtTable {
    fn default() -> Self {
        Self { gdt: unsafe { &mut *(0x800 as *mut [GdtEntry; GDT_ENTRY_AMOUNT]) } }
    }
}

impl Default for GdtEntry {
    fn default() -> Self {
        Self { limit_low: 0, base_low: 0, base_middle: 0, access: 0, granularity: 0, base_high: 0 }
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct GdtPtr {
    pub limit: u16,
    pub base: u32
}

impl Default for GdtPtr {
    fn default() -> Self {
        Self { limit: 0, base: 0 }
    }
}

pub fn init_gdt(entry: &mut GdtEntry, base: u32, limit: u32, access: u8, granularity: u8) {
    entry.base_low = (base & 0xffff) as u16;
    entry.base_middle = ((base >> 16) & 0xff) as u8;
    entry.base_high = ((base >> 24) & 0xff) as u8;

    entry.limit_low = (limit & 0xffff) as u16;
    entry.granularity = ((limit >> 16) & 0xf) as u8;

    entry.granularity |= granularity & 0xf0;
    entry.access = access;
    return;
}

use lazy_static::lazy_static;
use spin::Mutex;

use crate::{println, print};

lazy_static! {
    static ref GDT: Mutex<GdtTable> = Mutex::new(GdtTable::default());
}

pub fn gdt_install()
{
    let mut gp: GdtPtr = GdtPtr::default();

    /* Setup the GDT pointer and limit */
    gp.limit = ((size_of::<GdtEntry>() * GDT_ENTRY_AMOUNT) - (1 as usize)) as u16;
    gp.base = &GDT.lock().gdt[0] as *const _ as u32;

    /* https://wiki.osdev.org/Global_Descriptor_Table#Segment_Descriptor */
    /* Our NULL descriptor */
    init_gdt(&mut GDT.lock().gdt[0], 0, 0, 0, 0);
    /* kernel code */
    init_gdt(&mut GDT.lock().gdt[1], 0, 0xffffffff, 0x9B, 0xcf);
    /* kernel data */
    init_gdt(&mut GDT.lock().gdt[2], 0, 0xffffffff, 0x93, 0xcf);
    /* kernel stack */
    init_gdt(&mut GDT.lock().gdt[3], 0, 0xffffffff, 0x97, 0xcf);

    /* user code */
    init_gdt(&mut GDT.lock().gdt[4], 0, 0xffffffff, 0xfb, 0xcf);
    /* user data */
    init_gdt(&mut GDT.lock().gdt[5], 0, 0xffffffff, 0xf3, 0xcf);
    /* user stack */
    init_gdt(&mut GDT.lock().gdt[6], 0, 0xffffffff, 0xf7, 0xcf);

    /* Flush out the old GDT and install the new changes */
    unsafe {
        gdt_flush(&mut gp);
    }
}

#[allow(dead_code)]
pub fn print_gdt() {
    let mut dtr = GdtPtr::default();

    unsafe {
        core::arch::asm!("sgdt [{0}]", in(reg) &mut dtr);
        println!("{:?}", dtr);
        let gdt_entry_amout = (dtr.limit + 1) / 8;
        for i in 0..gdt_entry_amout {
            let gdt = &mut *((dtr.base + (i * size_of::<GdtEntry>() as u16) as u32) as *mut GdtEntry);
            println!("{:x?}", gdt);
        }
    }
}
