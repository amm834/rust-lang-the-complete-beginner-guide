// global scope
static mut R: i32 = 0;
pub fn run() {
    {
        // local scope
        let a = 5;
        println!("a={}", a);
    }
    // can't access local scope
    unsafe {
        // mutable static is unsafe
        R = 3;
        println!("{}", R);
    }
}
