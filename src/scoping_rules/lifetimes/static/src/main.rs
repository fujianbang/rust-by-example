static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
    &NUM
}

fn main() {
    {
        let static_string = "I'm in read-only memory";
        println!("static_string: {}", static_string);

        // static_string 离开作用域，引用不能再使用，但是数据仍然存在二进制文件
    }
    {
        let lifetime_num = 9;

        // 对 NUM 的引用被强制转换成 lifetime_num 的生命周期
        let coerced_static = coerce_static(&lifetime_num);

        println!("coerced_static: {}", coerced_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
