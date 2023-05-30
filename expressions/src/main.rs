fn main() {
    let mut x = 1 + 2 - 3 * 4 / 5 % 6;
    x += 1;
    assert_eq!(x, 2);

    let x = 0;
    assert!(x == 0);
    assert!(x != 1);
    assert!(x < 1);
    assert!(x <= 1);
    assert!(x >= 0);

    let mut x = 1u8 << 3;
    assert_eq!(x, 8);
    x >>= 1;

    assert_eq!(x, 4);

    {
        let x = 1;
        assert_eq!(x, 1);
    }

    let x = {
        let a = 1;
        let b = 2;
        a + b
    };

    assert_eq!(x, 3);

    let x = 0;
    if x == 0 {
        assert!(true);
    } else if x == 1 {
        assert!(false);
    } else {
        assert!(false)
    }

    let y = true;
    let x = if y { 0 } else { 1 };
    assert_eq!(x, 0);

    match x {
        0 => assert!(true),
        1 => assert!(false),
        _ => assert!(false),
    }

    // match
    let x = match y {
        true => {
            let a = 1;
            let b = 2;
            a + b
        }
        false => 0
    };
    assert_eq!(x, 3);

    // while
    let mut i = 0;
    while i < 10 {
        i += 1;
        println!("{i}");
    }
    

    // loop
    i = 0;
    loop { 
        i += 1;
        if i > 10 {
            break;
        }
    }

    let array = [1, 2, 3];
    for item in array {
        println!("item: {item}");
        if item == 2 {
            break;
        }
    }

    let array = [4, 5, 6];
    let x = for item in array {
        if item == 5 {
            continue;
        }
        println!("item: {item}");
    };

    assert_eq!(x, ());

    // etiquetas
    'infinito: loop {
        let array = [1, 2, 3];
        for item in array {
            if item == 2 {
                break 'infinito;
            }
            println!("item: {item}");
        }
    }

    // return
    if 0 == 42 {
        return ();
    } else {
        assert!(true)
    }


}
