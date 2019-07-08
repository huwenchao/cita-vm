#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

static inline long
__internal_syscall(long n, long _a0, long _a1, long _a2, long _a3, long _a4, long _a5)
{
    register long a0 asm("a0") = _a0;
    register long a1 asm("a1") = _a1;
    register long a2 asm("a2") = _a2;
    register long a3 asm("a3") = _a3;
    register long a4 asm("a4") = _a4;
    register long a5 asm("a5") = _a5;
    register long syscall_id asm("a7") = n;
    asm volatile ("scall": "+r"(a0) : "r"(a1), "r"(a2), "r"(a3), "r"(a4), "r"(a5), "r"(syscall_id));
    return a0;
}

#define syscall(n, a, b, c, d, e, f) \
    __internal_syscall(n, (long)(a), (long)(b), (long)(c), (long)(d), (long)(e), (long)(f))


#define SYSCODE_DEBUG 2177
#define SYSCODE_RET 2180
#define SYSCODE_SAVE 2181
#define SYSCODE_LOAD 2182
#define SYSCODE_ADDRESS 2190
#define SYSCODE_BALANCE 2191


// Function env_debug accepts a string that contains the text to be written to stdout(It depends on the VM).
// Params:
//   format: same as the standard C function `printf()`
// Return:
//   code: 0(success)
// Example:
//   evn_debug("Hello World!");
int env_debug(const char* s)
{
  return syscall(SYSCODE_DEBUG, s, 0, 0, 0, 0, 0);
}

// Function ret returns any bytes to host, as the output of the current contract.
// Params:
//   data: a pointer to a buffer in VM memory space denoting where the data we are about to send.
//   size: size of the data buffer
// Return:
//   code: 0(success)
//
// Note: This syscall(s) only allowed to call once. If called it multiple times, the last call will replace the
// previous call.
int env_ret(uint8_t *data, size_t size)
{
    return syscall(SYSCODE_RET, data, size, 0, 0, 0, 0);
}

// Function env_save stores any bytes with it's keys into the global SRAM.
// Params:
//   k: a pointer to a buffer in VM memory space denoting where the key located at.
//   k_size: size of the k buffer.
//   v: a pointer to a buffer in VM memory space denoting where the value located at.
//   v_size: size of the v buffer.
// Return:
//   code: 0(success)
int env_save(uint8_t *k, size_t k_size, uint8_t *v, size_t v_size)
{
    return syscall(SYSCODE_SAVE, k, k_size, v, v_size, 0, 0);
}


// Function env_load loads bytes with given key from the global SRAM.
// Params:
//   k: a pointer to a buffer in VM memory space denoting where the key located at.
//   k_size: size of the k buffer.
//   v: a pointer to a buffer in VM memory space denoting where we would load the data.
//   v_size: size of the v buffer.
// Return:
//   code: 0(success), 1(key not found)
int env_load(uint8_t *k, size_t k_size, uint8_t *v, size_t v_size, size_t *r_size)
{
    return syscall(SYSCODE_LOAD, k, k_size, v, v_size, r_size, 0);
}

// Function env_address loads current address from context.
// Params:
//   addr: a pointer to a buffer in VM memory space denoting where the address located at.
// Return:
//   code: 0(success)
int env_address(uint8_t *addr)
{
    return syscall(SYSCODE_ADDRESS, addr, 0, 0, 0, 0, 0);
}

// Function env_balance loads balance of the specific address.
// Params:
//   addr: a pointer to a buffer in VM memory space denoting where the address located at.
//   v: a pointer to a 4 bytes buffer where the value located at.
// Return:
//   code: 0(success)
int env_balance(uint8_t *addr, uint8_t *v)
{
    return syscall(SYSCODE_BALANCE, addr, v, 0, 0, 0, 0);
}
