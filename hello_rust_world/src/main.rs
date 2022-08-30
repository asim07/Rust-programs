#![allow(dead_code)]
#![allow(unused_variables)]
// #![allow(unused_imports)]
// #![allow(unused_mut)]

mod circular_reference;
mod cl;
mod concurrency;
mod data_structures;
mod functions;
mod life_and_mem;
mod ng_game;
mod sh;
mod standard_collections;
mod strings;
mod traits;

use std::mem; //memory
              //global variable
const MEANING_OF_LIFE: u8 = 42; //no fixed address, it is available in the entire program
static mut Z: i32 = 123;

fn data_types() {
    let a: u8 = 100; //u = unsigned, 8 bits, 0 to 255
    println!("a = {}", a); //immutable

    let mut b: i8 = 0; //i = signed, m means mutable, -128 to 127
    println!("b = {} before", b);
    b = 2;
    println!("b = {} after", b);
    let mut c = 123456789;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    //u8, u16, u32, u64, i8, i16, ...

    //usize isize --> dynamic sized typ depends on machine
    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!(
        "z = {}, takes up {} bytes, {}-bit OS",
        z,
        size_of_z,
        size_of_z * 8
    );

    let d: char = 'x';
    println!(
        "d = {} is a char, takes up {} bytes",
        d,
        mem::size_of_val(&d)
    );

    //f64 that is bydefault, and f32 ---> IEEE752 signed
    let e: f32 = -2.5;
    println!("e = {}, takes up {} bytes", e, mem::size_of_val(&e));

    let g: bool = false;
    println!("g = {}, takes up {} bytes", g, mem::size_of_val(&g));
}

fn operators() {
    //ARITHMETIC OPERATIONS

    let mut a = 2 + 3 * 2;
    println!("{}", a);
    a = a + 1; // -- and ++ is not allowed in rust so,
    a += 1; // a = a + 1
    println!("{}", a);
    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    /////////// POWER AND PI ///////////

    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let num: f32 = 3.24;
    let result = num.powi(4); //here powi stands for power is an integer type that is 4
    println!("Powi is: {}", result);

    let b_to_pi = f32::powf(num, std::f32::consts::PI); //in powf, f stands for floating point that is PI
    println!("{}^pi  = {}", num, b_to_pi);

    /////////// BITWISE OPERATORS (Only for Integers)///////////

    let c = 1 | 2; // | OR, & AND, ^ XOR ! NOR
                   //01 OR 10 = 11 == 3 in decimals
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10; //<< stands for shift to the left upto 10 places, 0,2,4,8,16,32,64,128,256,512,1024
    println!("2^10 = {}", two_to_10);

    /////////// LOGICAL OPERATORS ///////////
    let pi_less_4 = std::f64::consts::PI < 4.0;
    println!("Pi is less then 4 = {}", pi_less_4)
    //given logical operators: >, <, >=, <=, ==
}

fn scope_and_shadowing() {
    let x: i16 = 2;
    println!("x = {}", x);
    {
        let x = 20;
        println!("inside x = {}, and it i didnot declear the new x again here then it refers to the outer one that's called shadowing!", x);
        let a = 12;
        println!("inside a = {}", a)
    }
    println!("outside x = {}", x);
    // println!("In outside a not accessible{}", a) //this line gives me the warning of scope
}

fn if_statement() {
    let temp = 5;
    if temp > 30 {
        println!("really hot today!")
    } else if temp < 10 {
        println!("it's too cold today!")
    } else {
        println!("temperature is oky!")
    }
    //if statement that returns a value that you can assign
    let day = if temp > 20 { "sunny" } else { "cloudy" };
    println!("the temperature is {}", day);
}

fn while_and_loop() {
    let mut x = 1;
    while x < 1000 {
        x *= 2;

        if x == 64 {
            continue;
        }

        println!("x = {}", x);
    }

    let mut y = 1;

    loop
    // while true, it will not stop until we break out
    {
        y *= 2;
        println!("y = {}", y);

        // stop at 2^10
        if y == 1 << 10 {
            break;
        }
    }
}

fn forloop() {
    for x in 1..11
    // 1 to 10; 11..1 won't work
    {
        // skip 3
        if x == 3 {
            continue;
        }

        // stop at 7
        if x == 8 {
            break;
        }

        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statement() {
    let country_code = 999;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "unknown", // inclusive range ... is deprecated
        _ => "invalid",        // try commenting this out - must cover all cases!
    };

    println!("the country with code {} is {}", country_code, country);

    let x = false;

    let s = match x {
        true => "yes",
        false => "no",
    };
    println!("{}", s)
}

fn main() {
    // data_types();
    // operators();
    // scope_and_shadowing();
    // So an unsafe block is essentially your promise that you are working carefully with this variable
    // unsafe { println!("{}", Z) }
    // sh::stack_and_heap();
    // if_statement();
    // while_and_loop();
    // forloop();
    // match_statement();
    // cl::combination_lock();
    // data_structures::structures();
    // data_structures::enums();
    // data_structures::union();
    // data_structures::option_some_none();
    // data_structures::arrays();
    // data_structures::slices();
    // data_structures::touple();
    // data_structures::pattern_matching();
    // data_structures::generics();
    // standard_collections::vectors();
    // standard_collections::hashmaps();
    // standard_collections::hashsets();
    // standard_collections::iterators();
    // strings::strings_and_characters();
    // ng_game::number_guessing_game();
    // functions::function_demo();
    // functions::closures();
    // functions::high_order_functions();
    // traits::trait_func();
    // traits::trait_parameters();
    // traits::into_trait();
    // traits::drop_trait();
    // traits::operator_overloading();
    // traits::static_dispatch();
    // traits::dynamic_dispatch();
    // traits::another_dynamic_dispatch();
    // traits::vectors_with_different_objects();
    // life_and_mem::ownership();
    // life_and_mem::borrowing();
    // life_and_mem::lifetime();
    // life_and_mem::lifetime_with_struct();
    // life_and_mem::reference_counted_var();
    // life_and_mem::atomic_reference_counted_var();
    // circular_reference::c_r();
    concurrency::spawning_and_joining_thread();
}
