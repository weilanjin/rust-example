// cargo test -p base              # base 里所有测试
// cargo test -p base println      # 只跑名字含 println 的
// cargo test -p base -- --nocapture   # 全部测试，且显示打印

use std::fmt;
use std::fmt::Formatter;

// =========================
//  1. 常规输出

#[test]
fn println() {
    println!("Hello, {}!", "Rust"); // {} 这个就会一个槽，目标打印实现 Display trait
    println!("{}", 3.14);
    println!("{}", true);
    println!("{:?}", vec![1, 2, 3]);

    let s = "lance";
    println!("{}", s); // 不带引号（Display 风格）
    println!("{:?}", s); // 带引号 （Debug 风格） #[derive(Debug)]

    println!("{1}, {0}, {0}, {1}", "sql", "拼接");
    // SELECT username,age, FROM users
    println!("SELECT {n},{a} FROM {t}", n="username", a="age", t="users" );

    /*
        { }        → Display
        { :? }     → Debug
        { :#? }    → Debug + alternate(美化多行)
        { :x }     → LowerHex（Display 家族的另一套 trait）
        { :x? }    → Debug 但整数用十六进制
        { :>10? }  → Debug 且右对齐宽度 10
    */
}

// =========================
//  2. 对齐

#[test]
fn printin_align() {
    println!("\"{:>8}\"", 42); // "      42"
    println!("\"{:<8}\"", 42); // "42      "
    println!("\"{:^8}\"", 42); // "   42   "
    println!("\"{:*^8}\"", 42); // "***42***"
    println!("\"{:0>5}\"", 42); // "00042"

    println!("-----------");

    println!("{:>w$}", 42, w = 8); // 命名参数作宽度
    println!("{:>1$}", 42, 8); // 位置 1 作宽度
    println!("{:.p$}", 3.14159, p = 3); // 3.142
    println!("{:.*}", 3, 3.14159); // 3.142（精度由下一个参数给）
}

// =========================
//  3. 打印对象

#[derive(Debug)] // 没有强烈的自定义需求推荐这个
struct Point {
    x: i32,
    y: i32,
}

// struct 加了字段之后就会漏
impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

const _: Point = Point { x: 3, y: 9 };

#[test]
fn printin_struct() {
    let p = Point { x: 3, y: 9 };

    println!("{}", p); // (1, 2)
    println!("{:?}", p); // Point { x: 3, y: 9 }
    println!("{:#?}", p);
    /*
        Point {
            x: 3,
            y: 9,
        }
    */
}
