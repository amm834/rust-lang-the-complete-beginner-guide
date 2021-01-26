fn say_hi(name: &mut &str) -> String {
    return format!("Hello {}", name);
}

pub fn run() {
    let mut name = "Aung Myat Moe";
    let greet = say_hi(&mut name);
    println!("{}", greet);
}
