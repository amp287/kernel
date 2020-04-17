#!/bin/bash
# Script used to run tests.
# $1 is the cargo executable
# $2 is the time in seconds before the test is killed and deemed a failure

BINUTILS_LOCATION=/usr/local/opt/binutils/bin

if [ -z "$2" ]; then 
    echo "Timeout value is required!"
    exit 125
fi

rm -f $1.img

$BINUTILS_LOCATION/objcopy $1 --output-target=binary $1.img

rm -f $1.objdump

$BINUTILS_LOCATION/objdump --disassemble --disassembler-options=hex $1 >> $1.objdump

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

if [[ $? -eq 124 ]]; then
    echo "Test timed out"
elif [[ $? -eq 125 ]]; then
    echo "Timeout failed!"
elif [[ $? -eq 126 ]]; then
    echo "qemu-system-aarch64 call failed!"
fi 

exit $?

