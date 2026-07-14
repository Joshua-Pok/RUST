use crate::book::{Book, give_book_to_person, lend_book_to_person, update_book_condition};

mod book;

fn main() {
    let mut book = book::Book {
        title: "Rust".to_string(),
        author: "Joshua".to_string(),
        condition: "brand_new".to_string(),
    };

    let book2 = book::Book {
        title: "Golang".to_string(),
        author: "tom".to_string(),
        condition: "worn out".to_string(),
    };

    let mut p = book::Person {
        // we can only take mutable references from mutable
        // bindings
        name: "Joshua".to_string(),
        books: vec![],
    };

    give_book_to_person(book, &mut p);
    lend_book_to_person(&book2);

    update_book_condition(mut book, "worn out".to_string());

    println!("Hello, world!");
}
