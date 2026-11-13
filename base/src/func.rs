// ----------------------------
// 闭包
// ｜x｜ x + val
// 能够捕获外部环境变量的函数

#[test]
fn fn_closures() {
    let outer_var = 42;

    // 无法捕获 `fn` 条目中的动态环境
    // fn func(i: i32) -> i32 { i * outer_var }

    let cf = |i|  i + outer_var;
    println!("{}", cf(42));
}

// ----------------------------
// capture 捕获
// 通过引用：&T
// 通过可变引用：&mut T
// 通过值：T

#[test]
fn capture() {
    let haystack = vec![1, 2, 3];

    // move 强制闭包获取捕获变量的所有权
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1)); // true
    println!("{}", contains(&4)); // false
}

// -----------------------------
// 作为输入参数
// Fn：闭包通过引用使用捕获的值（&T）
// FnMut：闭包通过可变引用使用捕获的值（&mut T）
// FnOnce：闭包通过值使用捕获的值（T）

// 这个函数接受闭包不返回任何值
fn apply<F>(f: F) where F: FnOnce() {
    f()
}

// 这个函数接受一个闭包并返回 i32
fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
    f(3)
}

#[test]
fn fn_in_args() {
    // to_owned 从借用的数据创建拥有所有权的数据
    let mut farewall = "goodbye".to_owned();
    let diary = || {
        farewall.push_str(" diary");
        // 手动调用 drop 强制 farewell 通过值捕获
        // 现在需要 FnOnce
        drop(farewall);
    };
    apply(diary); // 调用apply 闭包函数

    let double = |x| 2 * x;
    let three_mul = apply_to_3(double);
    println!("3 的两倍 {}", three_mul); // 3 的两倍 6
}

// -----------------------------
// 作为输入函数

fn call_me<F: Fn()>(f: F) {
    f();
}

// 定义一个满足闭包的函数
fn fn_closure() {
    println!("fn_closure");
}

#[test]
fn fn_in_fn() {
    let closure = || println!("I am a closure");
    call_me(closure); // I am a closure
    call_me(fn_closure); // fn_closure
}

// -----------------------------
// 作为输出参数
// impl Fn
// impl FnMut
// impl FnOnce

fn create_fn() -> impl Fn() {
    let test = "Fn".to_owned();
    move || println!("{} I am a closure", test)
}

fn create_fmmut() -> impl FnMut() {
    let test = "FnMut".to_owned();
    move || println!("{} I am a closure", test)
}

fn create_fnonce() -> impl FnOnce() {
    let test = "FnOnce".to_owned();
    move || println!("{} I am a closure", test)
}

#[test]
fn fn_out_args() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fmmut();
    let fn_once = create_fnonce();

    fn_plain(); // Fn I am a closure
    fn_mut(); // FnMut I am a closure
    fn_once(); // FnOnce I am a closure
}

// ------------------------------
// 高阶函数 Higher Order Fn
// 可以实现惰性执行

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

// ------------------------------
// 发散函数
// 永远返回
fn foo() -> ! {
    panic!("This function is not called");
}

// 这个不是发散 恰好有一个可能的值
fn some_fn() {
    ()
}