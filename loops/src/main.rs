fn main() {
    // loop {
    //     println!("loop print");
    // }

    let mut counter = 0;
    let mut result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("loop result is: {}", result);

    //while 循环
    while result != 0 {
        result -= 1;
    };

    println!("while result is: {}", result);

    //while 遍历
    let mut vec = [1, 2, 3, 4, 5, 6];
    let mut index = 0;
    while index < vec.len() {
        println!("vec value is : {}", vec[index]);
        index += 1;
    }

    //for 遍历
    for x in &vec {
        println!("for loop value is :{}", x);
    }

    //vec 反转
    vec.reverse();
    for x in vec.iter() {
        println!("iter value is : {}", x);
    }

    for x in (1..5).rev() {
        println!("元组反转遍历 ：{}", x);
    }
}
