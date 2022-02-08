struct Empty;

struct Null;

// `T` 的泛型 trait
trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    // 此方法获得两个传入参数的所有权，并释放
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    // 释放empty和null
    empty.double_drop(null);
}
