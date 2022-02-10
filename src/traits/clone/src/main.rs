// 不包含资源的 单元结构体
#[derive(Debug, Clone, Copy)]
struct Nil;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let nil = Nil;
    // 复制 Nil，没有资源用于移动（move）
    let copied_nil = nil;

    println!("original: {:?}", nil);
    println!("copy: {:?}", copied_nil);


    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);

    let moved_pair = pair;
    println!("original: {:?}", moved_pair);

    // Error pair所有权已经转移
    // println!("original: {:?}", pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);

    // Error moved_pair已经drop()
    // println!("original: {:?}", moved_pair);

    println!("clone: {:?}", cloned_pair);
}
