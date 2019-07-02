from common import call

call('la fs put ./src/riscv/c/sdk.h')
call('la fs put ./examples/riscv_c_sdk_main.c')
call('la fs exec "/usr/local/riscv/bin/riscv64-unknown-elf-gcc -o riscv_c_sdk_main riscv_c_sdk_main.c"')
call('la fs get riscv_c_sdk_main /tmp/cita-vm/riscv_c_sdk_main')
