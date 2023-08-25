use std::fmt::{Display, Formatter};
use std::ptr::NonNull;

struct S;

impl Display for S {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "S")
    }
}

fn main() {
    let mut s: S = S{};
    let mut p: NonNull<dyn Display> = NonNull::new(&mut s).unwrap();
    let mut v: [i64;3] = [1, 2, 3];
    let mut u: &mut [i64] = &mut v;
    let mut w: NonNull<[i64]> = NonNull::new(&mut *u).unwrap();
    println!("{}", size_of::<* mut dyn Display>())
}

fn size_of<T>() -> usize {
    std::mem::size_of::<T>()
}