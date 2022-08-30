//Now, this file is going to contain some definitions for greetings done in different languages.
//This crate is actually composed of modules and Module is a kind of grouping for related functionality.
pub mod greetings {
    //sub modules

    //here the rust uses convention over configuration approach to figure out the location
    pub mod english;
    pub mod french {
        pub fn hello() -> String {
            "bonjour".to_string()
        }
        pub fn goodbye() -> String {
            "au revoir".to_string()
        }
    }
}
