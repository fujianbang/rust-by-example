struct Cardinal;

struct BlueJay;

struct Turkey;

trait Red {}

trait Blue {}

// 不包含任何功能的空 trait
impl Red for Cardinal {}

impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str { "red" }

fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
}
