pub fn run(){
    let a = | x:i32 | x +1;
    println!("{}",a(17));
    let b = | x | println!("{}",x);
    b("Hello!");
    let sum = |x,y| ->i32{
        let c = x + 1;
        let d = y + 1;
        c + d
    };
    println!("Total Value is {}",sum(10,20));
}