fn main() {
    let i = 3;

    {
        let borrow1 = &i; // borrow1 生命周期开始
        println!("borrow1: {}", borrow1);

        // borrow1 生命周期结束
    }

    {
        let borrow2 = &i;

        println!("borrow2: {}", borrow2);
    }


    // i 变量生命周期结束
}
