#include <string.h>

#include "pvm.h"
#include "pvm_extend.h"

// typedef struct clear_record {
//   uint8_t address[20];
//   uint8_t asset_id[32];
//   uint64_t amount;
// } clear_record;

int deposit(uint8_t *address, uint8_t *asset_id, uint64_t amount)
{
  // tranfer asset from user to contract
  uint8_t key[52];
  memcpy(key, address, 20);
  memcpy(key + 20, asset_id, 32);
  uint64_t current_amount;
  pvm_load(key, 52, (uint8_t*)&current_amount, 8, NULL);
  current_amount += amount;
  pvm_save(key, 52, (uint8_t*)&current_amount, 8);
  return 0;
}

int withdraw(uint8_t *address, uint8_t *asset_id, uint64_t amount)
{
  uint8_t key[52];
  memcpy(key, address, 20);
  memcpy(key + 20, asset_id, 32);
  uint64_t current_amount;
  pvm_load(key, 52, (uint8_t*)&current_amount, 8, NULL);
  if (current_amount < amount)
  {
    return 1;
  }
  current_amount -= amount;
  pvm_save(key, 52, (uint8_t*)&current_amount, 8);
  // tranfer asset from  contract to user
  return 0;
}

/**
 * The contract receive new balance state from trade engine and reset user's balance
 * saved in contract.
 * 
 * params:
 * - clear_records_len: the length of clear_records data
 * - clear_records: serial clear records, every record contains 60 bytes:
 *    - 00~19: address
 *    - 20~51: asset_id
 *    - 52~59: amount in u64, little-endian
 */
int simple_clear(uint32_t *clear_records_len, uint8_t *clear_records)
{
  // pvm_dump_memory(clear_records, *clear_records_len * 60);
  // pvm_dump_memory(clear_records_len, 4);
  for (uint32_t i = 0; i < *clear_records_len; i++)
  {
    pvm_save(clear_records + i * 60, 52, clear_records + i * 60 + 52, 8);
    // pvm_dump_memory(clear_records + i * 60, 60);
  }
  return 0;
}

int main(int argc, char *argv[])
{
  if (argc == 1)
  {
    return 0;
  }

  if (strcmp(argv[1], "clear") == 0)
  {
    if (argc != 4)
    {
      return 1;
    }
    return simple_clear((uint32_t *)argv[2], (uint8_t *)argv[3]);
  }
  return 0;
}
