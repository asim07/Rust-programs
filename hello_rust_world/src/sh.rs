#![allow(dead_code)]
use std::mem;

struct Point {
    x: f64,
    y: f64,
}

fn origin() -> Point {
    //i want to return the Point
    Point { x: 0.0, y: 0.0 }
}

pub fn stack_and_heap() {
    let p1 = origin(); //by using stack method
    let p2 = Box::new(origin()); //by using heap method
    println!("p1 takes up {} bytes", mem::size_of_val(&p1)); //we have taken up the size amount of the desired variable that's why we getting 16 bytes
    println!("p2 takes up {} bytes", mem::size_of_val(&p2)); ////I'm running this on 64 bits system, heap saves the address and the address takes only 64 bits according to my system, that's why we are getting 8 bytes

    //to get the box value in the stack
    let p3 = *p2;
    println!("{}", p3.x);
}
