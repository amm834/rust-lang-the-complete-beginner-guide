use crate::generics::Colors::Green;
pub fn run() {
    let person1: Person<&str > = Person {
        name: "Aung Myat Moe",
        company: "Google"
    };
    let person2: Person<&str > = Person {
        name: "Aung Myat Moe 2",
        company: "FB"
    };
    println!("{:?}", person1);
    println!("{:?}", person2);
    let color = Green("#f80");
    println!("{:?}",color);
    
    let points:Point<i32,f64> = Point {x:10,y:32.6};
    println!("{:?}",points);
    
}

#[derive(Debug)]
struct Person < T > {
    name: T,
    company: T
}

#[derive(Debug)]
enum Colors<T>{
    Green(T),
    Blue(T)
}

#[derive(Debug)]
struct Point<T,V>{
    x:T,
    y:V
}