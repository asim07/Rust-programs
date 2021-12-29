#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

use std::collections::HashMap;
use std::mem;

pub fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("Sqaure"),2); // inserting value in map

    map.insert(String::from("Hello").into(),4);

    map.entry("ello".into()).or_insert(1);
    println!("memory used by {:?} is : {} bytes",map,mem::size_of_val(&map));
}