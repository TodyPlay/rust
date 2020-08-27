use std::collections::HashMap;

fn main() {
    //声明一个vector
    let mut vec: Vec<i32> = Vec::new();
    //给vector放值
    vec.push(23);

    //声明一个vector，并初始化容器
    let mut vec: Vec<String> = Vec::with_capacity(12);

    vec.push("hello".to_string());

    let mut hash_map: HashMap<String, usize> = HashMap::new();

    hash_map.insert("hello".to_string(), "hello".len());

    hash_map.remove("hello2");

    println!("{:#?}", hash_map);
}
