use std::str::FromStr;

#[test]
fn matches() {
    let num = 3;
    match num {
        0 => println!("0"),
        1 | 2 => println!("1"), // 几个值
        3..14 => println!("3"), // 范围 3 <= ? < 14
        _ => println!("other"),
    }

    // 接收返回值

    let ok = true;
    let num = match ok {
        false => 0,
        true => 1,
    };
    println!("num {}", num); // num 1
}

// 数组、切片、元组类似
#[test]
fn match_tuple() {
    let triple = (0, -2, 3);
    match triple {
        (0, y, z) => println!("命中条件（首位 = 0）第一位必须是 `0`, 把第 2 绑定 `y`: {}, 把第 3 绑定 `z`: {}", y, z),
        (1, ..) => println!("命中条件（首位 = 1） 第一是 1， 后面的全部不管 `1`, `2`"),
        (.., 2) => println!("命中条件（末位 = 2）`2`, `3`"),
        (3, .., 4) => println!("命中条件（首位=3 且末位=4） 第一位 3、最后一位 4，中间全不管 `3`, `4`, `5`"),
        _ => println!("other"),
    }
}


#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
}

#[test]
fn match_enum() {
    let color = Color::RGB(255, 0, 0);

    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        Color::RGB(r, g, b) => println!("r: {}, g: {}, b: {}", r, g, b),
        Color::HSV(h, s, v) => println!("h: {}, s: {}, v: {}", h, s, v),
    }
    // output:
    // r: 255, g: 0, b: 0
}

struct Foo {
    x: (i32, i32),
    y: i32,
}

#[test]
fn match_struct() {
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x 第一个元素是 1, x: {}, y: {}", b, y),
        Foo { y:2, x:i} => println!("y 为 2，i = {:?}", i), // 顺序并不重要
        Foo { y, .. } => println!("y = {}", y), // 不关心 x 的值
    }
    // output:
    // x 第一个元素是 1, x: 2, y: 3
}

// 守卫模式
#[test]
fn match_guard() {
    let num: u8 = 4;
    match num {
        i if i < 5 => println!("i = {}", i),
        i if i > 5 => println!("i = {}", i),
        _ => println!("i = {}", num),
    }
    // i = 4
}

fn age() -> i32 {
    18
}

// @ 绑定值到 n
#[test]
fn match_bind() {
    match age() {
        0 => println!("没有生日"),
        n @ 1 ..10 => println!("我是 {} 岁的儿童", n),
        n @ 13 ..=19 => println!("我是 {} 岁的青少年", n),
        n @ (10 | 11 | 20 | 12) => println!("指定多个值 {}", n),
        n => println!("{}", n)
    }
    // 我是 18 岁的青少年
}

// if let 是 match 的语法糖, 少写其他case
#[test]
fn match_if_let() {
    let num = Some(10);
    /*
        match num {
            Some(i) => println!("{}", i),
            _ => {}
        }
    */

    if let Some(i) = num {
        println!("{}", i);
    }
}

// let else 匹配 支持异常处理
#[test]
fn let_else() {
    let count_str = "234";

    let count = if let Ok(count) = u64::from_str(count_str) {
        count
    } else {
        panic!("无法解析整数：'{count_str}'")
    };

    println!("{}", count);
}