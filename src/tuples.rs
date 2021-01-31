pub fn run() {
    let mut person = ("Aung Myat Moe", "NoSi", "Daw Than Than Swe", "student"); // at most 12 data access
    println!("{:?}", person);

    person.0 = "ABa";
    println!("Name => {}", person.0);

    let (name, pet, mother, job) = person;
    println!(
        "Name : {},Pet Name: {},Mom : {} ,Job : {}",
        name, pet, mother, job
    );
}
