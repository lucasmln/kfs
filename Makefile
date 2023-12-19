CP := cp
RM := rm -rf
MKDIR := mkdir -pv
DIR=$(PWD)

KERNEL = kernel
KERNEL_PATH = target/i386-unknown-none/debug/$(KERNEL)
KERNEL_RELEASE_PATH = target/i386-unknown-none/release/$(KERNEL)
CFG = build/grub.cfg
ISO_PATH := iso
BOOT_PATH := $(ISO_PATH)/boot
GRUB_PATH := $(BOOT_PATH)/grub

DOCKER_IMG_NAME = "kfs:1"
ISO_NAME = kernel.iso

.PHONY: all

all: release

dev: kernel iso

release: kernel_release iso

kernel:
	cargo build
	cp $(KERNEL_PATH) .

kernel_release:
	cargo build --release
	cp $(KERNEL_RELEASE_PATH) .

iso: kernel
	$(MKDIR) $(GRUB_PATH)
	$(CP) $(KERNEL) $(BOOT_PATH)
	$(CP) $(CFG) $(GRUB_PATH)
	grub-file --is-x86-multiboot $(BOOT_PATH)/$(KERNEL)
	grub-mkrescue -o $(ISO_NAME) $(ISO_PATH)

docker-compile: docker-build
	docker run -ti -v $(DIR):/app --rm $(DOCKER_IMG_NAME)

docker-build:
	docker build -t $(DOCKER_IMG_NAME) .

run:
	qemu-system-i386 -cdrom $(ISO_NAME)

clean:
	$(RM) *.o *iso $(OBJ)

fclean: clean
	$(RM) $(ISO_NAME) $(KERNEL)
