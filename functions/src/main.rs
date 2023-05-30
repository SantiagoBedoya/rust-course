type Op = fn(i32, i32) -> i32;
type Triple = (i32, i32, i32);

fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

fn resta(a: i32, b: i32) -> i32 {
    a - b
}

fn calcula(a: i32, b: i32, op: Op) -> Triple {
    (a, b, op(a, b))
}

fn main() {
    let mut result = calcula(1, 2, sumar);
    println!("{} + {} = {}", result.0, result.1, result.2);

    result = calcula(1, 2, resta);
    println!("{} - {} = {}", result.0, result.1, result.2);
}
