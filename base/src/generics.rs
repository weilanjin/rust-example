use std::fmt::{Debug, Display};

#[allow(dead_code)]

// 泛型
fn foo<T>(arg: T) {}

struct SingleGen<T>(T);

impl<T> SingleGen<T> {}

trait DoubleGen<T> {
    fn double_gen(self) -> T;
}

struct S<T: Display>(T);

fn compare_prints<T: Debug + Display>(t: &T) {}
fn compare_types<T: Debug, U: Display>(t: &T, u: &U) {}

trait Contains {
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}

struct Container(i32, i32);

impl Contains for Container {
    type A = i32;
    type B = i32;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool {
        (&self.0 == a) && (&self.1 == b)
    }
}