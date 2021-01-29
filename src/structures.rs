pub fn run() {
    let person1 = Person {
        name: String::from("Aung Myat Moe"),
        age: 17,
        company: String::from("Google")
    };
    println!("{:?}", person1);
    println!("Name : {}", person1.name);
    // get user detail
    println!("{}", person1.get_user_detail());
    // static function
    println!("{}", Person::static_fn());
}

#[derive(Debug)]

struct Person {
    name: String,
    age: i32,
    company: String
}

impl Person {
    fn get_user_detail(&self) -> String {
        format!("name => {},age => {},company => {}", &self.name, &self.age, &self.company)
    }
    fn static_fn()->String {
        String::from("I am static function.")
    }
}