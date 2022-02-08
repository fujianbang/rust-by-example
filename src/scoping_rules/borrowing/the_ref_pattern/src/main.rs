#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';

    // 以下两者等价
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    let _copy_of_x = {
        let Point { x: ref ref_to_x, y: _ } = point;

        // 返回一个 `point` 的 `x` 的拷贝
        *ref_to_x
    };

    let mut mutable_point = point;

    {
        let Point { x: _, y: ref mut mut_ref_to_y } = mutable_point;

        // 通过可变引用改变字段 `y`
        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})", mutable_point.x, mutable_point.y);

    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);

    {
        // 解构 `mutable_tuple` 来改变 `last` 的值
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_tuple);
}
