fn info(v: &Vec<String>) {
    for item in v {
        println!("- {item}")
    }
}

fn en_orden(v: &mut Vec<String>) {
    v.sort();
    info(v);
}

fn main() {
    let mut bandas = vec![
        "Rush".to_string(),
        "Leed Zeppelin".to_string(),
        "The Rolling Stones".to_string(),
    ];
    en_orden(&mut bandas);
    println!("{bandas:?}");

    let mut x = 42;
    let rm = &mut x;
    *rm += 1;

    println!("{rm}");

    let r1 = &x;
    let r2 = &x;
    let r3 = &x;

    println!("{x} {r1} {r2} {r3}");
}
