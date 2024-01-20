BOOT=src/boot_loader
KERNEL=src/os_code
BUILD=build
SRC_DIR=src/os_code/src

CC=nasm

KERNEL_OBJ_DIR := src/os_code/target/i686-unknown-bare_metal/release/deps

KERNEL_INCLUDES := $(shell ls -R src/os_code | egrep ^'[^.]*\.o$^' | egrep -v ^'*core*|*os_code*|*\.[^o]^')

KERNEL_MAIN := $(KERNEL_OBJ_DIR)/$(shell ls -R src/os_code | egrep ^'os_code[^.]*\.o$^' | egrep -v ^'*\.[^o]^')
KERNEL_FULL_INCLUDES := $(addprefix $(KERNEL_OBJ_DIR)/, $(KERNEL_INCLUDES))

SRC_FILES := $(addprefix $(SRC_DIR)/, $(shell ls -R $(SRC_DIR) | egrep ^'[^.]*\.rs$^' | egrep -v ^'*\.[^^(rs^)]^'))

LINKER_SCRIPT := src/link.ls

all: os.img

kernel: $(wildcard $(SRC_DIR)/*.rs)
	@echo $(SRC_FILES)
	cd $(KERNEL) && cargo build --release

$(BUILD)/boot_sect.bin: $(BOOT)/boot_sect.asm
	$(CC) $(BOOT)/boot_sect.asm -f bin -I$(BOOT) -o $@

$(BUILD)/kernel_entry.o: $(BOOT)/kernel_entry.asm
	$(CC) $(BOOT)/kernel_entry.asm -f elf -I$(BOOT) -o $@ 

os.img: $(BUILD)/boot_sect.bin $(BUILD)/kernel.bin
	cat $(BUILD)/boot_sect.bin $(BUILD)/kernel.bin > $@ 

$(BUILD)/kernel.bin: $(BUILD)/kernel_entry.o kernel
	@echo "Start linking:"
	ld -m elf_i386 -o $@ -T $(LINKER_SCRIPT) -A i386 $< '$(KERNEL_MAIN)' --oformat binary

clean:
	rm -rf $(BUILD)/*
	cd $(KERNEL) && cargo clean

dev: os.img
	bochs -q
