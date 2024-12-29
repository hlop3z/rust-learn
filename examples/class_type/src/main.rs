struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Associated function (similar to a constructor)
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }

    // Method
    fn greet(&self) {
        println!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        );
    }
}

fn main() {
    println!("\n");
    let person = Person::new("Alice".to_string(), 30);
    person.greet(); // Output: Hello, my name is Alice and I am 30 years old.
}
