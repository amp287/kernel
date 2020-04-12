#!/bin/bash

cargo objcopy --target aarch64-unknown-none.json $1 -- -O binary test.img

rm test.objdump

cargo objdump --target aarch64-unknown-none.json -- -disassemble -print-imm-hex $1 >> test.objdump

qemu-system-aarch64 -machine raspi3 -semihosting -nographic -kernel test.img

exit $?

