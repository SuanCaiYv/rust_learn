trait MyTrait {
    fn foo(&mut self) -> i64;
}

struct MyStruct {
    x: i64,
}

impl MyTrait for MyStruct {
    fn foo(&mut self) -> i64 {
        self.x
    }
}

fn main() {
    let mut v: Vec<&dyn MyTrait> = vec![];
}

fn size_of<T>() -> usize {
    std::mem::size_of::<T>()
}