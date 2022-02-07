// 此声明会查找 my.rs 或者 my/mod.rs 文件，并将文件内容放到 `my` 模块中
mod my;

fn function(){
   println!("called `function`") ;
}

fn main() {
    my::function();

    function();

    my::indirect_access();
}