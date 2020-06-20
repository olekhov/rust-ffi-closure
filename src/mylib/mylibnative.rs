extern crate libc;
use libc::{c_void, c_char, c_int, size_t};

type MyHandle = *mut c_void;

pub type FnCallback = unsafe extern "C" fn(*mut c_char, MyHandle) -> c_int;

#[link(name="mylib")]
extern {
    pub fn mylib_ctx_size() -> size_t;

    pub fn mylib_init_ctx(lib_ctx: MyHandle) -> c_int;

    pub fn mylib_func(lib_ctx: MyHandle, cb: FnCallback, cb_ctx: *mut c_void) -> c_int;

    pub fn mylib_print(lib_ctx: MyHandle) -> c_int;
}
