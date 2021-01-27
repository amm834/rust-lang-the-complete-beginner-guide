pub fn run(){
    let primes: Vec<i32> = Vec::new();
    let mut primes = vec![2,4,6,8,10];
    // print all data
    println!("{:?}",primes);
    // print via indecies
    println!("Index no3 => {}",primes[3]);
    
    // change data via Index
    primes[3] = 16;
    println!("{:?}",primes);
    
    let values = vec![2;10];
    println!("{:?}",values);
    
    const DEFAULT:i32 = 17;
    let numbers = [DEFAULT;10];
    println!("{:?}",numbers);
    
    // iteration
    for number in numbers.iter(){
        println!("{}",number);
    }
}