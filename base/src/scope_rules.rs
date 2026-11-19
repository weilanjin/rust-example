// ---------------------
// 所有权
// 只有最后引用的，才获取数据的访问和释放权限
#[test]
fn owner_move() {
    let x = 5; // 栈 分配整数
    let y = x; // 将 x 复制到 y, 资源没有移动

    // 两个值可以独立使用
    println!("x = {}, y = {}", x, y); // x = 5, y = 5

    let a = Box::new(5); // 指向堆分配的整数指针

    // 把指向 5 的指针拷贝给 b
    // 此时 5 同时被 a, b 引用
    // 但是 b 现在拥有它，a 不再拥有堆内存，无法访问数据
    let b = a;
    // println!("a = {}, b = {}", a, b);

    destroy_box(b);

    // 此时堆内存已被释放，这个操作会导致解引用已释放的内存
    // println!("b = {}", b);
}

fn destroy_box(c : Box<i32>) {
    println!("{:?}", c); // 5
    // c 被销毁，内存被释放
}

// ----------------------------
// 借用 borrow
// 大多数情况下，希望通过访问数据而不是获取数据的所有权
// 通过引用 (&T) 传递，而不是值（T）传递
// 当存在指向一个对象的引用时，该对象就不能被销毁

// 此函数正在获取一个盒子的所有权并销毁它
fn owner_box_32(b : Box<i32>) {
    println!("获取所有权 {}", b);
}

// 借用一个 i32
fn borrow_box_32(p : &i32) {
    println!("借用 {}", p);
}

#[test]
fn borrow() {
    let a = Box::new(5);
    let pint = 3;

    owner_box_32(a); // 获取所有权 5
    borrow_box_32(&pint); // 借用 3
}

#[derive(Clone, Copy)]
struct Point{x:i32, y:i32, z:i32}

// 借用别名
#[test]
fn borrow_alias() {
    let p = Point{x:12, y:34, z:35};
    let p1 = &p;
    let p2 = &p;

    println!("x = {}, y = {}, z = {}", p.x, p1.y, p2.z); // x = 12, y = 34, z = 35
}

// ref 关键字来获取结构体或元组字段的引用
#[test]
fn borrow_ref() {
    let c = 'Q';

    // 赋值左侧的 ref 的借用等同右侧的 & 借用
    let ref ref_c1 = c;
    let ref_c2 = &c;
    println!("ref_c1 = {}, ref_c12 = {}", ref_c1, ref_c2);
}

// ----------------------------
// 可变性

#[derive(Clone, Copy)]
struct Book {
    // 'static 是 rust 最长的生命周期
    // 这个数据从程序启动就存在，直到程序运行结束，才会释放
    title: &'static str,
    year: u32,
}

// 不可变借用
fn borrow_book(book : &Book) {
    println!("不可变，借用《{}》{}", book.title, book.year);
}

// 可变借用
fn borrow_edit(book : &mut Book) {
    book.year = 2027;
    println!("可变，借用《{}》{}", book.title, book.year);
}

#[test]
fn mutable() {
    let boxs = Box::new(5);
    // *boxs = 4; // boxs 不可变
    let mut b = boxs;
    *b = 6; // 可以修改 box 的内容
    println!("{:?}", b); // 6

    // 不可变的 book 实例
    let immut_book = Book {
        title: "Rust 入门",
        year: 2026,
    };


    // Clone 管“怎么复制”, Copy 管"复制时会不会移动（move)）"
    // #[derive(Clone)] 可以显式 .clone() 复制，赋值/传参时仍然会 move
    // #[derive(Clone, Copy)] 赋值/传参时自动按位复制。不 move

    // 创建一个可变的副本
    let mut mut_book = immut_book;
    borrow_edit( &mut mut_book );
    borrow_book(&immut_book);
}

// ----------------------------
// 部分移动

struct Person {
    name: String,
    age: Box<u8>,
}

#[test]
fn partial_moves() {
    let person = Person{
        name: String::from("John"),
        age: Box::new(20)
    };

    // 结构
    let Person{ name, ref age } = person;
    println!("name = {}, age = {}", name, age); // name = John, age = 20
    // println!("person {:?}", person); // person 发生了移动
    println!("age = {}", person.age); // 可以使用，因为它没有被移动
}