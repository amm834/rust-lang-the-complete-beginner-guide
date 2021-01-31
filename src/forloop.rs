pub fn run() {
    for i in 1..10 {
        println!("{0} * {0} = {1}", i, (i * i));
    }
    let pet = ["cat", "dog", "pig", "chicken", "bear"];
    for pet in pet.iter() {
        if pet == &"dog" {
            println!("{} is barks too much.", pet);
            continue;
        }
        if pet == &"bear" {
            println!("{} is not a pet.", pet);
            break;
        }
        println!("I love {}", pet);
    }
    for (pos, i) in (1..10).enumerate() {
        println!("{} * {} = {}", i, pos + 1, (i * (pos + 1)));
    }
}
