// Pipe
// ref:
// 1. https://plippe.github.io/blog/2021/06/09/rust-extension-methods.html
fn main() {
    "Hello, World"
        .pipe(|s| {
            println!("{s} [first pipe]");
            return s;
        })
        .pipe(|s| println!("{s} [second pipe]"));
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
