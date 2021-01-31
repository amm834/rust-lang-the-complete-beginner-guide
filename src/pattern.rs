pub fn run() {
    for i in 1..10 {
        println!("{} is {}.", i, amount(i));
    }
    // let point = (0,0);
    // let point = (0,3);
    let point = (3, 3);
    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("({},0)) is x axis", x),
        (0, y) => println!("(0,{})) is y axis", y),
        (x, y) => println!("({},{})", x, y),
    }
}

fn amount(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1..=2 => "a little",
        3..=7 => "some",
        _ if amount % 2 == 0 => "even number",
        _ => "a lot",
    };
}
