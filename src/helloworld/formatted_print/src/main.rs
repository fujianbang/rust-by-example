/*
 * format!: write formatted text to String
 * print!: same as format! but the text is printed to the console (io::stdout).
 * println!: same as print! but a newline is appended.
 * eprint!: same as format! but the text is printed to the standard error (io::stderr).
 * eprintln!: same as eprint!but a newline is appended.
 */
use std::fmt;

fn main() {
    // 基础用法
    println!("{} days", 31);

    // 指定序号，可重复用
    println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");

    // 指定命名参数
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jump over");

    // 格式化，例如`:b` 格式化为二进制
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // 通过:>在number前填充指定数量的空格
    println!("{number:>width$}", number = 1, width = 6);

    // 通过:0>在number前填充指定数量的0
    println!("{number:0>width$}", number = 1, width = 6);

    /* Activity */
    println!("My name is {0}, {1} {0}", "Bound", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    println!("This struct `{}` won't print...", Structure(3));

    let pi = 3.141592;
    println!("Pi is roughly {:.2}", pi)
}
