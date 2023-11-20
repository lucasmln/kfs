fn main() {
    cc::Build::new()
        .flag("-nostdlib")
		.flag("-ffreestanding")
		.flag("-fno-stack-protector")
		.flag("-mno-red-zone")
		.flag("-Wall")
		.flag("-Wextra")
        .file("src/gdt.s")
        .compile("gdt-lib");
}