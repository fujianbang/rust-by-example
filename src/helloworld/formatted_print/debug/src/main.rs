// 不能被 `fmt::Display` 或 `fmt::Debug` 打印
#[allow(dead_code)] // 禁用警告
struct UnPrintable(i32);

// 可以被 `fmt::Debug` 打印
#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // 此处 `{:?}` 和 `{}` 作用类似
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} nanem.",
             "Slater",
             "Christian",
             actor = "actor's");

    // 打印 `Structure`
    println!("Now {:?} will print!", Structure(3));

    // 打印 `Deep`
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    #[allow(dead_code)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty printing
    println!("{:#?}", peter);
}
