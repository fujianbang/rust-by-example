macro_rules! create_function {
    // 宏接受一个 `ident` 指示符表示的参数，并创建一个名为 `$func_name` 的函数
    // `ident` 指示符用于变量名或函数名
    ($func_name:ident) => (
        fn $func_name() {
            println!("You called {:?}()", stringify!($func_name))
        }
    )
}

create_function!(foo);
create_function!(bar);

macro_rules! print_result {
    // 接受一个 `expr` 类型的表达式，并将它作为字符串，连同其结果一起打印
    ($expression:expr) =>(
       println!("{:?} = {:?}",
           stringify!($expression), $expression)
    );
}

fn main() {
    foo();
    bar();

    print_result!(1u32 + 1);

    print_result!({
        let x = 1u32;

        x * x + 2 * x -1
    });
}
