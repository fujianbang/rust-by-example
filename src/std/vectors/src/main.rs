fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("pushed 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs.len());

    // Error 不可变vector不可增长
    // collected_iterator.push(0);

    // len() 获取当前Vector的大小
    println!("Vector size: {}", xs.len());

    // 下标索引
    println!("Second a element: {}", xs[1]);

    // pop() 移除vector最后一个元素并将之返回
    println!("Pop last element: {:?}", xs.pop());

    // Error vector越界 panic
    // println!("Fourth element: {}", xs[3]);

    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3
    }
    println!("Updated vector: {:?}", xs);
}



