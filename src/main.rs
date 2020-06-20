
mod mylib;

fn main() {

    let mut my = mylib::MyLib::new().unwrap();

    // "hello from rust\0" - exactly 16 bytes
    my.func("hello from rust\0").unwrap();
    my.print().unwrap();
}
