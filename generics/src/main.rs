use std::fmt::Display;

fn main() {
    let point1 = Point {
        x: 12,
        y: 13,
    };

    let point2 = Point {
        x: 12,
        y: 13,
    };

    let point3 = Point {
        x: point1,
        y: point2,
    };

    println!("{:#?}", point3);
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Display> ToString for Point<T> {
    fn to_string(&self) -> String {
        format!("{}{}", self.x, self.y)
    }
}
