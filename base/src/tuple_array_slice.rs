
// ----------------------
// 元组

#[derive(Debug)]
struct Matrix(f32, f32, f32);

#[test]
fn tuple() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64);
    let tuple_of_tuples = ((1u8, 2u16, 2u32), 3u64);
    println!("tuple {:?}", long_tuple);
    println!("tuple of tuples {:?}", tuple_of_tuples);

    let m = Matrix(1.1, 1.2, 2.2);
    println!("matrix {:?}, m.0 {}, m.1 {} m.2 {}", m, m.0, m.1, m.2);
}

// ----------------------
// 数组（固定大小）
#[test]
fn array() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2:[char;4] = ['a', 'b', 'c', 'd'];

    println!("arr {:?}", arr1);
    println!("arr2 len {}", arr2.len());

    // arr1.map()
}


// ----------------------
// 切片

#[test]
fn slice() {
    let xs = [1, 2, 3, 4, 5];
    let ys = [0; 500]; // 填充 500 个0
    analyze_slice(&xs); // 1, 5
    // 含大不含小，end > 1
    analyze_slice(&ys[1..4]); // 0, 3

    // .get() 安全访问数组 返回一个 Option
    for i in 0 .. xs.len() + 1 { // +1 超出了访问范围
        match xs.get(i) {
            Some(xval) => println!("{}, {}",i, xval),
            None => println!("{} not found", i),
        }
    }
}

fn analyze_slice(slice: &[i32]) {
    println!("slice {:?}", slice[0]);
    println!("slice len {}", slice.len());
}