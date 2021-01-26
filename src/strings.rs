pub fn run() {
    let cat: &str = "Noosi";
    println!("{}", cat);

    let dog: &'static str = "Bo Ni";
    println!("{}", dog);

    let cat = String::new();
    let mut cat = String::from("Noosi");
    println!("{}", cat);

    let cat_owner = format!("{} is the owner of {} .", "Aung Myat Moe", cat);
    println!("{}", cat_owner);
    println!("Length of dog is {}", dog.len());
    cat.push(' ');
    cat.push_str("Walala");
    println!("{}", cat);
}
