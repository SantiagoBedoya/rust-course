#[allow(dead_code)]
fn print_type<T>(_: T) {
    println!("type {}", std::any::type_name::<T>())
}

fn main() {
    let _ = true;
    let _ = false;

    println!("(1 < 5) = {}", 1 < 5);
    println!("(1 > 5) = {}", 1 > 5);
    println!("(true && false) = {}", true && false);
    println!("(true || false) = {}", true || false);

    println!("(true as u8) = {}", true as u8);
    println!("(false as u8) = {}", false as u8);

    // char
    let a = 'a';
    let b = '\x61';
    let c = '💪';
    let e = '\u{1f980}';
    println!("{a}, {b}, {c}, {e}");
    
    let f = 0x61_u8 as char;
    println!("{f}");

    let g = char::from_u32(0x61);
    println!("{g:?}");

    println!("is_digit? {}", a.is_digit(10));
    println!("is_alphabetic? {}", a.is_alphabetic());

    let i = b'a';
    println!("{i}");
    println!("{}", a as char);
    println!("(i == 0x61) = {}", i == 0x61);
    println!("(i == 97) = {}", i == 97);
    print_type(i);

}
