struct Book {
    title: String,
    author: String,
    condition: String,
}

struct Person {
    name: String,
    books: Vec<Book>, //a vec is a growable array
}

fn give_book_to_person(b: Book, p: &mut Person) {
    //binding: reference
    /*
    need an mutable reference in order to mutate
    p.books
    */
    p.books.push(b);
}

fn lend_book_to_person(b: &Book, p: &Person)){
    /*
    read only borrowing
    function receives reference to book

    */




}
