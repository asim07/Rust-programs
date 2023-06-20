use std::fmt;

fn return_thing<T: std::fmt::Display + std::fmt::Debug>(value: T) -> T {
    println!("Your value {}", &value);
    value
}

#[derive(Debug)]
struct Mystruct<'a> {
    name: String,
    phone: u32,
    email: &'a str,
}

// Implement `Display` for `MinMax`.
impl<'a> fmt::Display for Mystruct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{} {} {}", self.name, self.phone, self.email)
    }
}

fn main() {
    let myst = Mystruct {
        name: "hello world".to_string(),
        phone: 03456781690,
        email: "Helow world",
    };

    println!("Values of the struct : {}", &myst);

    return_thing(myst);
}
