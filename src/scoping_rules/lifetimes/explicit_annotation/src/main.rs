// 接受两个 i32的引用，分别具有不同的生命周期
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// 不带参数，不过有生命周期参数 `'a`
fn failed_borrow<'a>() {
    let _x = 12;

    // Error _x 生命周期短于 'a
    // let y: &'a i32 = &_x;
}

fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();
}
