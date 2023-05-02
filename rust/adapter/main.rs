/// Adapter
/// ref:
/// 1. https://refactoring.guru/design-patterns/adapter/rust/example
fn main() {
    let target = OrdinaryTarget;

    print!("A compatible target can be directly called: ");
    call(target);

    let adaptee = SpecificTarget;

    println!(
        "Adaptee is incompatible with client: '{req}'",
        req = adaptee.specific_request()
    );

    let adapter = TargetAdapter::new(adaptee);

    print!("But with adapter client can call its method: ");
    call(adapter);
}

/// Calls any object of a `Target` trait.
///
/// To understand the Adapter pattern better, imagine that this is
/// a client code, which can operate over a specific interface only
/// (`Target` trait only). It means that an incompatible interface cannot be
/// passed here without an adapter.
fn call(target: impl Target) {
    println!("'{req}'", req = target.request())
}

trait Target {
    fn request(&self) -> String;
}

struct OrdinaryTarget;

impl Target for OrdinaryTarget {
    fn request(&self) -> String {
        "Ordinary request".to_string()
    }
}

struct TargetAdapter {
    adaptee: SpecificTarget,
}

impl TargetAdapter {
    fn new(adaptee: SpecificTarget) -> Self {
        Self { adaptee }
    }
}

impl Target for TargetAdapter {
    /// Convert data
    fn request(&self) -> String {
        self.adaptee.specific_request()
    }
}

struct SpecificTarget;

impl SpecificTarget {
    fn specific_request(&self) -> String {
        "test".to_string()
    }
}
