use std::os::unix::prelude::PermissionsExt;

fn main() {
    let my_vec = vec![vec!["lahore", "karachi", "isb"], vec!["23", "43", "34"]];

    for mut x in my_vec {
        println!("city = {}", x[0]);
        while let Some(something) = x.pop() {
            if let Ok(number) = something.parse::<i32>() {
                println!("The number is {}", number);
            }
        }
    }
}
