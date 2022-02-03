fn main() {
    let a_binding;

    {
        let x = 2;

        // 初始化binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // 未初始化不可使用
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);
}
