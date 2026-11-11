#![allow(dead_code)]

enum Number {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char), // 按键
    Paste(String),  // 粘贴
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}", x, y);
        }
    }
}

#[test]
fn enums() {
    println!("zero is {}", Number::Zero as i32);
    println!("玫瑰的颜色是 #{:06x}", Color::Red as u32);

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 20 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed); // pressed 'x'
    inspect(pasted); // pasted "my text"
    inspect(click); // clicked at x=20, y=20
    inspect(load); // page loaded
    inspect(unload); // page unloaded
}
