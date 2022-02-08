use std::fmt::Debug;

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// 约束 `T` 必须实现 `Debug`
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` 必须实现 `HasArea`
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));
}
