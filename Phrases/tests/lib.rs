#[cfg(test)]
mod tests {

    extern crate phrases;
    //to test the greetings library
    #[test]
    #[should_panic] //now we are expecting the failure that's why it should be passed
    #[ignore] //tells us that test run doesnot run this particular test if any
    fn english_greetings_correct() {
        assert_eq!("hellow", phrases::greetings::english::hello());
    }
    //cargo build
    //cargo test
}
