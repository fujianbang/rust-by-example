fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // let 语句可以拆解绑定 tuple 的变量
    let (integer, boolean) = pair;

    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // tuple 可以包含不同类型数据
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 通过 0.1.2.3... 索引获取tuple值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // tuple 可以为 tuple 值
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    // 创建一个元素的tuple，`,`逗号不可遗漏
    println!("one element tuple: {:?}", (5u32, ));
    // 没有书写`,`逗号，则(5u32)为u32类型的数据类型，而不是tuple
    println!("just an integer: {:?}", (5u32));

    // tuple 声明
    let tuple = (1, "hello", 4.5, true);
    // 解构
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}
