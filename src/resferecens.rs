//simple reference example
// fn return_fucntion() -> String {
//     let country = String::from("Pak");
//     country
// }

// fn main() {
//     let country = return_fucntion();
//     println!("{}", country);
// }

// now mutable reference

// fn main() {
//     let mut number = 4;
//     let number_change = &mut number;
//     *number_change += 10;
//     let number_ref = &number_change;
//     println!("{}", number_change);
// }

// there three simple mutable references have
// 1. can have a mutable reference
// 2. can have immutable references
// 3. immutable  and mutable reference cant be at the same time.

//passing references to the functions

//this function is taking string as parameter not the ref, so its ownership will be transfered here and will be returned as need
fn print_country(country_name: String) -> String {
    println!("{}", country_name);
    country_name
}

//this fucntion is taking the arguments as reference and its just borrowed no ownership is transferred.
fn print_country_by_ref(name: &String) {
    println!("{}", name);
}

//main logic to run those
fn main() {
    let value = String::from("qsdadas");
    print_country_by_ref(&value);
    let value = print_country(value);
    print_country(value);
}
