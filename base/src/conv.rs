// From 和 Into 是相反的操作
// 如果实现了 From 就会自动实现了 Into
// 如果实现了 Into 也没有 From

// ----------------------------------------
// From 允许一个类型定义如何从另一个类似创建自身

#[test]
fn from() {
    let str = "helle";
    let s = String::from(str);

    println!("s {}", s);

    let num = Number::from(32);

    println!("num: {}", num.value);
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// ----------------------------------------
// Into 如何将一个类似转换成另外类型

#[derive(Debug)]
struct Number1 {
    value: i32,
}
impl Into<Number1> for i32 {
    fn into(self) -> Number1 {
        Number1 { value: self }
    }
}

#[test]
fn into() {
    let int = 5;

    let num: Number1 = int.into();
    println!("num: {}", num.value);
}

// ----------------------------------------
// TryFrom TryInto 尝试转换

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

#[test]
fn try_from_into() {
    assert_eq!(EvenNumber::try_from(4), Ok(EvenNumber(4)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 4i32.try_into();
    let result1: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Ok(EvenNumber(4)));
    assert_eq!(result1, Err(()));
}
