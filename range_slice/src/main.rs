fn main() {
    for i in 0.. {
        print!(" {i}");
        if i == 3 { break; }
    }
    println!();

    let a = [1, 2, 3, 4, 5, 6];
    let sa = &a[2..5];
    println!("{sa:?}");
    
    let v = vec![1, 2, 3, 4, 5, 6];
    let sv = &v[2..5];
    println!("{sv:?}");

    let s = "Hola".to_string();
    let sub = &s[2..];
    println!("{sub}");
    
}
