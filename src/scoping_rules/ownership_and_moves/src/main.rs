// 函数取得堆分配的内存所有权
fn destroy_box(c: Box<i32>) {
    println!("Destroy a box that contains {}", c);

    // `c` 被销毁，内存释放
}

fn main() {
    // 栈分配的整型u32
    let x = 5u32;

    // `x` 复制到 `y`，没有资源移动
    let y = x;

    // 两个值自由使用
    println!("x is {}, and y is {}", x, y);


    // `a` 指向堆分配堆整型的指针
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // 所有权移动，`a` 释放
    let b = a;

    destroy_box(b);
}
