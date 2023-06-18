use std::{io::stdin, os::unix::prelude::PermissionsExt};
struct Book {
    bookName: String,
    author: String,
}

impl Book {
    fn new(a: &str, b: &str) -> Book {
        Book {
            bookName: String::from(a),
            author: String::from(b),
        }
    }
    fn printInfo(&self) {
        print!(
            "book name   : {}author name : {}",
            self.bookName, self.author
        );
    }
}

fn main() {
    let book: Book;

    {
        let mut a: String = String::new();
        let mut b: String = String::new();

        println!("Enter Book Name");
        stdin().read_line(&mut a).unwrap();

        println!("Enter Book Author Name");
        stdin().read_line(&mut b).unwrap();

        book = Book::new(&a, &b);
    }

    book.printInfo();
}
