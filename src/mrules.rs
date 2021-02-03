/*macro_rules! echo {
    ($echo: expr) => {
        println!("{}", $echo);
    };
}*/
macro_rules! echo{
    ($($echo:expr),*) => ($(println!("{}",$echo);)*)
}

macro_rules! axis{
    (x=>$e:expr)=>(println!("X is {}",$e));
    (y=>$e:expr)=>(println!("Y is {}",$e));
}

macro_rules! create_function{
    ($func_name:ident) => {
        fn $func_name(){
            println!("{:?} is called!",stringify!($func_name));
        }
    }
}

pub fn run() {
    echo!("Hello World",4*2);
    axis!(x=>10);
    axis!(y=>20);
    create_function!(hey);
    hey();
}
