const IDT_ENTRY_AMOUT: usize = 256;

#[repr(C, packed)]
pub struct IdtEntry {
    isr_low: u16,       // The lower 16 bits of the ISR's address
    kernel_cs: u16,     // The GDT segment selector that the CPU will load into CS before calling the ISR
    reserverd: u8,      // Set to zero
    attributes: u8,     // Type and attributes; see the IDT page
    isr_high: u16       // The higher 16 bits of the ISR's address
}

#[repr(C, packed)]
pub struct IdtPtr {
    limit: u16,
    base: u32
}

pub struct IdtTable {
    pub idt: &'static mut [IdtEntry: IDT_ENTRY_AMOUT]
}

impl Default for IdtEntry {
    fn default() -> Self {
        Self { isr_low: 0, kernel_cs: 0, reserverd: 0, attributes: 0, isr_high: 0 }
    }
}

pub fn idt_set_descriptor(desc: &mut IdtEntry, vector: u8, isr: u32, flags: u8) {
    desc.isr_low = isr & 0xffff;
    desc.kernel_cs = 0x08;      // this value can be whatever offset your kernel code selector is in your GDT
    desc.attributes = flags;
    desc.isr_high = isr >> 16;
    desc.reserved = 0;
    return;
}


pub fn idt_init() {

}

pub fn exception_handler() {
    core::arch::asm!("cli")
    core::arch::asm!("hlt")
}