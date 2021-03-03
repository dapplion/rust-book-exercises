struct Point<T>
where
    T: std::ops::Mul<T, Output = T> + std::marker::Copy,
{
    x: T,
    y: T,
}

impl<T> Point<T>
where
    T: std::ops::Mul<T, Output = T> + std::marker::Copy,
{
    fn x(&self) -> &T {
        &self.x
    }
    fn area(&self) -> T {
        self.x * self.y
    }
}

fn main() {
    let pint = Point { x: 5, y: 10 };
    let pflt = Point { x: 1.0, y: 4.0 };
    println!("px = {}", pint.x());
    println!("px = {}", pflt.x());
}
