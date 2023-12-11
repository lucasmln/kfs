fn main() {
	println!("cargo:rerun-if-changed=src/gdt.s");
	println!("cargo:rerun-if-changed=src/idt/idt.s");
	cc::Build::new()
		.flag("-nostdlib")
		.flag("-ffreestanding")
		.flag("-fno-stack-protector")
		.flag("-mno-red-zone")
		.flag("-Wall")
		.flag("-m32")
		.flag("-Wextra")
		.file("src/gdt/gdt.s")
		.file("src/idt/idt.s")
		.compile("gdt-lib");
}
