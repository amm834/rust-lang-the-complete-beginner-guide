struct PHPDev {
    awesome: bool,
}

struct RustDev {
    awesome: bool,
}

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {
        println!("Hello World");
    }
}

impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        RustDev { awesome }
    }
    fn language(&self) -> &str {
        "Rust"
    }
    fn say_hello(&self) {
        println!(
            "println!(\"Hello World\");
            "
        );
    }
}

impl Developer for PHPDev {
    fn new(awesome: bool) -> Self {
        PHPDev { awesome }
    }
    fn language(&self) -> &str {
        "PHP"
    }
    fn say_hello(&self) {
        println!("echo\"Hello World\";");
    }
}

pub fn run() {
    let p = PHPDev::new(true);
    println!("{}", p.language());
    p.say_hello();

    let r = RustDev::new(true);
    println!("{}", r.awesome);
    println!("{}", r.language());
    r.say_hello();
}
