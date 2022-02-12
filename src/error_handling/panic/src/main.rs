extern crate core;

fn drink(beverage: &str) {
    if beverage == "lemonde" {
        panic!("AAAaaaaa!!!!");
    }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
}
