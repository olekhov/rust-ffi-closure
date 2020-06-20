
mod mylib;

fn main() {

    let mut my = mylib::MyLib::new().unwrap();

    // "hello from rust" - 15 bytes
    my.func("hello from rust").unwrap();
    my.print().unwrap();

    println!("Hello, world!");
}
