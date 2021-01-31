use crate::enums::Colors::Green;
use crate::enums::Person::Name;
pub fn run() {
    println!("{:?}", Colors::Red);
    println!("{:?}", Green);
    println!("{:?}", Name(String::from("Aung Myat Moe")));
}

#[derive(Debug)]
enum Colors {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
enum Person {
    Name(String),
    Age(u64),
    Company(String),
}
