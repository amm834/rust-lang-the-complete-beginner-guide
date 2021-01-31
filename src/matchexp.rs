use crate::matchexp::Colors::{Blue, Green, Red};

pub fn run() {
    check_color(Red);
    check_color(Green);
    check_color(Blue);
    country(95);
    country(45);
    country(-1);
}

fn country(code: i32) {
    let country = match code {
        95 => "Myanmar",
        45 => "Spain",
        1..=999 => "Unknown",
        _ => "Invalid",
    };
    println!("Country name is {}", country);
}

enum Colors {
    Red,
    Green,
    Blue,
}

fn check_color(color: Colors) {
    match color {
        Red => println!("♥️"),
        Green => println!("💚"),
        Blue => println!("💙"),
    }
}
