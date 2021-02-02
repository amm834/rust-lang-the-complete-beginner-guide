pub fn run() {
    get_square(3000);
    get_cube(4038);
}

fn get_square(limit: i32) {
    let mut i = 0;
    while i * i < limit {
        println!("{0} * {0} = {1}", i, i * i);
        i += 1;
    }
}

fn get_cube(limit: i32) {
    let mut i = 1;
    loop {
        println!("{0} * {0} * {0} = {1}", i, i * i * i);
        i += 3;
        if i * i * i > limit {
            break;
        };
    }
}
