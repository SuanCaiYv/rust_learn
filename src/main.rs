extern crate core;

mod base;

use core::panicking::panic;
use crate::base::*;

#[derive(Debug)]
struct MyType {
    a: i64,
    b: i32,
}

fn main() {
    let t1 = MyType{
        a: 1,
        b: 2,
    };
    println!("{:p}", &t1.a);
    println!("{:p}", &t1.b);
    let t2 = t1;
    println!("{:p}", &t2.a);
    println!("{:p}", &t2.b)
}

fn test() -> ! {
    panic("aaa");
}
