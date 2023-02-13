fn type_name<T: ?Sized>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6];
    // error syntax, just for understanding
    // let s1: [i32] = arr[0..2];
    // let s2: [i32] = arr[2..6];
    let ss1 = &arr[0..2];
    let ss2 = &arr[3..6];
    println!("{}", type_name(ss1));
    println!("{}", type_name(ss));
    println!("{}", type_name(&ss1));
    println!("{}", type_name(&ss2));
    /// output:
    /// <br>
    /// [i32]
    /// <br>
    /// [i32]
    /// <br>
    /// &[i32]
    /// <br>
    /// &[i32]
}