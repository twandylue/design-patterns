// Traits are more than interfaces
// ref: https://felix-knorr.net/posts/2023-04-17-traits.html
fn main() {
    let data = Data::String("test".to_string());

    println!("data string: {result}", result = data.get_as::<&str>())
}

enum Data {
    String(String),
    Vec(Vec<Data>),
}

impl Data {
    fn get_as<'a, T: DataAs<'a>>(&'a self) -> T {
        // T::get_as(&self)
        <T as DataAs>::get_as(&self)
    }
}

trait DataAs<'a>: std::fmt::Debug {
    fn get_as(data: &'a Data) -> Self;
}

impl<'a> DataAs<'a> for &'a str {
    fn get_as(data: &'a Data) -> &'a str {
        if let Data::String(s) = data {
            &s
        } else {
            "test1234"
        }
    }
}
