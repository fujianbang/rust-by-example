fn apply<F>(f: F)
    where
        F: FnOnce() {
    f();
}

// 闭包接收一个 i32 并返回一个 i32
fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    // 捕获两个变量 greeting（引用） / farewell（值）
    let diary = || {
        println!("I said {}.", greeting);

        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
