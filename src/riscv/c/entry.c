#include "duktape.h"
#include "sdk.h"

duk_double_t dummy_get_now(void) {
  return -11504520000.0;
}

static duk_ret_t duk_env_debug(duk_context *ctx) {
  duk_push_string(ctx, " ");
  duk_insert(ctx, 0);
  duk_join(ctx, duk_get_top(ctx) - 1);
  env_debug(duk_safe_to_string(ctx, -1));
  return 0;
}

static duk_ret_t duk_env_ret(duk_context *ctx) {
  void *ptr;
  duk_size_t sz;
  ptr = duk_get_buffer_data(ctx, -1, &sz);
  duk_pop_n(ctx, 1);

  env_ret(ptr, sz);
  return 0;
}

static duk_ret_t duk_env_save(duk_context *ctx) {
  void *v_ptr;
  duk_size_t v_sz;
  v_ptr = duk_get_buffer_data(ctx, -1, &v_sz);

  void *k_ptr;
  duk_size_t k_sz;
  k_ptr = duk_get_buffer_data(ctx, -2,  &k_sz);

  duk_pop_n(ctx, 2);

  env_save(k_ptr, k_sz, v_ptr, v_sz);
  return 0;
}

static duk_ret_t duk_env_load(duk_context *ctx) {
  void *k_ptr;
  duk_size_t k_sz;
  k_ptr = duk_get_buffer_data(ctx, -1, &k_sz);

  duk_pop_n(ctx, 1);

  void *v_ptr = duk_push_buffer(ctx, 4096, 1);
  duk_size_t r_size;
  env_load(k_ptr, k_sz, v_ptr, 4096, &r_size);

  duk_resize_buffer(ctx, -1, r_size);

  return 1;
}

static duk_ret_t duk_env_address(duk_context *ctx) {
  void *ptr = duk_push_buffer(ctx, 20, 0);
  env_address(ptr);
  return 1;
}

static duk_ret_t duk_env_balance(duk_context *ctx) {
  void *addr_ptr;
  addr_ptr = duk_get_buffer_data(ctx, -1, NULL);

  void *v_ptr = duk_push_buffer(ctx, 32, 0);

  env_balance(addr_ptr, v_ptr);

  return 1;
}

static duk_ret_t duk_env_origin(duk_context *ctx) {
  void *ptr = duk_push_buffer(ctx, 20, 0);
  env_origin(ptr);
  return 1;
}

static duk_ret_t duk_env_caller(duk_context *ctx) {
  void *ptr = duk_push_buffer(ctx, 20, 0);
  env_caller(ptr);
  return 1;
}

static duk_ret_t duk_env_callvalue(duk_context *ctx) {
  void *v_ptr = duk_push_buffer(ctx, 32, 0);
  env_callvalue(v_ptr);
  return 1;
}

static duk_ret_t duk_env_blockhash(duk_context *ctx) {
  duk_int_t h = duk_get_int(ctx, -1);
  duk_pop_n(ctx, 1);

  void *hash_ptr = duk_push_buffer(ctx, 32, 0);
  env_blockhash(h, hash_ptr);
  return 1;
}

static duk_ret_t duk_env_coinbase(duk_context *ctx) {
  void *ptr = duk_push_buffer(ctx, 20, 0);
  env_coinbase(ptr);
  return 1;
}

static duk_ret_t duk_env_timestamp(duk_context *ctx) {
  uint64_t timestamp;
  env_timestamp(&timestamp);
  duk_push_int(ctx, timestamp);
  return 1;
}

static duk_ret_t duk_env_number(duk_context *ctx) {
  void *v_ptr = duk_push_buffer(ctx, 32, 0);
  env_number(v_ptr);
  return 1;
}

static duk_ret_t duk_env_difficulty(duk_context *ctx) {
  void *v_ptr = duk_push_buffer(ctx, 32, 0);
  env_difficulty(v_ptr);
  return 1;
}

static duk_ret_t duk_env_gaslimit(duk_context *ctx) {
  uint64_t gaslimit;
  env_gaslimit(&gaslimit);
  duk_push_int(ctx, gaslimit);
  return 1;
}

void env_init(duk_context *ctx) {
  duk_push_object(ctx);

  duk_push_c_function(ctx, duk_env_debug, DUK_VARARGS);
  duk_put_prop_string(ctx, -2, "debug");

  duk_push_c_function(ctx, duk_env_ret, 1);
  duk_put_prop_string(ctx, -2, "ret");

  duk_push_c_function(ctx, duk_env_save, 2);
  duk_put_prop_string(ctx, -2, "save");

  duk_push_c_function(ctx, duk_env_load, 1);
  duk_put_prop_string(ctx, -2, "load");

  duk_push_c_function(ctx, duk_env_address, 0);
  duk_put_prop_string(ctx, -2, "address");

  duk_push_c_function(ctx, duk_env_balance, 1);
  duk_put_prop_string(ctx, -2, "balance");

  duk_push_c_function(ctx, duk_env_origin, 0);
  duk_put_prop_string(ctx, -2, "origin");

  duk_push_c_function(ctx, duk_env_caller, 0);
  duk_put_prop_string(ctx, -2, "caller");

  duk_push_c_function(ctx, duk_env_callvalue, 0);
  duk_put_prop_string(ctx, -2, "callvalue");

  duk_push_c_function(ctx, duk_env_blockhash, 1);
  duk_put_prop_string(ctx, -2, "blockhash");

  duk_push_c_function(ctx, duk_env_coinbase, 0);
  duk_put_prop_string(ctx, -2, "coinbase");

  duk_push_c_function(ctx, duk_env_timestamp, 0);
  duk_put_prop_string(ctx, -2, "timestamp");

  duk_push_c_function(ctx, duk_env_number, 0);
  duk_put_prop_string(ctx, -2, "number");

  duk_push_c_function(ctx, duk_env_difficulty, 0);
  duk_put_prop_string(ctx, -2, "difficulty");

  duk_push_c_function(ctx, duk_env_gaslimit, 0);
  duk_put_prop_string(ctx, -2, "gaslimit");

  duk_put_global_string(ctx, "env");
}

int main(int argc, char *argv[]) {
  duk_context *ctx = duk_create_heap_default();
  env_init(ctx);

  if (argc == 1) {
    return 1;
  } else if (argc == 2) {
    duk_eval_string(ctx, argv[1]);
    duk_pop(ctx); /* pop eval result */
  } else {
    /* TODO: load source from one of the dep */
  }
  duk_destroy_heap(ctx);

  return 0;
}
