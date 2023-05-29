fn main() {
    println!("{}", f32::MIN);
    println!("{}", f32::MAX);
    println!("{}", std::f32::consts::PI);
    println!("{}", std::f32::consts::E);

    let x = (-4_i32).abs();
    println!("{}", x);

    #[allow(arithmetic_overflow)]
    let y = 255_u8.overflowing_add(1);
    println!("{:?}", y);
}
