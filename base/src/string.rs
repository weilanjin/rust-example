use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[test]
fn to_string() {
    let circle = Circle { radius: 6 };
    let radius = "        3";
    let circle1: Circle = radius.parse().unwrap();

    println!("{}", circle.to_string());
    print!("{:?}", circle1);
}

#[derive(Debug)]
struct Circle {
    radius: i32, // 半径
}

// to string
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

// parse string
impl FromStr for Circle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse() {
            Ok(radius) => Ok(Circle { radius }),
            Err(e) => Err(e),
        }
    }
}
