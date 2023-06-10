static mut GLOBAL: &i32 = &0;
static ALBUM: i32 = 2112;

fn guardar(n: &'static i32) {
    unsafe { GLOBAL = n; }
}

fn en_ambos<'a>(a: &'a [i32], b: &'a [i32]) -> &'a i32 {
    for i in 0..a.len() {
        if a[i] == b[i] { return &a[i]; }
    }
    &a[0]
}

fn main() {
    let a = [1, 2, 3];
    let b = [5, 4, 3];
    let r = en_ambos(&a, &b);
    println!("{r}");

    guardar(&ALBUM);
}
