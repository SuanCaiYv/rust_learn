use std::ptr::NonNull;

type E = Option<NonNull<i64>>;

fn main() {
    println!("size: {}", std::mem::size_of::<E>());
}