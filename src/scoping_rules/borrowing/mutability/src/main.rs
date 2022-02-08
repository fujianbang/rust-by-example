#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // &'static 堆分配在只读内存区的字符串的引用
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}


fn main() {
    // 创建一个不可变的Book实例
    let immutabook = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    // 创建一个可变拷贝（因为实现了Copy trait，所以没有所有权转移，而是拷贝）
    let mut mutabook = immutabook;

    // 不可变地借用一个不可变对象
    borrow_book(&immutabook);

    // 不可变地借用一个可变对象
    borrow_book(&mutabook);

    // 可变地借用一个可变对象
    new_edition(&mut mutabook);

    // Error 不能可变地借用不可变对象
    // new_edition(&mut immutabook);
}
