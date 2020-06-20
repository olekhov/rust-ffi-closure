all: libmylib.so testlib

run: all
	LD_LIBRARY_PATH=. ./testlib

testlib: testlib.c
	gcc $< -o $@ -lmylib -L.

libmylib.so : mylib.c
	gcc mylib.c -shared -o $@
