extern crate libc;
use libc::{c_void, c_int, c_char};

mod mylibnative;

pub struct MyLib {
    ctx: Box<[u8]>,
}

impl MyLib {
    pub fn new() -> Result<MyLib, i32> {
        let ctx_size = unsafe { mylibnative::mylib_ctx_size() };
        let v: Vec<u8> = vec![0; ctx_size];

        let mut res = MyLib {
            ctx: v.into_boxed_slice(),
        };

        let r = unsafe { mylibnative::mylib_init_ctx(res.get_ctx()) } ;

        if r != 0 {
            return Err(r);
        }

        Ok( res )
    }


    unsafe extern "C" fn trampoline<F>(buffer: *mut c_char, ctx: *mut c_void) -> c_int
        where
            F: FnMut(&mut [u8]) -> c_int,
        {
            // Safety: We know the context is a pointer to our closure because `trampoline` is
            // private and the only way you can create one is via a publicly defined getter
            let closure = &mut *(ctx as *mut F);

            // Convert the buffer to a Rust slice.
            //
            // Safety: The caller didn't pass in the buffer size, so it *must* have
            // allocated at least MAX_BUFFER_LEN bytes
            let buffer = std::slice::from_raw_parts_mut(buffer as *mut u8, mylibnative::MAX_BUFFER_LEN);

            // actually invoke our closure
            closure(buffer)
        }

    fn get_callback<F>(_closure: &F) -> mylibnative::StoreCallback
        where
            F: FnMut(&mut [u8]) -> c_int,
        {
            // ask the compiler to generate a "trampoline" specific to our closure, F
            MyLib::trampoline::<F>
        }

    pub fn func(&mut self, data: &str) -> Result<(), i32> {

        let mut write_data_callback = | buffer: &mut [u8] | {
            buffer.copy_from_slice(data.as_bytes());
            // callback should return 0 on success
            0
        };

        // get a function that can be used as a callback
        let callback = MyLib::get_callback(&write_data_callback);

        let r = unsafe {
            mylibnative::mylib_func(self.get_ctx(),
            callback,
            &mut write_data_callback as *mut _ as *mut c_void,)
        };

        if r != 0 {
            return Err(r);
        }

        Ok( () )

    }

    pub fn print(&mut self) -> Result< (), i32 > {
        let r = unsafe { mylibnative::mylib_print(self.get_ctx()) };

        if r != 0 {
            return Err(r);
        }

        Ok( () )
    }

    fn get_ctx(&mut self) -> *mut c_void {
        self.ctx.as_mut_ptr() as *mut c_void
    }
}
