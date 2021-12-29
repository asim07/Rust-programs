#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

struct Point<T,S> {
    x:T,
    y:S
}

pub fn main() {
    let a = Point{x:34,y:"Hello world"};
    println!("{}   {}",a.x,a.y);
}