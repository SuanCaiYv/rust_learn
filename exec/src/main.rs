use std::ptr::NonNull;

type O = Option<NonNull<i64>>;

fn main() {
    println!("size: {}", std::mem::size_of::<O>());
}