use std::fmt;

pub struct City<'a> {
    name: &'a str,
    date: u32,
}

impl fmt::Display for City<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} , {}", self.name, self.date)
    }
}

fn main() {
    let my_city = City {
        name: "lahore",
        date: 1992,
    };
    println!("{}", my_city);
}
