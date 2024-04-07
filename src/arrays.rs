pub fn print_arrays() {
    // Array & Slice
    // Arrays must have a known length and all
    // elements must initialized
    let array = [1,2,3,4,5];
    let array2 = [0; 3]; //[0,0,0]

    println!("{:?}", array);
    println!("{:?}", array2);

    // Unlike arrays the length of a slice is
    // determined at runtime
    let slice = &array[1 .. 3];
    println!("{:?}", slice);
}
