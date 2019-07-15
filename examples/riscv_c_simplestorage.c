#include <string.h>

#include "pvm.h"
#include "pvm_extend.h"

int main(int argc, char* argv[]) {
  if (argc == 1) {
    return 0;
  }

  if (strcmp(argv[1], "get") == 0) {
    if (argc != 3) {
      return 1;
    }
    uint8_t *k = (uint8_t *)argv[2];
    uint8_t v[8];
    pvm_load(&k[0], strlen(k), &v[0], 8, NULL);
    uint64_t r = pvm_decode_u64(&v[0]);
    pvm_ret_u64(r);
    return 0;
  }

  if (strcmp(argv[1], "set") == 0) {
    if (argc != 4) {
      return 1;
    }
    uint8_t *k = (uint8_t *)argv[2];
    uint8_t v[8];
    pvm_encode_u64(&v[0], atoi(argv[3]));
    pvm_save(&k[0], strlen(k), &v[0], 8);
    return 0;
  }
  return 0;
}
