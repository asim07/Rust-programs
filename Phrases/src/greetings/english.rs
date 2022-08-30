//! This module contains english phrases
//!
//! # Examples
//! ```

//!  //Now here whatever I write will be completely valid or compilable code
//! let username = "maham";
//! Println!(“{}, {}”, phrases::greetings::english::hello(), username);

//!```
//! to make it's doc run the command below:
//! rustdoc english.rs

pub fn hello() -> String {
    "hello".to_string()
}
pub fn goodbye() -> String {
    "goodbye".to_string()
}
