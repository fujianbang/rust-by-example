#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // Error 不能进行 隐式转换
    // let integer: u8 = decimal;

    // 显式转换
    let integer = decimal as u8;
    let character = integer as char;

    // float类型不能直接转换为char
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 1000 转换为 u16
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 溢出 1000 - (256 * 3) = 232
    println!("1000 as a u8 is : {}", 1000 as u8);

    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 取模运算，结果和上诉类型转换结果相同
    println!("1000 mod 256 is : {}", 1000 % 256);


    // 正常转换
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 as u8 -> 128, i8[-128~127]
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    println!(" 232 as a i8 is : {}", 232 as i8);

    // Rust 1.45之后，as 语句把float转换为int，
    // 如果float值超过或者小于数据类型边界，转换后当值直接为边界值
    println!("300.0 is {}", 300.0_f32 as u8);
    println!("-100.0 as u8 is {}", -100.0_f32 as u8);
    println!("nan as u8 is {}", f32::NAN as u8);

    // 使用unsafe有一定运行时开销，转换数值溢出返回不可靠的值
    unsafe {
        println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
    }
}