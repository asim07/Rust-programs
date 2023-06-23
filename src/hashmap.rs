use std::{collections::HashMap, hash::Hash};
#[derive(Debug)]
struct Cities<'a> {
    name: String,
    population: HashMap<&'a str, u32>,
}

fn main() {
    let mut city = Cities {
        name: "Lahore".to_string(),
        population: HashMap::new(),
    };
    city.population.insert("Dha", 23);
    city.population.insert("Old lahore", 30);
    city.population.insert("behria", 3124);

    for (k, v) in city.population.iter() {
        println!("name of palce is {} and population  is {}", k, v);
    }
}
