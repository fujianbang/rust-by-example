fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3{
            println!("three");

            // 退出当前循环
            continue;
        }

        println!("{}", count);

        if count == 5{
            println!("Ok, that's enough");

            // 退出循环
            break;
        }
    }
}
