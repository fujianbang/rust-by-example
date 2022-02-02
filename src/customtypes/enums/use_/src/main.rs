#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 使用 use 则不需要在使用的时候手动标明scope
    use crate::Status::{Poor, Rich};
    // *：导入当前`Work`下所有name
    use crate::Work::*;

    let status = Poor; // 等效 Status::Poor
    let work = Civilian; // 等效 Work::Civilian

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}
