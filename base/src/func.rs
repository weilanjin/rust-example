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

fn fn_in() {

}