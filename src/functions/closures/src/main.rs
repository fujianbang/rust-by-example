fn main() {
    // 闭包函数
    fn function(i: i32) -> i32 { i + 1 }

    // 匿名闭包 `||`中为函数入参
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    // 匿名未声明参数类型闭包，函数调用时可自动推断
    // 未声明返回参数，自动推断
    let closure_inferred = |i| i + 1;

    let i = 1;

    // 调用闭包函数
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 简单闭包函数，未声明返回值，自动推断 `-> ()`
    let one = || 1;
    println!("closure returning one: {}", one());
}
