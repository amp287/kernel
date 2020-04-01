
all: build 

build: 
	cargo xbuild
	cargo objcopy --target aarch64-unknown-none.json target/aarch64-unknown-none/debug/kernel -- -O binary kernel8.img

objdump:
	cargo objdump --target aarch64-unknown-none.json -- -disassemble -print-imm-hex target/aarch64-unknown-none/debug/kernel

qemu:
	qemu-system-aarch64 -machine raspi3 -nographic -kernel kernel8.img

qemu-debug:
	qemu-system-aarch64 -D qemu_debug.log -d cpu_reset,guest_errors,int,mmu,page -machine raspi3 -nographic -kernel kernel8.img

qemu-gdb:
	qemu-system-aarch64 -machine raspi3 -nographic -gdb tcp::3333 -S -kernel kernel8.img

qemu-asm:
	qemu-system-aarch64 -machine raspi3 -nographic -kernel kernel8.img -d in_asm 

gdb: 
	~/dev_tools/aarch64-none-elf-gdb/bin/aarch64-none-elf-gdb target/aarch64-unknown-none/debug/kernel
	# use command: target remote :3333 to debug

clean: 
	rm -rf target