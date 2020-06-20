#include <stdio.h>
#include <string.h>

#include "mylib.h"

struct opaque_library_context {
	int inited;
	uint8_t some_data[16];
	uint8_t some_other_data[200];
};


size_t mylib_ctx_size()
{
	return sizeof(struct opaque_library_context);
}

int mylib_init_ctx(void* lib_ctx)
{
	struct opaque_library_context *ctx = (struct opaque_library_context*) lib_ctx;
	memset(ctx, 0, sizeof(struct opaque_library_context));

	// some other init stuff

	ctx->inited = 1;
	return 0;
}

int mylib_func(void *lib_ctx, callback_fn cb, void *cb_ctx)
{
	struct opaque_library_context *ctx = (struct opaque_library_context*) lib_ctx;
	uint8_t data_from_callback[16];

	if( 0 == cb(data_from_callback, cb_ctx) ) {
		// check data_from_callback
		// if data looks good, grab to library context
		memcpy(ctx->some_data, data_from_callback, sizeof(data_from_callback));
	} else {
		return -1;
	}

	return 0;
}

int mylib_print(void *lib_ctx)
{
	struct opaque_library_context *ctx = (struct opaque_library_context*) lib_ctx;
	int i;
	printf("value stored with callback:\n");
	for(i=0;i<sizeof(ctx->some_data);i++) {
		printf("%02X ", ctx->some_data[i]);
	}
	printf("\n");
	return 0;
}
