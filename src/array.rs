pub fn run() {
    // array must have same type
    let primes: [i32; 5] = [2, 4, 5, 6, 8];
    let mut doubles: [f64; 5] = [1.0, 2.0, 3.0, 4.0, 5.0];
    // print array with debug formatter
    println!("{:?}", primes);
    println!("{:?}", doubles);
    println!("Index No:3 of doubles => {}", doubles[3]);

    // access array chunk via index
    doubles[4] = 6.0;
    println!("Reassign via index => {:?}", doubles);

    const DEFAULT: i32 = 3;
    let numbers = [DEFAULT; 10];
    println!("{:?}", numbers);

    // iteration
    for number in numbers.iter() {
        println!("{}", number);
    }
}
