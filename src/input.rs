use core::mem::MaybeUninit;

struct IDT_entry {
    offset_lowerbits: u16,
    selector: u16,
    zero: u8,
	type_attr: u8,
	offset_higherbits: u8
}

const IDT_SIZE:usize = 256;

static mut IDT:MaybeUninit<[IDT_entry;IDT_SIZE]> = MaybeUninit::uninit();

fn idt_init() {
    let kb_addr: u32;
    let idt_addr: u32;
    let idr_ptr: [u32; 2];
}