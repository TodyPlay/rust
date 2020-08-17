fn main() {
    let num = 3;
    if num > 3 {
        println!("num > 3");
    } else {
        println!("num <= 3")
    }

    let bool = true;
    let num = if bool {
        5
    } else {
        6
    };
    println!("let if num is :{}", num)
}
