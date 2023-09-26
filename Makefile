CP := cp
RM := rm -rf
MKDIR := mkdir -pv

BIN = kernel
CFG = grub.cfg
ISO_PATH := iso
BOOT_PATH := $(ISO_PATH)/boot
GRUB_PATH := $(BOOT_PATH)/grub

DOCKER_IMG_NAME = "kfs:1"
KERNEL_NAME = "kernel.iso"

.PHONY: all
all: bootloader kernel linker iso
	@echo Make has completed.

bootloader: boot.asm
	nasm -f elf32 boot.asm -o boot.o

kernel: kernel.c
	rustc --target=i686-unknown-linux-gnu --emit=obj -C panic=abort kernel.rs -o kernel.o
	#gcc -m32 -c kernel.c -o kernel.o

linker: linker.ld boot.o kernel.o
	ld -m elf_i386 -T linker.ld -o kernel boot.o kernel.o

iso: kernel
	$(MKDIR) $(GRUB_PATH)
	$(CP) $(BIN) $(BOOT_PATH)
	$(CP) $(CFG) $(GRUB_PATH)
	grub-file --is-x86-multiboot $(BOOT_PATH)/$(BIN)
	grub-mkrescue -o $(KERNEL_NAME) $(ISO_PATH)

docker-compile: docker-build
	docker run -ti -v .:/app --rm $(DOCKER_IMG_NAME)

docker-build:
	docker build -t $(DOCKER_IMG_NAME) .

run:
	qemu-system-i386 -cdrom $(KERNEL_NAME)

clean:
		$(RM) *.o $(BIN) *iso

.PHONY: clean
