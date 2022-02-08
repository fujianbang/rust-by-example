struct A;

struct Single(A);

// 泛型定义
struct SingleGen<T>(T);

fn main() {
    let _s = Single(A);

    // 使用泛型
    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A); // 使用泛型<A>
    let _i32 = SingleGen(6); // 使用泛型<i32>
    let _char = SingleGen('a'); // 使用泛型<char>
}
