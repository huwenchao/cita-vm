#include <ctype.h>
#include <string.h>

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

int pvm_hex2bin(char *s, char *buf)
{
    int i,n = 0;
    for(i = 0; s[i]; i += 2) {
        int c = tolower(s[i]);
        if(c >= 'a' && c <= 'f')
            buf[n] = c - 'a' + 10;
        else buf[n] = c - '0';
        if(s[i + 1] >= 'a' && s[i + 1] <= 'f')
            buf[n] = (buf[n] << 4) | (s[i + 1] - 'a' + 10);
        else buf[n] = (buf[n] << 4) | (s[i + 1] - '0');
        ++n;
    }
    return n;
}

int pvm_bin2hex(uint8_t *bin, uint8_t len, char* out)
{
	uint8_t  i;
	for (i=0; i<len; i++) {
		out[i*2]   = "0123456789abcdef"[bin[i] >> 4];
		out[i*2+1] = "0123456789abcdef"[bin[i] & 0x0F];
	}
	out[len*2] = '\0';
    return 0;
}

int pvm_ret_str(const char *s)
{
    uint8_t *buffer = (uint8_t *)s;
    return pvm_ret(&buffer[0], strlen(buffer));
}

int pvm_ret_u64(uint64_t n)
{
    uint8_t list[8];
    pvm_encode_u64(&list[0], n);
    return pvm_ret(&list[0], 8);
}
