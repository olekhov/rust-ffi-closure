extern crate libc;
use libc::{c_void};

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

    pub fn func(&mut self, data: &str) -> Result<(), i32> {

        let mut data_callback = | mut out_data: &[u8;16], cb_ctx : *mut c_void | {
            out_data.copy_from_slice(data.as_bytes());
            0
        };

        //
        // ???
        //

        let mut cb_ctx = std::ptr::null();

        let r = unsafe {
            mylibnative::mylib_func(self.get_ctx(),
                data_callback /* ? */,
                cb_ctx /* ? */
                )
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
