#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

pub fn arrays () {
    let mut a:[i32;5] = [1,2,3,4,5];
    let b = [1;10];
    a[0] = 432;
    println!("Elements of a {:?}",a);
    println!("Elements of b {:?}",b);
}

