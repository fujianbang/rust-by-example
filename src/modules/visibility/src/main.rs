mod my_mod {
    fn private_function() {
        // 默认：私有可见性
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰语更改函数可见性
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 同一模块中，可以访问其他项（即便其他项为私有，但mod内可见）
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        // #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 给定路径中可见
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
        }

        // 仅在当前模块中可见
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // 仅在父模块中可见
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // 尽在当前crate中可见
    pub(crate) fn public_function_in_ctate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 模块机制有助于消除同名函数的歧义
    function();
    my_mod::function();

    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 可以在统一crate任何地方访问
    my_mod::public_function_in_ctate();
}
