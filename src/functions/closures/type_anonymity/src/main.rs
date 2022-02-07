fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // 捕获 x 到匿名类型中
    let print = || println!("{}", x);

    apply(print);
}
