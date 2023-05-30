fn main() {
    let varios_tipos = (42, 3.14f32, true, 'a', 2.12);
    println!("{varios_tipos:?}");
    let _origen = (0.0, 0.0);
    let _unitario = (
        (0.0, 1.0),
        (1.0, 0.0),
        (0.0, -1.0),
        (-1.0, 0.0),
    );
    
    let _uno_solo = (1.0,);
    let _unit = ();

    // array
    let tres_ceros = [0, 0, 0];
    println!("{tres_ceros:?}");

    let _tres_mas: [u8; 3] = [0,0,0];

    let _otra_vez = [0u8, 0u8, 0u8];
    let _un_kibibyte = [0u8; 1024];

    let tres = [1, 2, 3];
    println!("segundo: {}", tres[1]);
    println!("elementos: {}", tres.len());
    println!("primero: {}, ultimo: {}", tres[0], tres[tres.len() - 1]);

    let cuarto = tres[3];
    println!("{cuarto}");

}
