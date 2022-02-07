fn main() {
    use std::mem;

    let color = String::from("green");

    // 闭包函数，打印 color: {}
    let print = || println!("`color`: {}", color);
    print();

    // color被不可变借用
    let _reborrow = &color;
    print();

    // 被允许的重新借用
    let _color_moved = color;

    let mut count = 0;
    // 闭包使count值增加
    // 调用闭包，变量变化意味着闭包内部发生了变化，因此闭包需要使可变的
    let mut inc = || {
        // 闭包借用count
        count += 1;
        println!("`count`: {}", count);
    };
    // 使用可变借用调用闭包
    inc();
    inc();

    // 闭包不再借用`&mut count`，因此可以重新被借用
    let _count_reborrowed = &mut count;

    // 不可复制类型（non-copy type）
    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();

    // 在 | 之前使用 `move` 会强制闭包取得被捕获变量的所有权
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // ERROR,haystack所有权已经被移动到闭包
    // println!("{}", haystack.len());
}
