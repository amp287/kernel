#!/bin/bash
# Script used to run tests.
# $1 is the cargo executable
# $2 is the time in seconds before the test is killed and deemed a failure

if [ -z "$2" ]; then 
    echo "Timeout value is required!"
    exit 125
fi

cargo objcopy --target aarch64-unknown-none.json $1 -- -O binary $1.img

rm -f $1.objdump

cargo objdump --target aarch64-unknown-none.json -- -disassemble -print-imm-hex $1 >> $1.objdump

if [[ "$OSTYPE" == "linux-gnu" ]]; then
    TIMEOUT_CMD="timeout"
elif [[ "$OSTYPE" == "darwin"* ]]; then 
    TIMEOUT_CMD="gtimeout"
else 
    echo "Unsupported OS: $OSTYPE"
    exit 200
fi

echo "Testing: $1.img"

$TIMEOUT_CMD $2 qemu-system-aarch64 -machine raspi3 -semihosting -nographic -kernel $1.img

exit $?
