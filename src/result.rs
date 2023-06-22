// fn check_error(value: i32) -> Result<(), ()> {
//     if value % 2 == 0 {
//         return Ok(());
//     } else {
//         return Err(());
//     }
// }
// fn main() {
//     if check_error(4).is_ok() {
//         println!("its working");
//     } else {
//         println!("Its error");
//     }
// }

//=================================================================================
//bit complex stuff

// fn check_if_five(number: i32) -> Result<i32, String> {
//     match number {
//         5 => Ok(number),
//         _ => Err("Its not a even number".to_string()),
//     }
// }

// fn main() {
//     let mut my_vec = Vec::new();
//     for number in 2..=7 {
//         my_vec.push(check_if_five(number));
//     }
//     println!("{:?}", my_vec);
// }

//================================================================
//now with turbifish

use std::num::ParseIntError;

fn return_number(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>()
}

fn main() {
    let value = return_number("32");
        let value = value.unwrap();
    println!("{}", value);
}
