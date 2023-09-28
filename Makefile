CP := cp
RM := rm -rf
MKDIR := mkdir -pv

RUSTC = rustc
KERNEL = kernel
KERNEL_LIB = libkernel.a
KERNEL_LIB_PATH = target/x86/debug/$(KERNEL_LIB)
CFG = build/grub.cfg
LINKER_FILE = build/linker.ld
ISO_PATH := iso
BOOT_PATH := $(ISO_PATH)/boot
GRUB_PATH := $(BOOT_PATH)/grub

DOCKER_IMG_NAME = "kfs:1"
ISO_NAME = kernel.iso

SRCS = $(addprefix src/, $(SRC))
SRC = main.rs
OBJ = $(SRCS:.rs=.o)

.PHONY: all
all: bootloader kernel linker iso
	@echo Make has completed.

bootloader:
	nasm -f elf32 boot.asm -o boot.o

kernel:
	cargo build

linker: bootloader kernel
	ld -m elf_i386 -T $(LINKER_FILE) -o $(KERNEL) boot.o $(KERNEL_LIB_PATH)

iso: kernel
	$(MKDIR) $(GRUB_PATH)
	$(CP) $(KERNEL) $(BOOT_PATH)
	$(CP) $(CFG) $(GRUB_PATH)
	grub-file --is-x86-multiboot $(BOOT_PATH)/$(KERNEL)
	grub-mkrescue -o $(ISO_NAME) $(ISO_PATH)

docker-compile: docker-build
	docker run -ti -v .:/app --rm $(DOCKER_IMG_NAME)

docker-build:
	docker build -t $(DOCKER_IMG_NAME) .

run:
	qemu-system-i386 -cdrom $(ISO_NAME)

run-kernel:
	qemu-system-i386 -kernel $(KERNEL)

clean:
	$(RM) *.o *iso $(OBJ)

fclean: clean
	$(RM) $(ISO_NAME) $(KERNEL)

.PHONY: clean
