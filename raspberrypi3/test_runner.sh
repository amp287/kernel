#!/bin/bash

cargo objcopy --target aarch64-unknown-none.json $1 -- -O binary test.img

qemu-system-aarch64 -machine raspi3 -semihosting -nographic -kernel test.img

exit $?

