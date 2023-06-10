fn main() {
    let mut s = "Hola".to_string();
    print!("{s}");
    s = ", mundo".to_string();
    println!("{s}");

    fn exclama(mut t: String) -> String {
        t.push_str("!!!");
        t
    }
    s = exclama(s);
    println!("{s}");

    fn foo(s: String) -> String { s }
    fn bar(s: String) -> String { s }

    if s.len() > 5 {
        foo(s);
    } else {
        bar(s);
    }

    s = "Hola".to_string();
    let mut i = 0;
    while i < 3 {
        s = foo(s);
        i+=1;
    }

    let v = vec![
        "uno".to_string(),
        "dos".to_string(),
        "tres".to_string(),
    ];
    for element in v {
        println!("{element}")
    }

    let mut v = vec![
        "uno".to_string(),
        "cinco".to_string(),
        "dos".to_string(),
        "tres".to_string(),
        "cuatro".to_string(),
    ];
    
    let cinco = v.remove(1);
    println!("cinco: {cinco:?}");

    let dos = v.swap_remove(1);
    println!("dos: {dos:?}");

    let cuatro = v.pop();
    println!("cuatro: {cuatro:?}");

    println!("{v:?}")


}
