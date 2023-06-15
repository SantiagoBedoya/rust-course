use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum NumError {
    Zero,
    Three,
}

impl fmt::Display for NumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => write!(f, "Cannot be zero"),
            Self::Three => write!(f, "Cannot be three"),
        }
    }
}

impl Error for NumError {}

type GenericError = Box<dyn Error + Send  + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;


fn to_num(s: &str) -> GenericResult<i32> {
    let n: i32 = s.parse()?;
    if n == 0 {
        return Err(Box::new(NumError::Zero));
    } else if n == 3 {
        return Err(Box::new(NumError::Three));
    }
    Ok(n)
}

fn main() {
    let n = to_num("3").unwrap();
    println!("{n}");
}
