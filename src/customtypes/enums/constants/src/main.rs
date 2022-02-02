// 具有'static 生命周期，可以是可变的变量
static LANGUAGE: &str = "Rust";
// 常量，数值不可变
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" })

    // THRESHOLD = 5
}
