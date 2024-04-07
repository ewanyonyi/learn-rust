mod basic_types {
    // // Basic Types & Variables
    // bool - Boolean
    // Unsigned integers
    // u8, u16, u32, u64, u128
    // Signed integers
    // i8, i16, i32, i64, i128
    // Floating point numbers
    // f32, f64
    // Platform specific integers
    // usize - Unsigned integer. Same number of bits as the
    // platform's pointer type.
    // isize - Signed integer. Same number of bits as the
    // platform's pointer type.
    // char - Unicode scalar value
    // &str - String slice
    // String - Owned string

    // Constant
    const MAX_POINTS: u32 = 100_000;
    // Static variable
    // Unlike constants static variables are
    // stored in a dedicated memory location
    // and can be mutated.
    static MAJOR_VERSION: u32 = 1;
    static mut COUNTER: u32 = 0;
    }