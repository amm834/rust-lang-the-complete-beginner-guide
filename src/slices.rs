pub fn run() {
    let nums = [1, 2, 3, 4, 5];
    let slices = &nums[1..4];
    println!("{:?}", slices);
    let mut colors = ["red", "green", "blue", "deep purple"];
    update_colors(&mut colors[1..3]);
    println!("{:?}", colors);
}

pub fn update_colors(colors: &mut [&str]) {
    colors[0] = "brown";
    colors[1] = "pink";
}
