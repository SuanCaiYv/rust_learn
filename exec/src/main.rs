enum Book {
    Cpp(String),
    Java(f64),
    Golang{
        name: String,
        price: f64,
    },
    Rust(u64),
}

fn main() {
    let golang_size = std::mem::size_of::<String>() + std::mem::size_of::<f64>();
    println!("tag size: {} bytes.", std::mem::size_of::<Book>() - golang_size);
}