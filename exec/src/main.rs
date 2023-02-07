use std::fmt::Debug;

trait DoSomething<T> {
    fn do_sth(&self, value: T);
}

impl<'a, T: Debug> DoSomething<T> for &'a usize {
    fn do_sth(&self, value: T) {
        println!("{:?}", value);
    }
}

fn foo(b: Box<dyn for<'a> DoSomething<&'a usize>>) {
    let s: usize = 10;
    b.do_sth(&s);
}

fn main() {
    let x = Box::new(&2usize);
    foo(x);
}