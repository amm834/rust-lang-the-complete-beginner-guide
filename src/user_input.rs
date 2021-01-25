use std::io;

pub fn run() {
    let mut input = String::new();
    println!("Input Something!");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Your input is {}", input);
        }
        Err(e) => {
            println!("Something went wrong {}", e)
        }
    }
}
