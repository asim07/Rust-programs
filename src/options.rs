#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub fn test() {
    let x = 3.0;
    let y = 2.0;
    let result = if y != 0.0 {Some(x/y)} else {None};

    match result {
        Some(z) => {
            println!("It works");
            println!("{} ",z);
        },
        None => println!("It didnt work")
    }
    if let Some(z) = result {println!("{}",z)};

}
