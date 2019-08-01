#include <ctype.h>
#include <string.h>

#include "pvm.h"

const char *HEX_CHARS = "0123456789abcdef";

uint8_t hex2int(char c)
{
  if (c >= 'a' && c <= 'f')
  {
    return c - 'a';
  }
  else if (c >= 'A' && c <= 'F')
  {
    return c - 'A';
  }
  else
  {
    return c - '0';
  }
}

int pvm_hex2bin(const char *hex, int hex_length, uint8_t *bin)
{
  for (int i = 0; i < hex_length; i += 2)
  {
    bin[i / 2] = hex2int(hex[i]) >> 4 + hex2int(hex[i + 1]);
  }
  return 0;
}

int pvm_bin2hex(uint8_t *bin, int bin_len, char *hex)
{
  for (int i = 0; i < bin_len; i++)
  {
    hex[i * 2] = HEX_CHARS[bin[i] >> 4];
    hex[i * 2 + 1] = HEX_CHARS[bin[i] & 0x0F];
  }
  hex[bin_len * 2] = '\0';
  return 0;
}

int pvm_dump_memory(uintptr_t addr, int length)
{
  char *hex = (char *)malloc(length * 2 + 1);
  pvm_bin2hex((u_int8_t *)addr, length, hex);
  pvm_debug(hex);
  return 0;
}

int pvm_ret_str(const char *s)
{
  uint8_t *buffer = (uint8_t *)s;
  return pvm_ret(&buffer[0], strlen(buffer));
}

int pvm_ret_u64(uint64_t n)
{
  return pvm_ret((uint8_t *)&n, 8);
}
