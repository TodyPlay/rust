fn main() {
    let s1 = "hello";
    let s2 = "world";

    let str = lifetime(s1, s2);

    println!("{}", str);

    let str = lifetime2();

    println!("{}", str)
}

fn lifetime<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn lifetime2() -> &'static str {
    "123"
}