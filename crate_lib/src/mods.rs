pub struct User {
    pub name: String,
    age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> User {
        User {
            name,
            age,
        }
    }
}

pub fn print_hello() {
    println!("hello word");
}