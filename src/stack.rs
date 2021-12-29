#![allow(dead_code)]
use std::mem;

struct Point {
    x: i64,
    y: i64,
    z:i64
}

fn origin () -> Point {
    Point { x: 1, y:2,z:10}

}
pub fn stack_and_heap() {
    let p1 = origin();
    let p2 = Box::new(origin());
    let p3 = &p2;
    println!("memory used by p1 is : {} bytes",mem::size_of_val(&p1));
    println!("memory used by p2 is : {} bytes",mem::size_of_val(&p2));
    println!("memory used by p3 is : {} bytes",mem::size_of_val(&p3));
    println!("memory used by p3.x = {} and p3.y = {}",mem::size_of_val(&p3.x),mem::size_of_val(&p3.y));
}


