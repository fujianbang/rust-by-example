struct A;

struct S(A);

// 泛型类型
struct SGen<T>(T);

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // 显式指定类型参数 `char`
    generic::<char>(SGen('a'));

    // 隐式指定类型参数 `char`
    generic(SGen('c'));
}
