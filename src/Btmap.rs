use std::collections::HashMap;

fn main() {
    let canadian_cities = vec!["calgary", "vancouver", "Gimli"];
    let german_cities = vec!["karl", "bad doberan", "bielefeld"];
    let mut city_hashmap = HashMap::new();
    for city in canadian_cities {
        city_hashmap.insert(city, "Canada");
    }
    for city in german_cities {
        city_hashmap.insert(city, "Germany");
    }

    println!("{}", city_hashmap["karl"]);
}
