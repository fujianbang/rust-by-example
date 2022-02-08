struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    // 创建两个point不可变引用
    let borrowed_point = &point;
    let another_borrow = &point;

    // Error 当前存在不可变引用，故不可创建可变借用
    // let mutable_borrow = &mut point;

    // 被借用的值重新使用
    println!("Point has coordinates: ({}, {}, {})",
             borrowed_point.x, another_borrow.y, point.z);

    // 后续不再有使用 `borrowed_point` 和 `another_borrow` ，故point可以使用可变引用重新借用
    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    println!("Point has coordinates: ({}, {}, {})",
             mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    let new_borrowed_point = &point;
    println!("Point now has coordinates: ({}, {}, {})",
             new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}
