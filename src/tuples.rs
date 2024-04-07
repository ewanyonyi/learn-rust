pub fn print_tuples() {
    let coordinates = (82, 64);
    let score = ("Team A", 12);

    println!("Tuple: {:?}", coordinates);
    println!("First element: {}", score.0);
    println!("Second element: {}", score.1);
}
