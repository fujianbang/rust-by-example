fn function() {
    println!("called `function()`");
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call() {
        print!("called `my::indirect_all()`, that\n> ");

        // = my::function
        self::function();
        function();

        self::cool::function();

        // 父作用域，即`my`外面，self的上一层
        super::function();
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
