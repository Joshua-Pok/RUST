pub struct Book {
    pub title: String,
    pub author: String,
    pub condition: String,
}

pub struct Person {
    pub name: String,
    pub books: Vec<Book>, //a vec is a growable array
}

pub fn give_book_to_person(b: Book, p: &mut Person) {
    //binding: reference
    /*
    need an mutable reference in order to mutate
    p.books
    */
    p.books.push(b);
}

pub fn lend_book_to_person(b: &Book) {
    /*
    read only borrowing
    function receives reference to book

    */

    format!("Title: {}, Condition: {}", b.title, b.condition);
}

pub fn update_book_condition(b: &mut Book, new_condition: String) {
    b.condition = new_condition
}
