// 取得box所有权并销毁
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// 借用一个i32
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // 借用了box，但没有取得所有权
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // 获取 box 中数据的引用
        let _ref_to_i32: &i32 = &boxed_i32;

        // ERROR 当前作用域外借用，不能将其销毁
        // eat_box_i32(boxed_i32);

        borrow_i32(_ref_to_i32);
        // _ref_to_i32 离开作用域不再被借用
    }

    eat_box_i32(boxed_i32);
}
