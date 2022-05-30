extern crate core;

use std::thread;

mod base;

struct MyType<'a, 'b, T, U> {
    v1: &'a T,
    v2: &'b U,
}

const s: &str = "aaa";

fn main() {
    let v = vec![1, 2, 3];
    // 'static被缩小为'a，所以可行
    f(s);
    f(v);
}

fn f<'a, T: 'a>(v: T) {}

