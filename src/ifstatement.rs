use rand::{thread_rng, Rng};

pub fn run() {
    let mut rng = thread_rng();
    let num = rng.gen_range(0..13);
    if num > 5 {
        println!("{} is greater than 5.", num);
    } else if num == 5 {
        println!("{} is equal to 5.", num);
    } else {
        println!("{} is less than 5.", num);
    }
}
