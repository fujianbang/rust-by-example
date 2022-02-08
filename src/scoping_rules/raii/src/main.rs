fn create_box() {
    let _box1 = Box::new(3i32);

    // _box1 销毁，释放内存
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn main() {
    // 堆上分配一个整型数据
    let _box2 = Box::new(5i32);

    // 嵌套作用域
    {
        let _box3 = Box::new(4i32);

        // _box3 内存释放
    }

    for _ in 0u32..1_00 {
        create_box()
    }

    // _box2 销毁，释放内存

    // 析构函数
    let x = ToDrop;
    println!("Made a ToDrop!");
}
