/// Singleton
/// ref:
/// 1. https://refactoring.guru/design-patterns/singleton/rust/example#example-2
use std::sync::Mutex;

static ARRAY: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn main() {
    do_call();
    do_call();
    do_call();
    println!("Called {times} times", times = ARRAY.lock().unwrap().len())
}

fn do_call() {
    ARRAY.lock().unwrap().push(1);
}
