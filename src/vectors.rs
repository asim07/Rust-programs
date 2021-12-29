#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

fn vectors () {

let mut a = Vec::new();
a.push(1);
a.push(2);
a.push(3);
println!("a: {:?}", a);
let index:usize = 2;
println!("Fetch element at index {}   :  {}",index,a[index]);

for index in &a {
println!("{}",index);
}

match a.get(2) {
    Some(v) => println!("Its done"),
    None => println!("Doesnt work")
}

}

pub fn main() {
    vectors();
}