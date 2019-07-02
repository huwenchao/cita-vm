#include <string.h>

#include "sdk.h"

int main(int argc, char* argv[]) {
  env_debug("Testing: debug");
  env_debug("Test[v]: debug");

  env_debug("Testing: ret");
  uint8_t *buffer_ret = (uint8_t *)"Test: ret";
  env_ret(&buffer_ret[0], strlen(buffer_ret));
  env_debug("Test[v]: ret");

  env_debug("Testing: save");
  uint8_t *buffer_save_k = (uint8_t *)"Test: save_k";
  uint8_t *buffer_save_v = (uint8_t *)"Test: save_v";
  env_save(&buffer_save_k[0], strlen(buffer_save_k), &buffer_save_v[0], strlen(buffer_save_v));
  env_debug("Test[v]: save");

  env_debug("Testing: load");
  uint8_t buffer_load_v[20];
  env_load(&buffer_save_k[0], strlen(buffer_save_k), &buffer_load_v[0], 20);
  const char* s = buffer_load_v;
  if (strcmp("Test: save_v", s) == 0) {
    env_debug("Test[v]: load");
  } else {
    env_debug("Test[x]: load");
  }

  return 0;
}
