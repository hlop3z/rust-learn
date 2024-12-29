// Run @ `cargo run`

use std::collections::HashMap;

fn main() {
    println!("\n");
    // Dict
    demo_dict();
    // List
    demo_list();
}

fn demo_dict() {
    let mut my_dict = HashMap::new(); // Create an empty HashMap

    // Insert key-value pairs
    my_dict.insert("one", 1);
    my_dict.insert("two", 2);
    my_dict.insert("three", 3);

    println!("{:?}", my_dict); // Output: {"one": 1, "two": 2, "three": 3}
}

fn demo_list() {
    let mut my_list = Vec::new(); // Create an empty list

    my_list.push(10); // Add an element
    my_list.push(20);
    my_list.push(30);

    println!("{:?}", my_list); // Output: [10, 20, 30]
}
