extern crate hello_lib;

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

fn main() {
    let hello = hello_lib::Greeter::new("Hello");
    hello.greet("world");
    let _zeros = 0_i128;
    let timestamp = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() & 0xffffffffffff) << 80;
    println!("{:b}", &timestamp);
    println!("{:#X}", &timestamp);
    let _a=0;
}
