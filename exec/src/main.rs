use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    let a = Cell::new(1);
    let b = RefCell::new(1);

    let a_rc = Rc::new(a);
    let b_rc = Rc::new(b);

    let a_rc1 = a_rc.clone();
    let a_rc2 = a_rc.clone();

    let b_rc1 = b_rc.clone();
    let b_rc2 = b_rc.clone();

    a_rc1.set(a_rc1.get() + 1);
    a_rc2.set(a_rc2.get() + 1);

    *b_rc1.borrow_mut() += 1;
    *b_rc2.borrow_mut() += 1;

    println!("{}", a_rc.get()); // 3
    println!("{}", *b_rc.borrow()); // 3
}
