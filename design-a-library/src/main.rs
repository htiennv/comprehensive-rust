#![allow(unused_variables, dead_code)]

struct Book {
    title: String,
    year: i16,
}

struct Library {
    books: Vec<Book>,
}

impl Book {
    fn new(title: &str, year: i16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.year)
    }
}

impl Library {
    fn new() -> Library {
        Library { books: Vec::new() }
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
        self.books.is_empty()
    }

    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn print_books(&self) {
        for book in &self.books {
            println!("{}", book);
        }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let book = self.books.iter().min_by(|x, y| x.year.cmp(&y.year));
        book
    }
}

fn main() {
    println!("Hello, world!");

    let mut library = Library::new();

    println!("Our library is empty: {}", library.is_empty());

    library.add_book(Book::new("Go in Action", 1998));
    library.add_book(Book::new("Distributed system", 1973));

    library.print_books();

    match library.oldest_book() {
        Some(book) => println!("My oldest book is {}", book),
        None => println!("My lirary is empty"),
    }

    println!("Nuber books on library: {}", library.len());
}
