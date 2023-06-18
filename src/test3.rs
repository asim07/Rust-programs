use std::fmt;
// #[derive(Copy, Clone)]
struct A<'a> {
    test: i32,
    value: &'a str,
}

impl fmt::Display for A<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.test, self.value)
    }
}

fn main() {
    let mut e = A {
        test: 1,
        value: "hello",
    };
    let a = &mut e;
    {
        let b = &a;
        let c = &b;
    }
    println!("{}", a);
    // e.test = 20;
    // println!("{}", );
}
