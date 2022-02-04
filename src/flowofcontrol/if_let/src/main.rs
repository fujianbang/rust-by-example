enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn main() {
    // 全都是 Option<i32> 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    // 通过 if let 来处理只匹配一个模式的值而忽略其他模式的情况
    // 省去match语句
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emotion {
        println!("Matched {:?}!", i);
        // 解构失败继续往后判断
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emotion :)!");
    }

    // example
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a { // 模式匹配
        println!("a is foobar");
    }

    if let Foo::Bar = b { // 模式匹配
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred({})", value);
    }
}
