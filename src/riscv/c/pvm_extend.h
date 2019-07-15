#include "pvm.h"

void pvm_encode_u64(uint8_t* buffer, uint64_t n)
{
    buffer[0] = (n >> 56) & 0xFF;
    buffer[1] = (n >> 48) & 0xFF;
    buffer[2] = (n >> 40) & 0xFF;
    buffer[3] = (n >> 32) & 0xFF;
    buffer[4] = (n >> 24) & 0xFF;
    buffer[5] = (n >> 16) & 0xFF;
    buffer[6] = (n >> 8) & 0xFF;
    buffer[7] = (n >> 0) & 0xFF;
}

uint64_t pvm_decode_u64(uint8_t* buffer)
{
    uint64_t r = (uint64_t)buffer[0] << 56 |
        (uint64_t)buffer[1] << 48 |
        (uint64_t)buffer[2] << 40 |
        (uint64_t)buffer[3] << 32 |
        (uint64_t)buffer[4] << 24 |
        (uint64_t)buffer[5] << 16 |
        (uint64_t)buffer[6] << 8 |
        (uint64_t)buffer[7];
    return r;
}

int pvm_ret_u64(uint64_t n)
{
    uint8_t list[8];
    pvm_encode_u64(&list[0], n);
    return pvm_ret(&list[0], 8);
}
