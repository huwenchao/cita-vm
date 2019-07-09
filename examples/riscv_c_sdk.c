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
  size_t sz;
  env_load(&buffer_save_k[0], strlen(buffer_save_k), &buffer_load_v[0], 20, &sz);
  const char* s = buffer_load_v;
  if ((strcmp("Test: save_v", s) == 0) && (sz == 12)) {
    env_debug("Test[v]: load");
  } else {
    env_debug("Test[x]: load");
  }

  env_debug("Testing: address");
  uint8_t addr[20];
  env_address(&addr[0]);
  if (addr[19] == 0x01) {
      env_debug("Test[v]: address");
  } else {
      env_debug("Test[x]: address");
  }

  env_debug("Testing: balance");
  uint8_t account1[20] = {
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
  };
  uint8_t v[32];
  env_balance(&account1[0], &v[0]);
  if (v[31] == 10) {
    env_debug("Test[v]: balance");
  } else {
    env_debug("Test[x]: balance");
  }

  env_debug("Testing: origin");
  uint8_t origin[20];
  env_origin(&origin[0]);
  if (origin[19] == 0x02) {
      env_debug("Test[v]: origin");
  } else {
      env_debug("Test[x]: origin");
  }

  return 0;
}
