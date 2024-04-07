// In Rust, enums (short for enumerations) are a data type that allows you to define 
// a type by enumerating its possible variants. Each variant can optionally carry data. 
// Enums are particularly useful for representing a fixed set of possible values.

pub fn create_enum() {
    // Definition
    enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Speak(String),
    ChangeBGColor(i32, i32, i32),
    }
    // Instantiation
    let msg1 = Command::Quit;
    let msg2 = Command::Move{ x: 1, y: 2 };
    let msg3 = Command::Speak("Hi".to_owned());
    let msg4 = Command::ChangeBGColor(0, 0, 0);
}

enum Direction {
    North,
    South,
    East,
    West,
}

pub fn give_directions() {
    let direction1 = Direction::North;
    let direction2 = Direction::East;

    match direction1 {
        Direction::North => println!("Go north!"),
        Direction::South => println!("Go south!"),
        Direction::East => println!("Go east!"),
        Direction::West => println!("Go west!"),
    }

    match direction2 {
        Direction::North => println!("Go north!"),
        Direction::South => println!("Go south!"),
        Direction::East => println!("Go east!"),
        Direction::West => println!("Go west!"),
    }
}

enum TrafficLight {
    Red,
    Green,
    Yellow(u32), // Variant with associated data (the duration of the yellow light)
}

pub fn traffic_lights() {
    let light1 = TrafficLight::Red;
    let light2 = TrafficLight::Yellow(30);

    match light1 {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Green => println!("Go!"),
        TrafficLight::Yellow(duration) => println!("Prepare to stop in {} seconds!", duration),
    }

    match light2 {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Green => println!("Go!"),
        TrafficLight::Yellow(duration) => println!("Prepare to stop in {} seconds!", duration),
    }
}

