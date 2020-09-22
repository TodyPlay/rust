use std::any::Any;

fn main() {
    let user = User {
        name: String::from("xm"),
        age: 12,
    };

    check(&user);
    type_id(&user);
}

struct User {
    name: String,
    age: u32,
}

fn check(any: &dyn Any) {
    let result = any.is::<User>();
    println!("{}", result);
}

fn type_id(any: &dyn Any) {
    let type_id = any.type_id();
    println!("{:#?}", type_id)
}
