use std::sync::{atomic::{AtomicI32, Ordering}, Arc};

#[derive(Clone)]
struct MyType {
    counter: Arc<AtomicI32>,
    s: String,
}

fn main() {
    let t1 = MyType {
        counter: Arc::new(AtomicI32::new(0)),
        s: "aaa".to_string(),
    };
    let _ = t1.counter.fetch_add(1, Ordering::Release);
    let _ = t1.counter.fetch_add(1, Ordering::Release);
    let t2 = t1.clone();
    let _ = t2.counter.fetch_add(1, Ordering::Release);
    let _ = t2.counter.fetch_add(1, Ordering::Release);
    println!("{:p}", &t1.s);
    println!("{:p}", &t2.s);
}
