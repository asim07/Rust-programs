extern crate rand;
use rand::{thread_rng, Rng};

//my crate
extern crate phrases;
use phrases::greetings::french;

fn main() {
    let mut rnd = thread_rng();
    let b: bool = rnd.gen();
    println!("{}", b);

    //my builted crate
    println!(
        "English: {}, {}",
        phrases::greetings::english::hello(),
        phrases::greetings::english::goodbye()
    );

    println!("French: {}, {}", french::hello(), french::goodbye())
}
