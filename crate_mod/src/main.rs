extern crate crate_lib;

use crate_lib::mods::User;

fn main() {
    let user = User::new(String::from("xiao_ming"), 12);
    let name = &user.name;
    println!("{}", name);

}
