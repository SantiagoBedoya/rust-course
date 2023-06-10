#![allow(dead_code)]

trait Imprime {
    fn imprime(&self) -> String;
}

trait Resalta: Imprime {
    fn resalta(&self) -> String {
        format!("*** {} ***", self.imprime())
    }
}

#[derive(PartialEq, PartialOrd)]
struct Foo;

impl std::fmt::Display for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo")
    }
}

impl Imprime for Foo {
    fn imprime(&self) -> String {
        format!("Foo")
    }
}

impl Resalta for Foo {
    fn resalta(&self) -> String {
        format!("____ {} ____", self.imprime())
    }
}

fn main() {
    let f = Foo;
    println!("{}", f.imprime());
    println!("{}", f.resalta());
    println!("{f}");
    println!("{}", f == Foo);
    println!("{}", f > Foo);
    println!("{}", f < Foo);
}
