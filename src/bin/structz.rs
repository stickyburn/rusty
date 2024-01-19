#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, inner: &Rectangle) -> bool {
        self.area() > inner.area()
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let inner = Rectangle {
        width: 10,
        height: 10,
    };

    println!("Passed rectangle: {:#?}", rect);
    println!("Area can hold: {}", rect.can_hold(&inner));
}
