#include <stdlib.h>
#include "sdk.h"

void encode_u64(uint8_t* buffer, uint64_t n)
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

int return_u64(uint64_t n)
{
    uint8_t list[8];
    encode_u64(&list[0], n);
    return env_ret(&list[0], 8);
}


int fibonacci(int n)
{
  if (n == 0 || n == 1)
    return n;
  else
    return (fibonacci(n-1) + fibonacci(n-2));
}

int main(int argc, char* argv[]) {
    int n = atoi(argv[1]);
    int r = fibonacci(n);
    return return_u64(r);
}
