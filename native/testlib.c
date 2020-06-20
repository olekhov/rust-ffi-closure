#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "mylib.h"

uint8_t for_lib[16]= {
  0xf4, 0x70, 0x09, 0xee, 0x43, 0x99, 0x09, 0xdb,
  0x32, 0x85, 0x03, 0x14, 0x46, 0xde, 0x40, 0x9d
};

int callback(uint8_t * data, void *ctx)
{
	memcpy(data, for_lib, sizeof(for_lib));
	return 0;
}

int main()
{
	void *lib_ctx;

	lib_ctx = malloc(mylib_ctx_size());

	mylib_init_ctx(lib_ctx);
	mylib_func(lib_ctx, callback, NULL);

	mylib_print(lib_ctx);

	free(lib_ctx);
	return 0;
}
