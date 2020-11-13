#!/bin/bash
# Script used to run tests.
# $1 is the cargo executable
# $2 is the time in seconds before the test is killed and deemed a failure

BINUTILS_LOCATION=/usr/local/opt/binutils/bin

if [ -z "$2" ]; then 
    echo "Timeout value is required!"
    exit 125
fi

if [ ! -z "$3" ]; then
    set -x
fi

rm -f $1.img

$BINUTILS_LOCATION/objcopy $1 --output-target=binary $1.img

rm -f $1.objdump

$BINUTILS_LOCATION/objdump --disassemble $1 >> $1.objdump

if [[ "$OSTYPE" == "linux-gnu" ]]; then
    TIMEOUT_CMD="timeout"
elif [[ "$OSTYPE" == "darwin"* ]]; then 
    TIMEOUT_CMD="gtimeout"
else 
    echo "Unsupported OS: $OSTYPE"
    exit 200
fi

printf "\nTesting: \n\t$1.img\n\n"

$TIMEOUT_CMD -k 5 $2 qemu-system-aarch64 -machine raspi3 -semihosting -nographic -kernel $1.img

ret=$?

if [[ $ret -eq 124 ]]; then
    echo "[FAIL] Test timed out"
    ret=1
elif [[ $ret -eq 137 ]]; then
    echo "[FAIL] Test killed"
    ret=1
elif [[ $ret -eq 125 ]]; then
    echo "[FAIL] Timeout failed!"
    ret=1
elif [[ $ret -eq 126 ]]; then
    echo "[FAIL] qemu-system-aarch64 call failed!"
    ret=1
fi 

printf "\n\n"

exit $ret

