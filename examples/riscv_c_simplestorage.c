#include <string.h>

#include "sdk.h"

int main(int argc, char* argv[]) {
  if (argc == 1) {
    return 0;
  }
  if (strcmp(argv[1], "get") == 0) {
    if (argc != 3) {
      return 1;
    }
    return 0;
  }

  if (strcmp(argv[1], "set") == 0) {
    if (argc != 4) {
      return 1;
    }

    uint8_t *k = (uint8_t *)argv[2];
    uint64_t v = atoi(argv[3]);


    env_save(&buffer_save_k[0], strlen(buffer_save_k), &buffer_save_v[0], strlen(buffer_save_v));

    return 0;
  }
  return 0;
}
