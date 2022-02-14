use std::mem;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn origin() -> Point {
    Point { x: 0.0, y: 0.0 }
}

fn boxed_origin() -> Box<Point> {
    // 堆上分配一个Point
    Box::new(Point { x: 0.0, y: 0.0 })
}

fn main() {
    // 栈分配
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        p1: origin(),
        p2: Point { x: 3.0, y: 4.0 },
    };

    // 堆分配
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        p1: origin(),
        p2: origin(),
    });

    // 函数输出装箱
    let boxed_point: Box<Point> = Box::new(origin());

    // 两层装箱
    let box_in_a_box: Box<Box<Point>> = Box::new(boxed_origin());

    println!("Point occupies {} bytess in the stack", mem::size_of_val(&point));
    println!("Rectangle occupies {} bytes in zhe stack", mem::size_of_val(&rectangle));

    // 8 bytes 是指针宽度
    println!("Boxed point occupies {} bytes in the stack", mem::size_of_val(&boxed_point));
    println!("Boxed rectangle occupies {} bytes in the stack", mem::size_of_val(&boxed_rectangle));
    println!("Boxed box occupies {} bytes in the stack", mem::size_of_val(&box_in_a_box));

    let unboxed_point: Point = *boxed_point;
    println!("Unboxed point occupies {} bytes in the stack", mem::size_of_val(&unboxed_point));
}

