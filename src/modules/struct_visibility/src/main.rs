mod my {
    // 公有结构体带公有字段
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // 公有结构体带私有字段
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // 公有构造方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    let open_box = my::OpenBox { contents: "public information" };

    println!("The open box contains: {}", open_box.contents);

    let _closed_box = my::ClosedBox::new("classified information");

    // 无法调用，contents不可见
    //println!("The closed box contains: {}", _closed_box.contents);
}
