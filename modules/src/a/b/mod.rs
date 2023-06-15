use crate::a::StructA;

#[derive(Debug)]
pub struct StructB {
    pub x: u8,
    pub y: u8,
}

pub fn info_b() {
    let sa = StructA { x: 1, y: 2 };
    println!("x: {}, y: {}", sa.x, sa.y);

    let sb = StructA { x: 1, y: 2 };
    println!("x: {}, y: {}", sb.x, sb.y);
}
