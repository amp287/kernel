
all: build 

build: 
	cargo build
	cargo objcopy --target aarch64-unknown-none.json -- -O binary kernel8.img

test:
	cargo xtest -- 30

objdump:
	cargo objdump --target aarch64-unknown-none.json -- --disassemble --print-imm-hex

qemu:
	qemu-system-aarch64 -machine raspi3 -nographic -kernel kernel8.img

qemu-debug:
	qemu-system-aarch64 -D qemu_debug.log -d cpu_reset,guest_errors,int,mmu,page -machine raspi3 -nographic -kernel kernel8.img

qemu-gdb:
	qemu-system-aarch64 -machine raspi3 -nographic -gdb tcp::3333 -S -kernel kernel8.img

qemu-gdb-test:
	qemu-system-aarch64 -machine raspi3 -nographic -gdb tcp::3333 -S -kernel test.img

qemu-asm:
	qemu-system-aarch64 -machine raspi3 -nographic -kernel kernel8.img -d in_asm 

gdb: 
	~/dev_tools/aarch64-none-elf-gdb/bin/aarch64-none-elf-gdb target/aarch64-unknown-none/debug/kernel
	# use command: target remote :3333 to debug (gdb)
	
lldb: lldb target/aarch64-unknown-none/debug/kernel
	# use command: gdb-remote localhost:3333 (lldb)

gdb-test:
	~/dev_tools/aarch64-none-elf-gdb/bin/aarch64-none-elf-gdb target/aarch64-unknown-none/debug/mmu_test-459c245701d620ea

clean: 
	rm -rf target