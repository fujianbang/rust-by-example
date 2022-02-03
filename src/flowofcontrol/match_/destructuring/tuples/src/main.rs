fn main() {
    let triple = (0, -2, 3);

    println!("Tell me about {:?}", triple);

    match triple {
        // 解构匹配，y/z自动为triple的第二/三个值
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        // 仅匹配第一位，`..` 不关心具体数值
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        // 其他情况
        _ => println!("It doesn't matter what they are"),
    }
}
