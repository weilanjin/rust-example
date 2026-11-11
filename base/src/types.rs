// 别名
type NanoSecond = u64;
type Inch = u64;

#[test]
fn alias() {
    let ns: NanoSecond = 1;
    let inch: Inch = 2;

    println!("NanoSecond {}, Inch {}", ns, inch);
}

#[test]
fn primitive_types() {
    // 标准数据类型
    // i8 i16 i32 i64 i128 isize (是跟平台走的有符号整数)
    // u8 u16 u32 u64 u128 usize
    // f32 f64
    // char (Unicode) 'a' 'b' 'A'
    // bool false true

    // 复杂类型
    // 数组 【1，2，3】
    // 元组 （1, true）

    let n1 = 1;
    let age: u8 = 18;
    let port = 3306u16;

    let mut n2 = 2; // 自动推断 u8
    n2 += age;

    let is_ok = n2 > age; // bool

    let arr = [1, 3, 4, 5]; // [i32, 4] i32 类型，长度 4
    println!("arr length {}", arr.len());

    let tuple = (n1, n2, port, is_ok);

    println!("tuple {:?}", tuple); // tuple (1, 20, 3306, true)
}

// 类型转换
#[test]
fn type_as() {
    let decimal = 65.4321_f32;
    let integer = decimal as u8; // 显式转换

    println!("{:?}", integer); // 65

    // 浮点数不能直接转成字符
    // let character = decimal as char;
}