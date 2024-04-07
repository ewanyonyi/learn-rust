use std::collections::HashMap;

pub fn print_hashmap() {
    let mut subs = HashMap::new();
    subs.insert(String::from("LGR"), 100000);
    subs.insert(2.to_string(), 40);
    // Insert key if it doesn't have a value
    subs.entry("Golang Dojo".to_owned()).or_insert(3);

    println!("hash map:{:?}", subs);

    if let Some(value) = subs.get("key") {
        println!("Value: {}", value);
    }

    if subs.contains_key("LGR") {
        println!("Key exists");
    }

    println!("Number of items: {}", subs.len());

    for (key, value) in subs.iter() {
        println!("Key: {}, Value: {}", key, value);
    }

    for key in subs.keys() {
        println!("Key: {}", key);
    }

    for value in subs.values() {
        println!("Value: {}", value);
    }

    subs.clear();

    println!("hash map: {:?}", subs);


}