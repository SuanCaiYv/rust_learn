#[derive(Debug)]
struct MyType {
    a: i64,
    b: Box<Inner>,
}

#[derive(Debug)]
struct Inner {
    a: i32,
    b: i32,
}

fn main() {
    let mut t = MyType {
        a: 1,
        b: Box::new(Inner {
            a: 2,
            b: 3,
        }),
    };
    let a;
    {
        a = &t;
    }
    let b;
    {
        b = &mut t;
    }
    println!("{:?}", a);
}
