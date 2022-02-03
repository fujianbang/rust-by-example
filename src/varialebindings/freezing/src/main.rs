fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = _mutable_integer;

        // _mutable_integer 已经被重新定义，当前为不可变变量
        // _mutable_integer = 50;

        // 离开作用域，当前shadowing的_mutable_integer失效
    }

    _mutable_integer = 3;
}
