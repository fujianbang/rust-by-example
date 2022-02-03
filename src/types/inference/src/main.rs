fn main() {
    // elem 自动断言为 u8 类型
    let elem = 5u8;

    // 创建一个空的vector（一个可增长数组）
    let mut vec = Vec::new();

    // elem 插入到vec，编译器可以自动推断Vec<?>为Vec<u8>
    vec.push(elem);

    println!("{:?}", vec);
}
