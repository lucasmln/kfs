fn main() {
	println!("cargo:rerun-if-changed=src/gdt.s");
	cc::Build::new()
		.flag("-nostdlib")
		.flag("-ffreestanding")
		.flag("-fno-stack-protector")
		.flag("-mno-red-zone")
		.flag("-Wall")
		.flag("-m32")
		.flag("-Wextra")
		.file("src/gdt.s")
		.compile("gdt-lib");
}
