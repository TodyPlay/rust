fn main() {
    let str1 = String::from("hello");
    let str2 = "hello".to_string();

    println!("{}", str1 == str2);

    let str1 = String::from("str1");
    let str2 = String::from("str2");

    let str3 = str1 + &str2;
    println!("{}",  str3);
}
