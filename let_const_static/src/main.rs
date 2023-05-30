fn main() {
    let x: i32 = 42;
    let mut y = 42;
    y += 1;

    const Z: i32 = 42;
    static A: i32 = 42;
    static mut B: i32 = 42;
    unsafe { B += 1 }

    println!("{x}, {y}, {Z}, {A}");
}
