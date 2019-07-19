trait Printable {
    fn print(&self);
}

struct Book {
    number: i32,
}
impl Book {
    fn print(&self) {
        println!("Book={}", self.number);
    }

    fn new(n: i32) -> Self {
        Book { number: n }
    }
}
impl Printable for Book {
    fn print(&self) {
        println!("Book={}", self.number);
    }
}

struct Note<T>
where
    T: Printable,
{
    arrays: Vec<T>,
}

impl<T> Note<T>
where
    T: Printable,
{
    pub fn print(&self) {
        for a in &self.arrays {
            a.print();
        }
    }

    pub fn new() -> Self {
        let v: Vec<T> = vec![];
        Note { arrays: v }
    }
}

fn main() {
    let mut a: Note<Book> = Note::new();
    a.arrays.push(Book::new(200));
    a.arrays.push(Book::new(1000));
    a.arrays.push(Book::new(5));
    a.print();
}
