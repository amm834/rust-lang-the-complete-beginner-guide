pub fn run() {
    let square = |x: i32| x * x;
    apply(square, 2);

    // sum under 500
    // sum only even nums
    let limit = 500;
    let mut sum = 0;
    for i in 0.. {
        let isq = i * i;
        if isq > limit {
            break;
        } else {
            if is_even(isq) {
                sum += isq;
            }
        }
    }
    println!("The sum by traditional way is {}", sum);
    fn is_even(x: i32) -> bool {
        x % 2 == 0
    }
    let sum2 = (0..)
        .map(|x| x * x)
        .take_while(|x| x <= &limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);
    println!("Using HOFs is {}", sum2);
}

fn apply(f: fn(i32) -> i32, a: i32) {
    println!("Result {}", f(a));
}
