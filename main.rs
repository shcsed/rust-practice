fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let rect1 = Rectangle{width: 30, height: 50};
    let rect2 = Rectangle{width: 10, height: 20};
    let rect3 = Rectangle{width: 30, height: 50};
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    let square = Rectangle::square(15);
    println!("see square: {square:?}")
}
