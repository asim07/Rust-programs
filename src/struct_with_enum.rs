struct Book {
    number: u32,
}
impl Book {
    fn get_number(&self) -> u32 {
        self.number
    }

    fn change_number(&mut self, number: u32) {
        self.number = number;
    }

    //associated function
    fn new(number: u32) -> Book {
        Book { number }
    }

    //normal fucntion
    pub fn test(value: u32) -> u32 {
        value * 2
    }
}

fn main() {
    let mut book = Book::new(23);
    println!("{}", book.get_number());
    book.change_number(32);
    println!("{}", book.get_number());
    println!("{}", Book::test(32));
}
