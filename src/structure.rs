pub fn create_struct() {

    // Definition
    #[derive(Debug)]
    struct User {
        username: String,
        active: bool,
    }

    // Instantiation
    let user1 = User {
        username: String::from("bogdan"),
        active: true,
    };

    // Tuple struct
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);

    println!("{:?}", user1);
    println!("{:?}", black);
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function to create a new instance of Rectangle
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if the rectangle is a square
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

pub fn create_rectangle() {
    // Creating a new instance of Rectangle using associated function
    let rect = Rectangle::new(10, 20);

    // Using a method to calculate the area
    println!("Area: {}", rect.area()); // Output: Area: 200

    // Using a method to check if it's a square
    println!("Is square? {}", rect.is_square()); // Output: Is square? false
}
