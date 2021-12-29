#![allow(dead_code)]
#![allow(unused_variables)]

pub fn for_loop() {

for(index,value)  in (20..41).enumerate() {
    println!("{} -> {}",index,value);
}

}

pub fn match_case() {
    let country_code = 400000;

    let result = match country_code {
        92 => "Pakistan",
        91 =>  "India",
        93 => "bangla",
        1..=3000 => "unknown",
        _ => "invalid"
    };
    println!("country is  = {}",result)
}