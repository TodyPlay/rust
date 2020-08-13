use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess a number");

    let rand_num = rand::thread_rng().gen_range(1, 101);

    // println!("this random num is :{}", rand_num);

    loop {
        println!("input your number");
        let mut num_str = String::new();
        io::stdin().read_line(&mut num_str).expect("读取失败,请重启程序");

        println!("your num : {}", num_str);

        let num: u32;

        let result = num_str.trim().parse();

        match result {
            Ok(ok) => num = ok,
            Err(err) => {
                println!("输入的不是数字请重试 :{}", err.to_string());
                continue;
            }
        }

        let order = num.cmp(&rand_num);

        match order {
            Ordering::Less => println!("your num less"),
            Ordering::Greater => println!("your num big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
