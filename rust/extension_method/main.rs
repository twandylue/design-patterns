// Extension methonds
// ref:
// 1. https://plippe.github.io/blog/2021/06/09/rust-extension-methods.html
use std::fmt::Debug;

fn main() {
    println!("Hello, World!");
    let p = Person::new();
    p.debug();
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: String,
}

impl Person {
    fn new() -> Self {
        Person {
            name: "andy".to_string(),
            age: 18,
            address: "test".to_string(),
        }
    }
}

// Extension Method
trait DebugExt: Debug {
    fn debug(&self) {
        println!("{self:?}");
    }
}

impl DebugExt for Person {}
