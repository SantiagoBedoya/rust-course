#[derive(Debug)]
pub struct StructA {
    pub x: u8,
    pub y: u8,
}

pub fn info_a() {
    let s = StructA { x: 1, y: 2 };
    println!("x: {}, y: {}", s.x, s.y)
}

pub mod b;
pub use b::info_b;