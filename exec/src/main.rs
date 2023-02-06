#![allow(unused)]

fn main() {
    // Default representation, alignment lowered to 2.
    #[repr(packed(2))]
    struct PackedStruct {
        first: i16,
        second: i8,
        third: i32,
    }

    // C representation, alignment raised to 8
    #[repr(C, align(8))]
    struct AlignedStruct {
        first: i16,
        second: i8,
        third: i32,
    }
}