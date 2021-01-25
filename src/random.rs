use rand::{thread_rng,Rng};

pub fn run(){
  let mut rng = thread_rng();
  let a:u32 = rng.gen();
  println!("{}",a);
  let b:u32 = rng.gen_range(1..100);
  println!("{}",b);
}