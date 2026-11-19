// 生命周期

#[test]
fn scope_borrow() {
    let i = 3; // i 生命周期 start
    {
        let j = &i; // j 生命周期 start
        println!("i={}, j={}", i, j); // i=3, j=3
        // j 生命周期 end
    }
    {
        let k = &i; // k 生命周期 start
        println!("k={}", k); // k=3
        // k 生命周期 end
    }
    // i 生命周期 end
}

// -----------------------------
// 显式注解

// 'a 和 'b 的生命周期至少和 print_refs 一样长
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

#[test]
fn annotations() {
    let (four, nine) = (4, 9);
    // 将两个变量借用，然后传递给函数
    print_refs(&four, &nine); // x is 4 and y is 9
}