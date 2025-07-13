#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    publication_year: u16,
}

fn describe_book(book: &Book) -> String {
    format!(
        "'{}' by {}, published in {}.",
        book.title, book.author, book.publication_year
    )
}

fn main() {
    let book1 = Book {
        title: "The Great Gatsby".to_string(),
        author: "F. Scott Fitzgerald".to_string(),
        publication_year: 1925,
    };
    let book2 = Book {
        title: "To Kill a Mockingbird".to_string(),
        author: "Harper Lee".to_string(),
        publication_year: 1960,
    };
    println!("{}", describe_book(&book1));
    println!("{}", describe_book(&book2));
}
