// hide warnings for unused code
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 错误/哨兵/空实现
#[derive(Debug)]
struct Unit;
struct NotFound; // 没有字段可带，但类型有名有姓

fn find(id: u64) -> Result<u64, NotFound> {
    if id == 0 { Err(NotFound) } else { Ok(id) }
}

struct Pair(i32, f32);

// 结构体方法
impl Pair {
    // `self` 是 `self: Self` 的语法糖
    fn destroy(self) {
        println!("{} {}", self.0, self.1);
    }
}

#[test]
fn structures() {
    let name = String::from("John");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter); // Person { name: "John", age: 27 }

    let unit = Unit;
    let pair = Pair(1, 0.1);
    println!("Unit {:?}, pair {}", unit, pair.0); // Unit Unit, pair 1
}
