extern crate core;

mod base;

use std::fmt::Debug;
use crate::base::*;

#[derive(Debug)]
struct Type0 {
    a: i32,
    b: i32,
}

#[derive(Debug)]
struct Type {
    a: &'static str,
    b: i64,
}

fn main() {
    let a = Type0 { a: 1, b: 2 };
    let b = Type { a: "aaa", b: 3 };
    f(&a);
    f(&b);
}

fn f<T>(v: &T) where T: Debug + 'static {
    println!("{:?}", v)
}
