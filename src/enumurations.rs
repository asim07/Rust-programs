#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_must_use)]

enum Colors {
    Red,
    Green,
    RGB(u8, u8, u8),
    Combos{ dark : u8,  litedark : u8 , output:u8}
}

pub fn test() {
    let a = Colors::Combos{dark: 32,litedark:23,output: 40};
    match a {
        Colors::Red => println!("Red detected"),
        Colors::Green => println!("Green detected"),
        Colors::RGB(1,2,4) => println!("RGB Colors detected"),
        Colors::Combos{dark: 32,litedark:23,output: 40 } => println!("Combo  detected"),
        _ => {
            println!("InValid Output");
        }
    }
}