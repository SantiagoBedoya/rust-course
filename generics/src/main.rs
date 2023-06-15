#![allow(dead_code)]

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }
}

impl<T, U> std::fmt::Display for Point<T, U>
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}

impl<T, U> std::ops::Add for Point<T, U> 
where
    T: std::ops::Add<Output = T>,
    U: std::ops::Add<Output = U>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

fn main() {
    let p1 = Point::new(1, 2);
    println!("{p1:?}");

    let p2 = Point::new(1, 2);
    println!("{p2}");

    let p3 = p1 + p2;
    println!("{p3}")
}
