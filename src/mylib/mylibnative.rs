extern crate libc;
use libc::{c_void, c_char, c_int, size_t};

type MyLibHandle = *mut c_void;

pub const MAX_BUFFER_LEN : usize = 16;
pub type StoreCallback = unsafe extern "C" fn(buffer:*mut c_char, callback_ctx: *mut c_void) -> c_int;

#[link(name="mylib")]
extern "C" {
    pub fn mylib_ctx_size() -> size_t;

    pub fn mylib_init_ctx(lib_ctx: MyLibHandle) -> c_int;

    pub fn mylib_func(lib_ctx: MyLibHandle, cb: StoreCallback, cb_ctx: *mut c_void) -> c_int;

    pub fn mylib_print(lib_ctx: MyLibHandle) -> c_int;
}
