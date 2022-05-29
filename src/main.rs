extern crate core;

mod base;

struct MyType<T> {
    val: T
}

fn main() {
    let mut x = MyType{val: String::from("aaa")};
    let mut y = &mut x;
    y.val = String::from("bbb")
}

