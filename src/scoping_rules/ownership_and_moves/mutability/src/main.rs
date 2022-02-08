fn main() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // 所有权改变，数据可变性也发生了改变
    let mut mutable_box = immutable_box;

    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);
}
