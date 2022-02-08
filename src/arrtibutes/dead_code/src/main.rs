fn used_function() {}

// 禁用警告之类的lint提醒
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}

fn main() {
    used_function();
}
