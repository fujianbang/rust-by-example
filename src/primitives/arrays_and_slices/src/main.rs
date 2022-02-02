use std::mem;

// 借用
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 固定大小数组
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 自动初始化数组元素
    let ys: [i32; 500] = [0; 500];

    // 数组索引
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // len() 函数打印数组长度
    println!("number of elements in array: {}", xs.len());

    // 数组是由栈管理内存分配
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以通过slice切片被自动 借用
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // 异常，数组越界
    // println!("{}", xs[5]);
}
