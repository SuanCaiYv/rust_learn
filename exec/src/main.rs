#![allow(unused)]

trait T {
    fn say(&self);
}

struct A {
    a: i32,
    b: i32,
}

struct B {
    c: i64,
    d: i64,
}

impl T for A {
    fn say(&self) {
        println!("A");
    }
}

impl T for B {
    fn say(&self) {
        println!("B");
    }
}

fn main() {
    let a = A { a: 1, b: 2 };
    let b = B { c: 3, d: 4 };
    // here, a and b are both trait object of T, but there are different types.
    // so with the purpose of 'save' them to same format(or said, type), directly assignment is not allowed.
    // we need an other way to resolve this, as you guess -- FatPointer is introduced to reach this goal.
    // DST is just an noun to describe this kind of type that can not be stored in a single pointer(or reference).
    let x: &dyn T = &a;
    let y: &dyn T = &b;
    x.say();
    y.say();
}