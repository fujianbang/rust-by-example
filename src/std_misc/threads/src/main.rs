use std::thread;

static NTHREAD: i32 = 10;

fn main() {
    let mut children = vec![];

    for i in 0..NTHREAD {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i)
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
