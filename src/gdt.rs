use core::mem::size_of;
use core::arch::asm;

const GDT_BASE: u32 = 0x00000800;

extern "C" {
    fn gdt_flush(gp: &mut GdtPtr);
}

#[repr(C, packed)]
pub struct GdtEntry {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub access: u8,
    pub granularity: u8,
    pub base_high: u8
}

impl Default for GdtEntry {
    fn default() -> Self {
        Self { limit_low: 0, base_low: 0, base_middle: 0, access: 0, granularity: 0, base_high: 0 }
    }
}

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
    entry.granularity = ((limit << 16) & 0xf) as u8;

    entry.granularity |= granularity & 0xf0;
    entry.access = access;
    return;
}

// pub fn gdt_flush(gp: &mut GdtPtr) {
//     unsafe {
//         asm!(
//             "lgdt {0}",
//             "mov 0x10, ax",
//             "mov ax, ds",
//             "mov ax, es",
//             "mov ax, fs",
//             "mov ax, gs",
//             "ret",
//             in(reg) gp
//         );
//     }
// }

pub fn gdt_install(gp: &mut GdtPtr, gdt: &mut GdtEntry)
{
    /* Setup the GDT pointer and limit */
    gp.limit = ((size_of::<GdtEntry>() * 3) - (1 as usize)) as u16;
    gp.base = gdt as *const _ as u32;

    /* Our NULL descriptor */
    // init_gdt(0, 0, 0, 0, 0);

    /* The second entry is our Code Segment. The base address
    *  is 0, the limit is 4GBytes, it uses 4KByte granularity,
    *  uses 32-bit opcodes, and is a Code Segment descriptor.
    *  Please check the table above in the tutorial in order
    *  to see exactly what each value means */
    init_gdt(gdt, GDT_BASE, 0xFFFFFFFF, 0x9A, 0xCF);

    /* The third entry is our Data Segment. It's EXACTLY the
    *  same as our code segment, but the descriptor type in
    *  this entry's access byte says it's a Data Segment */
    // init_gdt(2, 0, 0xFFFFFFFF, 0x92, 0xCF);

    /* Flush out the old GDT and install the new changes! */
    unsafe {
        gdt_flush(gp);
    }
}
