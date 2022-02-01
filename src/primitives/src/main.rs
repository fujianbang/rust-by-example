fn main() {
    // 类型注释
    let logical: bool = true;

    let a_float: f64 = 1.0; // 常规注释
    let an_integer = 5i32; // 后缀注释（i32）

    let default_float = 3.0;
    let default_integer = 7;

    // 自动类型推断
    let mut inferred_type = 12;
    inferred_type = 4294967296i64; // 后缀注释（i64）

    // 可变变量值可以被修改
    let mut mutable = 12;
    mutable = 21;

    // 不可用！变量类型不可变
    // mutable = true

    // 变量可以通过shadowing覆盖重新生命（类型可变）
    let mutable = true;
}
