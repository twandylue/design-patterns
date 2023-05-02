// Pipe
// ref:
// 1. https://plippe.github.io/blog/2021/06/09/rust-extension-methods.html
use std::fmt::Debug;

fn main() {
    "Hello, World"
        .pipe(|s| {
            println!("{s} [first pipe]");
            return s;
        })
        .pipe(|s| println!("{s} [second pipe]"));
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

// Pipe
trait PipeIdExt {
    fn pipe<A, F>(self, f: F) -> A
    where
        Self: Sized,
        F: FnOnce(Self) -> A,
    {
        f(self)
    }
}

impl PipeIdExt for &str {}
