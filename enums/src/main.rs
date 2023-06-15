#![allow(dead_code)]

#[derive(Debug, PartialEq, PartialOrd)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404
}

impl HttpStatus {
    fn from_u32(code: u32) -> Self {
        match code {
            200 => Self::Ok,
            304 => Self::NotModified,
            404 => Self::NotFound,
            _ => panic!("Invalid HttpStatus")
        }
    }
}

#[derive(Debug)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
}

#[derive(Debug)]
enum Time {
    Now,
    Past(TimeUnit, u32),
    Future{
        unit: TimeUnit,
        value: u32
    }
}

enum Option<T> {
    None,
    Some(T)
}

fn main() {
    let event = Time::Now;
    let event_2 = Time::Future { unit: TimeUnit::Hours, value: 8 };
    println!("{:?} {:?}", event, event_2);
}
