#pragma once
#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef int (*callback_fn)(uint8_t *data, void *cb_ctx);

size_t mylib_ctx_size();
int mylib_init_ctx(void* lib_ctx);
int mylib_func(void *lib_ctx, callback_fn cb, void *cb_ctx);
int mylib_print(void *lib_ctx);

#ifdef __cplusplus
};
#endif
