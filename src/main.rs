extern crate core;

use std::cell::{Cell, RefCell};
use std::rc::Rc;

mod base;

fn main() {
    let v = 0;
    let r = RefCell::new(v);
    let rc1 = Rc::new(r);
    let rc2 = rc1.clone();
    let rc3 = rc1.clone();
    *rc2.borrow_mut() += 1;
    *rc3.borrow_mut() += 1;
    println!("{}", rc1.borrow());
}
